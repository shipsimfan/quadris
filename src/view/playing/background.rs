use super::Textures;
use colosseum::{Input, Sprite, Vector2, Window};

pub struct Background {
    left_pane: Sprite,
    right_pane: Sprite,

    left_tiles: Sprite,
    right_tiles: Sprite,
}

impl Background {
    pub fn new<I: Input>(textures: &Textures, window: &mut Window<I>) -> Self {
        let mut left_pane = Sprite::new(window, Some(textures.background_left().clone()));
        let mut right_pane = Sprite::new(window, Some(textures.background_right().clone()));

        left_pane
            .transform_mut()
            .set_position(Vector2::new(-4.5, 10.5));
        right_pane
            .transform_mut()
            .set_position(Vector2::new(13.5, 10.5));

        left_pane.transform_mut().set_scale(Vector2::new(8.0, 20.0));
        right_pane
            .transform_mut()
            .set_scale(Vector2::new(8.0, 20.0));

        left_pane.transform_mut().set_z_order(1.0);
        right_pane.transform_mut().set_z_order(1.0);

        let cell_size = window.height() / 20.0;
        let width = window.width() / cell_size;
        let left_width = (width / 2.0) - 4.5 - 8.5;
        let right_width = (width / 2.0) + 4.5 - 17.5;

        let rem = 1.0 - left_width.fract();
        let mut left_tiles = Sprite::with_uv(
            window,
            Some(textures.tile().clone()),
            20.0,
            0.0,
            rem,
            left_width + rem,
        );
        let mut right_tiles = Sprite::with_uv(
            window,
            Some(textures.tile().clone()),
            20.0,
            0.0,
            0.0,
            right_width,
        );

        left_tiles
            .transform_mut()
            .set_position(Vector2::new(-8.5 - (left_width / 2.0), 10.5));
        right_tiles
            .transform_mut()
            .set_position(Vector2::new(17.5 + (right_width / 2.0), 10.5));

        left_tiles
            .transform_mut()
            .set_scale(Vector2::new(left_width, 20.0));
        right_tiles
            .transform_mut()
            .set_scale(Vector2::new(right_width, 20.0));

        Background {
            left_pane,
            right_pane,
            left_tiles,
            right_tiles,
        }
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.left_tiles.render(window);
        self.right_tiles.render(window);
        self.left_pane.render(window);
        self.right_pane.render(window);
    }
}
