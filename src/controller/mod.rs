mod playing;

pub trait GameState {
    fn update(
        &mut self,
        window: &mut colosseum::Window<colosseum::StateTrackingInput>,
    ) -> Option<Box<dyn GameState>>;

    fn render(&mut self, window: &mut colosseum::Window<colosseum::StateTrackingInput>);
}

pub struct Game {
    current_state: Box<dyn GameState>,
}

impl colosseum::Game for Game {
    type Input = colosseum::StateTrackingInput;

    const INITIAL_TITLE: &'static str = "Tetris Clone";
    const INITIAL_FIXED_UPDATE_DELTA_TIME: Option<f32> = Some(1.0 / 60.0);

    fn new(window: &mut colosseum::Window<Self::Input>) -> Self {
        Game {
            current_state: playing::Playing::new(0, window),
        }
    }

    fn update(&mut self, _: f32, _: &mut colosseum::Window<Self::Input>) {}

    fn fixed_update(&mut self, window: &mut colosseum::Window<Self::Input>) {
        match self.current_state.update(window) {
            Some(new_state) => self.current_state = new_state,
            None => {}
        }
    }

    fn render(&mut self, window: &mut colosseum::Window<Self::Input>) {
        self.current_state.render(window)
    }

    fn clear_color(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
    }
}
