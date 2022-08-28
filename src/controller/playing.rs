use super::GameState;
use crate::model::{Game, BOARD_HEIGHT, BOARD_WIDTH};
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

enum ARE {
    None,
    ARE(u8),
    LineDelay(u8, u8),
}

pub struct Playing {
    game: Game,
    camera: Camera,

    drop_counter: u8,
    das: DAS,

    are: ARE,
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
            das: DAS::None,
            are: ARE::None,
        })
    }
}

impl GameState for Playing {
    fn update(
        &mut self,
        window: &mut colosseum::Window<colosseum::StateTrackingInput>,
    ) -> Option<Box<dyn GameState>> {
        // Update ARE
        self.are.tick();

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
                    Some((new_are, line_clear)) => match self.are {
                        ARE::None => {
                            if line_clear {
                                self.are = ARE::LineDelay(20, new_are);
                            } else {
                                self.are = ARE::ARE(new_are);
                            }
                        }
                        _ => {}
                    },
                    None => {}
                }
            }
        } else {
            self.das = DAS::None;
        }

        if self.are.over() {
            match self.are {
                ARE::ARE(_) => {
                    self.are = ARE::None;
                    if self.game.finish_are(window) {
                        panic!("Game Over");
                    }
                }
                ARE::LineDelay(_, new_are) => self.are = ARE::ARE(new_are),
                ARE::None => {}
            }
        } else if self.are.none() {
            if self.drop_counter == 0 {
                self.drop_counter = self.game.drop_time();
                match self.game.move_down(false) {
                    Some((new_are, line_clear)) => {
                        if line_clear {
                            self.are = ARE::LineDelay(20, new_are);
                        } else {
                            self.are = ARE::ARE(new_are);
                        }
                    }
                    None => {}
                }
            } else {
                self.drop_counter -= 1;
            }
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

impl ARE {
    pub fn tick(&mut self) {
        match self {
            ARE::None => {}
            ARE::ARE(value) | ARE::LineDelay(value, _) => *value -= 1,
        }
    }

    pub fn none(&self) -> bool {
        match self {
            ARE::None => true,
            _ => false,
        }
    }

    pub fn over(&self) -> bool {
        match self {
            ARE::None => false,
            ARE::ARE(value) | ARE::LineDelay(value, _) => *value == 0,
        }
    }
}
