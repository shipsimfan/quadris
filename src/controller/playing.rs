use super::GameState;
use crate::{
    model::{Game, ARE, BOARD_HEIGHT, BOARD_WIDTH},
    view::{PlayingUI, Textures},
};
use colosseum::{Camera, Input, Projection, StateTrackingInput, Vector3, Window};

#[derive(PartialEq, Eq)]
enum DASKey {
    Left,
    Right,
    Down,
    RotateLeft,
    RotateRight,
}

enum DAS {
    None,
    Active(DASKey, u8),
}

pub enum NextState {
    GameOver,
    Pause,
}

pub struct Playing {
    game: Game,
    camera: Camera,

    drop_counter: u8,
    das: DAS,

    are: ARE,

    frame_counter: usize,

    ui: PlayingUI,
}

const DAS_INITIAL_DELAY: u8 = 16;
const DAS_REPEAT_DELAY: u8 = 6;

impl Playing {
    pub fn new(
        starting_level: usize,
        textures: &Textures,
        window: &mut Window<StateTrackingInput>,
    ) -> GameState {
        let unit_size = window.height() / BOARD_HEIGHT as f32;
        let width = window.width() / unit_size;

        let mut camera = Camera::new(window);
        camera.set_projection(Projection::orthographic(width, -0.1, 2.1), window);
        camera.set_position(Vector3::new(
            BOARD_WIDTH as f32 / 2.0 - 0.5,
            BOARD_HEIGHT as f32 / 2.0 + 0.5,
            0.0,
        ));

        let game = Game::new(starting_level, textures.tile().clone(), window);
        let drop_counter = game.drop_time();
        let ui = PlayingUI::new(&game, 0, textures, window);

        GameState::Playing(Playing {
            game,
            camera,
            drop_counter,
            frame_counter: 0,
            das: DAS::None,
            are: ARE::None,
            ui,
        })
    }

    pub fn update(
        &mut self,
        window: &mut colosseum::Window<colosseum::StateTrackingInput>,
    ) -> Option<NextState> {
        if window.input().get_key(0x1B) {
            return Some(NextState::Pause);
        }

        // Update ARE & frame counter
        self.frame_counter += 1;
        match &mut self.are {
            ARE::ARE(value) => *value -= 1,
            _ => {}
        }

        // Read input & update game
        if window.input().get_key(b'A') {
            if self.das.add_key_frame(DASKey::Left) {
                self.game.move_left();
            }
        } else if window.input().get_key(b'D') {
            if self.das.add_key_frame(DASKey::Right) {
                self.game.move_right();
            }
        } else if window.input().get_key(b'Q') {
            if self.das.add_key_frame(DASKey::RotateLeft) {
                self.game.rotate_left();
            }
        } else if window.input().get_key(b'E') {
            if self.das.add_key_frame(DASKey::RotateRight) {
                self.game.rotate_right();
            }
        } else if window.input().get_key(b'S') {
            if self.das.add_key_frame(DASKey::Down) {
                match self.game.move_down(true) {
                    Some(are) => match self.are {
                        ARE::None => self.are = are,
                        _ => {}
                    },
                    None => {}
                }
            }
        } else {
            self.das = DAS::None;
        }

        if match &mut self.are {
            ARE::ARE(step) => {
                if *step == 0 {
                    if self.game.finish_are(window) {
                        return Some(NextState::GameOver);
                    }
                    true
                } else {
                    false
                }
            }
            ARE::LineDelay(step, lines_cleared) => {
                if self.frame_counter % 4 == 0 {
                    if *step < 5 {
                        self.game.clear_animation(*step, &lines_cleared);
                        *step += 1;
                        false
                    } else {
                        self.game.collapse(lines_cleared);
                        if self.game.finish_are(window) {
                            return Some(NextState::GameOver);
                        }
                        true
                    }
                } else {
                    false
                }
            }
            ARE::None => {
                if self.drop_counter == 0 {
                    self.drop_counter = self.game.drop_time();
                    match self.game.move_down(false) {
                        Some(are) => self.are = are,
                        None => {}
                    }
                } else {
                    self.drop_counter -= 1;
                }
                false
            }
        } {
            self.are = ARE::None;
        }

        self.ui.update(&self.game);
        None
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.camera.set_active(window);
        self.ui.render(window);
        self.game.render(window);
    }
}

impl DAS {
    pub fn add_key_frame(&mut self, key: DASKey) -> bool {
        match self {
            DAS::None => {
                *self = DAS::Active(key, DAS_INITIAL_DELAY);
                true
            }
            DAS::Active(das_key, count) => {
                if *das_key != key {
                    *self = DAS::Active(key, DAS_INITIAL_DELAY);
                    true
                } else {
                    *count -= 1;
                    if *count == 0 {
                        *self = DAS::Active(key, DAS_REPEAT_DELAY);
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}
