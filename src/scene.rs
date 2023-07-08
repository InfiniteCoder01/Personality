use crate::resources::*;
use crate::tower::*;

pub struct Scene {
    texture: Texture2D,
    metadata: ImageColors,
    tower: Tower,
    pub bullets: Vec<Bullet>,
}

pub struct Bullet {
    position: Vector2,
    velocity: Vector2,
}

impl Bullet {
    pub fn new(position: Vector2, velocity: Vector2) -> Self {
        Self { position, velocity }
    }

    pub(self) fn update(&mut self, rl: &RaylibHandle) {
        self.position += self.velocity * rl.get_frame_time();
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }
}

#[allow(dead_code)]
impl Scene {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, index: usize) -> Self {
        Self {
            texture: rl
                .load_texture(thread, &format!("Assets/Scene{}-1.png", index))
                .expect("Failed to load scene texture."),
            metadata: Image::load_image(&format!("Assets/Scene{}-2.png", index))
                .expect("Failed to load scene metadata.")
                .get_image_data(),
            tower: Tower::new(rl, thread, rvec2(115, 48), index),
            bullets: Vec::new(),
        }
    }

    pub(crate) fn update(
        &mut self,
        rl: &mut RaylibHandle,
        player_center: Vector2,
        bullet_texture: &Texture2D,
        game_over: &mut bool,
    ) {
        self.tower.update(
            rl,
            self.bullets
                .iter()
                .filter(|bullet| {
                    let center = bullet.position()
                        + rvec2(bullet_texture.width(), bullet_texture.height()) / 2.0;
                    (if self.tower.flipped() {
                        center.x < self.tower.position().x
                    } else {
                        center.x > self.tower.position().x + self.tower.size().x
                    }) && center.y > self.tower.position().y
                        && center.y < self.tower.position().y + self.tower.size().y
                })
                .min_by_key(|bullet| bullet.position().x as i32 + bullet_texture.width() / 2)
                .map(|bullet| bullet.position().y + bullet_texture.height() as f32 / 2.0),
            player_center,
            game_over,
        );

        for i in (0..self.bullets.len()).rev() {
            self.bullets[i].update(rl);
            let rect = rrect(
                self.bullets[i].position().x,
                self.bullets[i].position().y,
                bullet_texture.width(),
                bullet_texture.height(),
            );
            if !rrect(0, 0, self.texture.width, self.texture.height).check_collision_recs(&rect) {
                self.bullets.remove(i);
            } else if self.tower.rect().check_collision_recs(&rect) {
                self.bullets.remove(i);
                self.tower.hit();
            } else if let Some(rect2) = self.tower.shield_rect() {
                if rect2.check_collision_recs(&rect) {
                    self.bullets.remove(i);
                    self.tower.hit_shield();
                }
            } else {
                for j in i..self.bullets.len() {
                    if j != i {
                        let rect2 = rrect(
                            self.bullets[j].position().x,
                            self.bullets[j].position().y,
                            bullet_texture.width(),
                            bullet_texture.height(),
                        );
                        if rect.check_collision_recs(&rect2) {
                            self.bullets.remove(i.max(j));
                            self.bullets.remove(i.min(j));
                            break;
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, bullet_texture: &Texture2D) {
        d.draw_texture(&self.texture, 0, 0, Color::WHITE);
        self.tower.draw(d);
        for bullet in &self.bullets {
            d.draw_texture_v(bullet_texture, bullet.position(), Color::WHITE);
        }
    }

    pub fn reverse_roles(&mut self, player_center: Vector2) {
        self.tower
            .reverse_roles(player_center.x < self.tower.position().x + self.tower.size().x / 2.0);
    }

    pub fn width(&self) -> i32 {
        self.texture.width
    }

    pub fn height(&self) -> i32 {
        self.texture.height
    }

    pub fn size(&self) -> Vector2 {
        rvec2(self.texture.width, self.texture.height)
    }

    pub fn metadata(&self, pos: Vector2) -> Color {
        self.metadata[pos.x as usize + pos.y as usize * self.width() as usize]
    }

    pub fn tower(&self) -> &Tower {
        &self.tower
    }
}
