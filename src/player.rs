use crate::resources::*;
use crate::scene::*;

pub struct Player {
    rect: Rectangle,
    velocity: Vector2,
    jumps: u8,

    textures: Animation,
    camera: Camera2D,
    frame: f32,
    flip: i8,

    shoot_sound: Sound,
    jump_sound: Sound,
}

impl Player {
    pub(crate) fn new(rl: &mut RaylibHandle, thread: &RaylibThread, pos: Vector2) -> Self {
        let textures = Animation::load(rl, thread, "Assets/Player");
        let size = textures.size();
        Self {
            rect: rrect(pos.x, pos.y, size.x, size.y),
            velocity: Vector2::zero(),
            jumps: 0,

            textures,
            camera: Camera2D {
                target: pos + size / 2.0,
                offset: rvec2(rl.get_screen_width(), rl.get_screen_height()) / 2.0,
                rotation: 0.0,
                zoom: 1.0,
            },
            frame: 0.0,
            flip: 1,

            shoot_sound: Sound::load_sound("Assets/Shoot.wav").expect("Failed to load sound!"),
            jump_sound: Sound::load_sound("Assets/Jump.wav").expect("Failed to load sound!"),
        }
    }

    fn collides(&self, scene: &Scene) -> bool {
        if let Some(mut rect) = scene.tower().shield_rect() {
            rect.y = 0.0;
            rect.height = scene.height() as _;
            if self.rect.check_collision_recs(&rect) {
                return true;
            }
        }
        for x in self.rect.x.floor() as i32..(self.rect.x + self.rect.width).ceil() as i32 {
            for y in self.rect.y.floor() as i32..(self.rect.y + self.rect.height).ceil() as i32 {
                if scene.metadata(rvec2(x, y)) == Color::BLACK {
                    return true;
                }
            }
        }
        false
    }

    fn resolve_collision(&mut self, scene: &Scene, scale: f32, dir: Vector2, undo: bool) -> f32 {
        let mut offset = 0.0;
        while self.collides(scene) {
            if !rrect(0, 0, scene.width(), scene.height()).check_collision_recs(&self.rect) {
                self.rect.x -= dir.x * offset;
                self.rect.y -= dir.y * offset;
                return f32::INFINITY;
            }

            offset += 0.5 / scale;
            self.rect.x += dir.x * 0.5 / scale;
            self.rect.y += dir.y * 0.5 / scale;
        }
        if undo {
            self.rect.x -= dir.x * offset;
            self.rect.y -= dir.y * offset;
        }
        offset
    }

    pub(crate) fn update(&mut self, rl: &RaylibHandle, audio: &mut RaylibAudio, scene: &mut Scene) {
        let speed = 100.0;
        let rate = 0.1;
        let gravity = 400.0;
        let jump = -200.0;
        let cut = 0.5;
        let scale = rl.get_screen_height() as f32 / scene.height() as f32;
        let jumps = 2;
        let bullet_speed = 1.0;

        // * Shoot
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let gun = self.position() + rvec2(if self.flip < 0 { 4 } else { 6 }, 11);
            let aim = rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera) - gun;

            scene.bullets.push(Bullet::new(
                gun,
                aim.normalized() * (aim.length() * 0.5 + 80.0) * bullet_speed,
            ));
            audio.play_sound(&self.shoot_sound);
        }

        // * Movement
        self.velocity.x = lerp(
            self.velocity.x,
            (rl.is_key_down(KeyboardKey::KEY_D) as i32 - rl.is_key_down(KeyboardKey::KEY_A) as i32)
                as f32
                * speed,
            rl.get_frame_time() / rate,
        );

        self.velocity.y += gravity * rl.get_frame_time();
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.jumps > 0 {
            self.velocity.y = jump;
            self.jumps -= 1;
            audio.play_sound(&self.jump_sound);
        }
        if rl.is_key_released(KeyboardKey::KEY_SPACE) && self.velocity.y < 0.0 {
            self.velocity.y *= cut;
        }

        let motion = self.velocity * rl.get_frame_time();
        self.rect.x += motion.x;
        if self.collides(scene) {
            let step = self.resolve_collision(scene, scale, rvec2(0, -1), true);
            let step_ratio = motion.x.abs().ceil() / step;

            if step_ratio >= 1.0 {
                self.rect.y -= step;
            } else {
                self.velocity.x = 0.0;
                self.resolve_collision(scene, scale, rvec2(-motion.x.signum(), 0), false);
            }
        }

        self.rect.y += motion.y;
        if self.collides(scene) {
            if self.velocity.y > 0.0 {
                self.jumps = jumps;
            }
            self.velocity.y = 0.0;
            self.resolve_collision(scene, scale, rvec2(0, -motion.y.signum()), false);
        }

        // * Camera
        self.camera.zoom = scale;

        self.camera.offset = rvec2(rl.get_screen_width(), rl.get_screen_height()) / 2.0;
        self.camera.target = self.center();

        self.camera.target.x = self.camera.target.x.clamp(
            self.camera.offset.x / scale,
            scene.width() as f32 - self.camera.offset.x / scale,
        );
        self.camera.target.y = self.camera.target.y.clamp(
            self.camera.offset.y / scale,
            scene.height() as f32 - self.camera.offset.y / scale,
        );

        // * Animation
        if self.velocity.x.abs() > 0.5 {
            self.frame = (self.frame + rl.get_frame_time() * 5.0) % 2.0;
        }

        if rl.get_mouse_x() > rl.get_world_to_screen2D(self.position(), self.camera).x as i32 {
            self.flip = 1;
        } else {
            self.flip = -1;
        }
    }

    pub(crate) fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_texture_rec(
            &self.textures[self.frame as usize],
            rrect(0, 0, self.rect.width * self.flip as f32, self.rect.height),
            self.position(),
            Color::WHITE,
        );
    }

    pub fn position(&self) -> Vector2 {
        rvec2(self.rect.x, self.rect.y)
    }

    pub fn size(&self) -> Vector2 {
        rvec2(self.rect.width, self.rect.height)
    }

    pub fn center(&self) -> Vector2 {
        self.position() + self.size() / 2.0
    }

    pub fn rect(&self) -> Rectangle {
        self.rect
    }

    pub fn camera(&self) -> Camera2D {
        self.camera
    }
}
