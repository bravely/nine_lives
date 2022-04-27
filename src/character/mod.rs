use anyhow::Error;
use crankstart::{
    graphics::{Graphics, LCDBitmapFlip},
    sprite::{Sprite, SpriteManager},
};

const INITIAL_X: f32 = (400.0 - 16.0) / 2.0;
const INITIAL_Y: f32 = (240.0 - 32.0) / 2.0;

pub struct Character {
    sprite: Sprite,
    // image: Bitmap,
    // image_data: BitmapData,
}

impl Character {
    pub fn new() -> Result<Self, Error> {
        let sprite_manager = SpriteManager::get_mut();
        let graphics = Graphics::get();

        let mut sprite = sprite_manager.new_sprite()?;
        let image = graphics.load_bitmap("images/chrono_soldier1")?;
        // let image_data = image.get_data()?;
        sprite.set_image(image.clone(), LCDBitmapFlip::kBitmapUnflipped)?;
        sprite.move_to(INITIAL_X, INITIAL_Y)?;

        // sprite_manager.add_sprite(&sprite)?;

        Ok(Self {
            sprite,
            // image,
            // image_data,
        })
    }

    pub fn ensure(&mut self) -> Result<(), Error> {
        let sprite_manager = SpriteManager::get_mut();
        sprite_manager.add_sprite(&self.sprite)?;
        self.sprite.set_z_index(1000)?;
        Ok(())
    }
}
