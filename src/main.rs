pub mod player;
pub mod resources;
pub mod scene;
pub mod tower;

use player::*;
use resources::*;
use scene::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(860, 480)
        .resizable()
        .title("Personality")
        .build();

    let mut audio = RaylibAudio::init_audio_device();

    let bullet_texture = rl
        .load_texture(&thread, "Assets/Bullet.png")
        .expect("Failed to load bullet texture.");

    let game_over_sound =
        Sound::load_sound("Assets/GameOver.wav").expect("Failed to load game over sound.");

    loop {
        let mut player = Player::new(&mut rl, &thread, rvec2(10, 40));
        let mut scene = Scene::new(&mut rl, &thread, 1);

        let mut bullet_timer = 0.0;
        let mut roles_reversed_timer = get_random_value::<i32>(10, 20) as f32;
        let mut roles_reversed_text_timer = None;
        let mut play_time = 0.0;
        let mut game_over = false;
        while !game_over {
            if rl.window_should_close() {
                return;
            }

            player.update(&rl, &mut audio, &mut scene);
            scene.update(&mut rl, &mut audio, &bullet_texture, &mut game_over);
            play_time += rl.get_frame_time();

            if !scene.tower().reversed() {
                // * Bullet timer
                bullet_timer -= rl.get_frame_time();
                while bullet_timer <= 0.0 {
                    let flip = get_random_value::<i32>(0, 1) != 0;
                    scene.bullets.push(Bullet::new(
                        rvec2(
                            if flip { 0 } else { scene.width() },
                            get_random_value::<i32>(32, scene.height() - 24),
                        ),
                        rvec2(
                            if flip { 1 } else { -1 } * get_random_value::<i32>(10, 200),
                            get_random_value::<i32>(-10, 10),
                        ),
                    ));
                    bullet_timer += get_random_value::<i32>(100, 200) as f32 / 100.0;
                }

                // * Reverse timer
                roles_reversed_timer -= rl.get_frame_time();
                if roles_reversed_timer <= 0.0
                    && !player.rect().check_collision_recs(&{
                        let mut rect = scene.tower().rect();
                        rect.x -= 10.0;
                        rect.width += 20.0;
                        rect.y = 0.0;
                        rect.height = scene.height() as _;
                        rect
                    })
                {
                    scene.reverse_roles(player.center(), &mut audio);
                    roles_reversed_timer = get_random_value::<i32>(10, 20) as f32;
                    roles_reversed_text_timer = Some(1.0);
                }
            }

            if let Some(timer) = roles_reversed_text_timer.as_mut() {
                *timer -= rl.get_frame_time() / 2.0;
                if *timer <= 0.0 {
                    roles_reversed_text_timer = None;
                }
            }

            // * Draw
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::SKYBLUE);
            {
                let mut d = d.begin_mode2D(player.camera());
                scene.draw(&mut d, &bullet_texture);
                player.draw(&mut d);
            }
            d.draw_fps(12, 12);
            if let Some(timer) = roles_reversed_text_timer {
                d.draw_text(
                    "Roles Reversed!",
                    20,
                    20,
                    (timer * 100.0) as _,
                    Color::RAYWHITE,
                );
            }
        }

        audio.play_sound(&game_over_sound);

        while game_over {
            if rl.window_should_close() {
                return;
            }

            if rl.is_key_pressed(KeyboardKey::KEY_P) {
                game_over = false;
            }

            let screen_size = (rl.get_screen_width(), rl.get_screen_height());
            let mut d = rl.begin_drawing(&thread);
            d.draw_text(
                "Game Over",
                (screen_size.0 - measure_text("GameOver", 60)) / 2,
                10,
                60,
                Color::WHITE,
            );
            let score = &format!("You held on for {} seconds", play_time as i32);
            d.draw_text(
                score,
                (screen_size.0 - measure_text(score, 30)) / 2,
                70,
                30,
                Color::WHITE,
            );
            d.draw_text(
                "Press P to restart",
                (screen_size.0 - measure_text("Press P to restart", 20)) / 2,
                110,
                20,
                Color::WHITE,
            );
        }
    }
}
