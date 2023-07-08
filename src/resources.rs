use std::ops::Index;

pub use raylib::prelude::*;

pub struct Animation {
    textures: Vec<Texture2D>,
}

impl Animation {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        let mut textures = Vec::new();
        for i in 1..usize::MAX {
            let filename = &format!("{}{}.png", path, i);
            if !std::path::Path::new(filename).exists() {
                break;
            }
            textures.push(
                rl.load_texture(thread, filename)
                    .expect("Failed to load texture"),
            );
        }

        Self { textures }
    }

    pub fn width(&self) -> i32 {
        self.textures[0].width
    }

    pub fn height(&self) -> i32 {
        self.textures[0].height
    }

    pub fn size(&self) -> Vector2 {
        rvec2(self.textures[0].width, self.textures[0].height)
    }
}

impl Index<usize> for Animation {
    type Output = Texture2D;

    fn index(&self, index: usize) -> &Self::Output {
        &self.textures[index]
    }
}
