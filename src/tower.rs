use crate::resources::*;

pub enum TowerState {
    Normal,
    Reversed {
        shield: Shield,
        bad_health: f32,
        timer: f32,
    },
}

pub struct Tower {
    position: Vector2,
    textures: Animation,
    shield_textures: Animation,
    hit_sound: Sound,
    roles_reversed_sound: Sound,

    health: f32,
    damaged: f32,
    state: TowerState,
}

impl Tower {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        position: Vector2,
        index: usize,
    ) -> Self {
        let textures = Animation::load(rl, thread, &format!("Assets/Tower{}-", index));
        let shield_textures = Animation::load(rl, thread, &format!("Assets/Shield{}-", index));
        Self {
            position,
            textures,
            shield_textures,
            hit_sound: Sound::load_sound("Assets/Hit.wav").expect("Failed to load hit sound."),
            roles_reversed_sound: Sound::load_sound("Assets/RolesReversed.wav")
                .expect("Failed to load roles reversed sound."),

            health: 1.0,
            damaged: 0.0,
            state: TowerState::Normal,
        }
    }

    pub(crate) fn update(
        &mut self,
        rl: &mut RaylibHandle,
        bullet: Option<Vector2>,
        game_over: &mut bool,
    ) {
        self.damaged = (self.damaged - rl.get_frame_time()).max(0.0);
        if let TowerState::Reversed {
            shield,
            bad_health,
            timer,
        } = &mut self.state
        {
            *timer -= rl.get_frame_time();
            if *timer <= 0.0 {
                *game_over = true;
            }

            shield.update(rl, bullet, &self.shield_textures);
            if *bad_health <= 0.0 {
                self.state = TowerState::Normal;
            }
        } else if self.health <= 0.0 {
            *game_over = true;
        }
    }

    pub(crate) fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        let health_bar_size = rvec2(20, 4);
        let health_bar_pos = self.position
            + rvec2(
                (self.textures.width() - health_bar_size.x as i32) / 2,
                -3 - health_bar_size.y as i32,
            );

        d.draw_texture_v(
            &self.textures[(self.damaged > 0.0) as usize],
            self.position,
            Color::WHITE,
        );

        let mut health_color = Color::RED;
        let mut health = self.health;
        let mut clock = None;
        if let TowerState::Reversed {
            shield,
            bad_health,
            timer,
        } = &self.state
        {
            shield.draw(d, &self.shield_textures);
            health_color = Color::BLUE;
            health = *bad_health;
            clock = Some(timer);
        }

        d.draw_rectangle_v(health_bar_pos, health_bar_size, Color::BLACK);
        d.draw_rectangle_v(health_bar_pos + 1.0, health_bar_size - 2.0, Color::GRAY);
        d.draw_rectangle_v(
            health_bar_pos + 1.0,
            (health_bar_size - 2.0) * rvec2(health, 1),
            health_color,
        );
        if let Some(clock) = clock {
            d.draw_text(
                &format!("{}", *clock as i32),
                health_bar_pos.x as i32,
                health_bar_pos.y as i32 - 10,
                10,
                Color::RED,
            );
        }
    }

    pub fn reverse_roles(&mut self, flip: bool, audio: &mut RaylibAudio) {
        self.state = TowerState::Reversed {
            shield: Shield::new(
                self.position()
                    + rvec2(
                        if flip { -10.0 } else { self.size().x + 3.0 },
                        (self.size().y - self.shield_textures.size().y) / 2.0,
                    ),
                flip,
            ),
            bad_health: 1.0,
            timer: 10.0,
        };
        audio.play_sound(&self.roles_reversed_sound);
    }

    pub fn hit(&mut self, audio: &mut RaylibAudio) {
        if let TowerState::Reversed { bad_health, .. } = &mut self.state {
            *bad_health -= 1.0 / 10.0;
        } else {
            self.health -= 1.0 / 100.0;
        }
        self.damaged = 0.1;
        audio.play_sound(&self.hit_sound);
    }

    pub fn hit_shield(&mut self) {
        if let TowerState::Reversed { shield, .. } = &mut self.state {
            shield.damaged = 0.1;
        }
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn size(&self) -> Vector2 {
        self.textures.size()
    }

    pub fn rect(&self) -> Rectangle {
        rrect(
            self.position.x,
            self.position.y,
            self.textures.width(),
            self.textures.height(),
        )
    }

    pub fn shield_rect(&self) -> Option<Rectangle> {
        if let TowerState::Reversed { shield, .. } = &self.state {
            Some(shield.rect(&self.shield_textures))
        } else {
            None
        }
    }

    pub fn flipped(&self) -> bool {
        if let TowerState::Reversed { shield, .. } = &self.state {
            shield.flipped()
        } else {
            false
        }
    }

    pub fn reversed(&self) -> bool {
        matches!(self.state, TowerState::Reversed { .. })
    }
}

pub struct Shield {
    position: Vector2,
    target: f32,
    flip: bool,
    damaged: f32,
}

impl Shield {
    pub fn new(position: Vector2, flip: bool) -> Self {
        Self {
            position,
            target: position.y,
            flip,
            damaged: 0.0,
        }
    }

    pub(self) fn update(
        &mut self,
        rl: &mut RaylibHandle,
        bullet: Option<Vector2>,
        shield_textures: &Animation,
    ) {
        let mut time = 0.7;

        self.damaged = (self.damaged - rl.get_frame_time()).max(0.0);
        if let Some(bullet) = bullet {
            self.target = bullet.y - shield_textures.height() as f32 / 2.0;
            time -= (self.position.x + shield_textures.size().x / 2.0 - bullet.x).abs() / 200.0;
        }

        self.position.y += (self.target - self.position.y) * rl.get_frame_time() / time;
    }

    pub(self) fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, shield_textures: &Animation) {
        d.draw_texture_rec(
            &shield_textures[(self.damaged > 0.0) as usize],
            rrect(
                0,
                0,
                shield_textures.width() * if self.flip { -1 } else { 1 },
                shield_textures.height(),
            ),
            self.position,
            Color::WHITE,
        );
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn rect(&self, shield_textures: &Animation) -> Rectangle {
        rrect(
            self.position.x,
            self.position.y,
            shield_textures.width(),
            shield_textures.height(),
        )
    }

    pub fn flipped(&self) -> bool {
        self.flip
    }
}
