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

    let bullet_texture = rl
        .load_texture(&thread, "Assets/Bullet.png")
        .expect("Failed to load bullet texture.");

    loop {
        let mut player = Player::new(&mut rl, &thread, rvec2(10, 40));
        let mut scene = Scene::new(&mut rl, &thread, 1);

        let mut bullet_timer = 0.0;
        let mut roles_reversed_timer = rand::thread_rng().gen_range(20.0..50.0);
        let mut roles_reversed_text_timer = None;
        let mut play_time = 0.0;
        let mut game_over = false;
        while !game_over {
            if rl.window_should_close() {
                return;
            }

            player.update(&rl, &mut scene);
            scene.update(&mut rl, player.center(), &bullet_texture, &mut game_over);
            play_time += rl.get_frame_time();

            if !scene.tower().reversed() {
                // * Bullet timer
                bullet_timer -= rl.get_frame_time();
                while bullet_timer <= 0.0 {
                    let flip = rand::thread_rng().gen_bool(0.5);
                    scene.bullets.push(Bullet::new(
                        rvec2(
                            if flip { 0 } else { scene.width() },
                            rand::thread_rng().gen_range(32..scene.height() - 24),
                        ),
                        rvec2(
                            if flip { 1 } else { -1 } * rand::thread_rng().gen_range(10..200),
                            rand::thread_rng().gen_range(-10..10),
                        ),
                    ));
                    bullet_timer += rand::thread_rng().gen_range(1.0..2.0);
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
                    scene.reverse_roles(player.center());
                    roles_reversed_timer = rand::thread_rng().gen_range(20.0..50.0);
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
