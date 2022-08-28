use super::GameState;
use crate::model::{Game, ARE, BOARD_HEIGHT, BOARD_WIDTH};
use colosseum::{Camera, Projection, StateTrackingInput, Vector3, Window};

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

pub struct Playing {
    game: Game,
    camera: Camera,

    drop_counter: u8,
    das: DAS,

    are: ARE,

    frame_counter: usize,
}

const DAS_INITIAL_DELAY: u8 = 16;
const DAS_REPEAT_DELAY: u8 = 6;

impl Playing {
    pub fn new(
        starting_level: usize,
        window: &mut Window<StateTrackingInput>,
    ) -> Box<dyn GameState> {
        let unit_size = window.height() / BOARD_HEIGHT as f32;
        let width = window.width() / unit_size;

        let mut camera = Camera::new(window);
        camera.set_projection(Projection::orthographic(width, -1.0, 1.0), window);
        camera.set_position(Vector3::new(
            BOARD_WIDTH as f32 / 2.0 - 0.5,
            BOARD_HEIGHT as f32 / 2.0 + 0.5,
            0.0,
        ));

        let game = Game::new(starting_level, window);
        let drop_counter = game.drop_time();

        Box::new(Playing {
            game,
            camera,
            drop_counter,
            frame_counter: 0,
            das: DAS::None,
            are: ARE::None,
        })
    }

    fn finish_are(&mut self, window: &mut Window<StateTrackingInput>) {
        if self.game.finish_are(window) {
            panic!("Game Over");
        }
    }
}

impl GameState for Playing {
    fn update(
        &mut self,
        window: &mut colosseum::Window<colosseum::StateTrackingInput>,
    ) -> Option<Box<dyn GameState>> {
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
                    self.finish_are(window);
                    true
                } else {
                    false
                }
            }
            ARE::LineDelay(step, lines_cleared) => {
                if self.frame_counter % 4 == 0 {
                    self.game.clear_animation(*step, &lines_cleared);
                    *step += 1;
                    if *step == 5 {
                        self.game.collapse(lines_cleared);
                        self.finish_are(window);
                        true
                    } else {
                        false
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

        None
    }

    fn render(&mut self, window: &mut Window<StateTrackingInput>) {
        self.camera.set_active(window);
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
