use super::BOARD_HEIGHT;
use colosseum::{Input, Sprite, Texture, Vector2, Vector4, Window};

#[derive(Clone, Copy)]
pub enum TileColor {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
}

pub struct Tile {
    sprite: Sprite,
}

impl Tile {
    pub fn new<I: Input>(
        color: TileColor,
        position: (isize, isize),
        texture: Texture,
        window: &mut Window<I>,
    ) -> Self {
        let mut sprite = Sprite::new(window, Some(texture));
        sprite.set_tint(color.into());
        sprite.transform_mut().set_position(Vector2::new(
            position.0 as f32,
            BOARD_HEIGHT as f32 - position.1 as f32,
        ));

        Tile { sprite }
    }

    pub fn set_position(&mut self, position: (isize, isize)) {
        self.sprite.transform_mut().set_position(Vector2::new(
            position.0 as f32,
            BOARD_HEIGHT as f32 - position.1 as f32,
        ));
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.sprite.render(window)
    }
}

impl Into<Vector4> for TileColor {
    fn into(self) -> Vector4 {
        match self {
            TileColor::Red => Vector4::new(1.0, 0.0, 0.0, 1.0),
            TileColor::Orange => Vector4::new(1.0, 0.5, 0.0, 1.0),
            TileColor::Yellow => Vector4::new(1.0, 1.0, 0.0, 1.0),
            TileColor::Green => Vector4::new(0.0, 1.0, 0.0, 1.0),
            TileColor::Cyan => Vector4::new(0.0, 1.0, 1.0, 1.0),
            TileColor::Blue => Vector4::new(0.0, 0.0, 1.0, 1.0),
            TileColor::Purple => Vector4::new(0.5, 0.0, 0.5, 1.0),
        }
    }
}
