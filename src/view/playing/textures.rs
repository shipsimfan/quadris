use colosseum::{Input, SampleType, Texture, Window};

pub struct Textures {
    digits: Box<[Texture]>,
    background_left: Texture,
    background_right: Texture,
    tile: Texture,
}

impl Textures {
    pub fn load<I: Input>(window: &mut Window<I>) -> Self {
        let mut digits = Vec::with_capacity(10);
        for i in 0..10 {
            digits.push(Texture::load(
                format!("./textures/{}.qoi", i),
                SampleType::Point,
                window,
            ));
        }

        Textures {
            digits: digits.into_boxed_slice(),
            background_left: Texture::load(
                "./textures/background_left.qoi",
                SampleType::Point,
                window,
            ),
            background_right: Texture::load(
                "./textures/background_right.qoi",
                SampleType::Point,
                window,
            ),
            tile: Texture::load("./textures/tile.qoi", SampleType::Point, window),
        }
    }

    pub fn tile(&self) -> &Texture {
        &self.tile
    }

    pub fn digits(&self) -> &[Texture] {
        &self.digits
    }

    pub fn background_left(&self) -> &Texture {
        &self.background_left
    }

    pub fn background_right(&self) -> &Texture {
        &self.background_right
    }
}
