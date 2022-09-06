use colosseum::{Input, Sprite, Texture, Vector2, Window};

pub struct Number<const DIGITS: usize> {
    digits: Box<[Sprite]>,
    last_value: usize,
}

const fn max_value(digits: usize) -> usize {
    10usize.pow(digits as u32 + 1) - 1
}

fn offset(digits: usize) -> f32 {
    ((digits - 1) as f32) / 2.0
}

impl<const DIGITS: usize> Number<DIGITS> {
    pub fn new<I: Input>(
        initial_value: usize,
        digit_textures: &[Texture],
        window: &mut Window<I>,
    ) -> Self {
        let initial_value = initial_value.min(max_value(DIGITS));

        let mut digits = Vec::with_capacity(DIGITS);
        let mut value = initial_value;
        let offset = offset(DIGITS);
        for i in (0..DIGITS).rev() {
            let digit = value % 10;
            value /= 10;

            let mut sprite = Sprite::new(Some(digit_textures[digit].clone()));
            sprite
                .transform_mut()
                .set_position(Vector2::new(i as f32 - offset, 0.0));
            digits.push(sprite);
        }

        digits.reverse();

        Number {
            digits: digits.into_boxed_slice(),
            last_value: initial_value,
        }
    }

    pub fn set_position(&mut self, position: Vector2) {
        let offset = offset(DIGITS);
        for i in 0..DIGITS {
            self.digits[i]
                .transform_mut()
                .set_position(Vector2::new(position.x() + i as f32 - offset, position.y()));
        }
    }

    pub fn set_value(&mut self, value: usize, digit_textures: &[Texture]) {
        if value == self.last_value {
            return;
        }

        let mut v = value;
        for i in (0..DIGITS).rev() {
            let d = v % 10;
            v /= 10;

            self.digits[i].set_texture(Some(digit_textures[d].clone()));
        }

        self.last_value = value;
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        for digit in self.digits.iter_mut() {
            digit.render(window);
        }
    }
}
