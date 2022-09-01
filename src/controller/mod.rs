use self::playing::{NextState as PlayingNextState, Playing};
use crate::view::Textures;
use colosseum::{Input, StateTrackingInput};

mod playing;

pub enum GameState {
    Playing(Playing),
}

pub struct Game {
    current_state: GameState,
}

impl colosseum::Game for Game {
    type Input = colosseum::StateTrackingInput;

    const INITIAL_TITLE: &'static str = "Tetris Clone";
    const INITIAL_FIXED_UPDATE_DELTA_TIME: Option<f32> = Some(1.0 / 60.0);

    fn new(window: &mut colosseum::Window<Self::Input>) -> Self {
        let textures = Textures::load(window);

        Game {
            current_state: playing::Playing::new(0, &textures, window),
        }
    }

    fn update(&mut self, _: f32, _: &mut colosseum::Window<Self::Input>) {}

    fn fixed_update(&mut self, window: &mut colosseum::Window<Self::Input>) {
        self.current_state.update(window);
    }

    fn render(&mut self, window: &mut colosseum::Window<Self::Input>) {
        self.current_state.render(window)
    }

    fn clear_color(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
    }
}

impl GameState {
    pub fn update(&mut self, window: &mut colosseum::Window<StateTrackingInput>) {
        match self {
            Self::Playing(playing) => match playing.update(window) {
                Some(next_state) => match next_state {
                    PlayingNextState::GameOver => panic!("Game Over!"),
                    PlayingNextState::Pause => panic!("Pause!"),
                },
                None => {}
            },
        }
    }

    pub fn render<I: Input>(&mut self, window: &mut colosseum::Window<I>) {
        match self {
            Self::Playing(playing) => playing.render(window),
        }
    }
}
