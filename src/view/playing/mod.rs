use self::{background::Background, number::Number};
use crate::model::Game;
use colosseum::{Input, StateTrackingInput, Texture, Vector2, Window};

mod background;
mod number;
mod textures;

pub use textures::*;

pub struct PlayingUI {
    digits: Box<[Texture]>,

    background: Background,

    score: Number<6>,
    top_score: Number<6>,
    lines_level: Number<3>,
    lines_total: Number<6>,
    level: Number<2>,

    stats: Box<[Number<3>]>,
}

impl PlayingUI {
    pub fn new(
        game: &Game,
        top_score: usize,
        textures: &Textures,
        window: &mut Window<StateTrackingInput>,
    ) -> Self {
        let mut digits = Vec::with_capacity(10);
        digits.extend(textures.digits().iter().map(|texture| texture.clone()));

        let mut score = Number::new(game.score(), textures.digits(), window);
        score.set_position(Vector2::new(13.5, 18.0));

        let mut top_score = Number::new(top_score, textures.digits(), window);
        top_score.set_position(Vector2::new(13.5, 16.0));

        let mut lines_level = Number::new(game.level_lines(), textures.digits(), window);
        lines_level.set_position(Vector2::new(13.5, 11.0));

        let mut lines_total = Number::new(game.total_lines(), textures.digits(), window);
        lines_total.set_position(Vector2::new(13.5, 9.0));

        let mut level = Number::new(game.level(), textures.digits(), window);
        level.set_position(Vector2::new(13.5, 5.0));

        let mut stats = Vec::with_capacity(7);
        let mut y = 11.5;
        for stat in game.stats() {
            let mut number = Number::new(*stat, textures.digits(), window);
            number.set_position(Vector2::new(-3.0 - (7.0 / 16.0), y));
            stats.push(number);
            y -= 1.5;
        }

        PlayingUI {
            digits: digits.into_boxed_slice(),
            background: Background::new(textures, window),
            score,
            top_score,
            lines_level,
            lines_total,
            level,
            stats: stats.into_boxed_slice(),
        }
    }

    pub fn update(&mut self, game: &Game) {
        self.score.set_value(game.score(), &self.digits);
        self.lines_total.set_value(game.total_lines(), &self.digits);
        self.lines_level.set_value(game.level_lines(), &self.digits);
        self.level.set_value(game.level(), &self.digits);

        let stats = game.stats();
        for i in 0..7 {
            self.stats[i].set_value(stats[i], &self.digits);
        }
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.background.render(window);
        self.score.render(window);
        self.top_score.render(window);
        self.lines_level.render(window);
        self.lines_total.render(window);
        self.level.render(window);

        for stat in self.stats.iter_mut() {
            stat.render(window);
        }
    }
}
