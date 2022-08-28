use super::{
    board::Board,
    piece::{Piece, PieceGenerator},
};
use colosseum::{Input, Window};

pub struct Game {
    board: Board,
    level: usize,
    score: usize,
    lines_cleared: usize,
    lines_target: usize,
    current_piece: Option<Piece>,
    next_piece: Piece,
    piece_generator: PieceGenerator,
}

const DROP_TIMES: &[u8] = &[
    48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3,
];

const MAX_SCORE: usize = 999999;

impl Game {
    pub fn new<I: Input>(starting_level: usize, window: &mut Window<I>) -> Self {
        let mut piece_generator = PieceGenerator::from_time();
        let mut current_piece = Piece::new(piece_generator.next_piece_class(), window);
        current_piece.set_start_position();

        Game {
            board: Board::new(),
            level: starting_level,
            score: 0,
            lines_cleared: 0,
            lines_target: (starting_level * 10 + 10)
                .min((starting_level as isize * 10 - 50).max(100) as usize),
            current_piece: Some(current_piece),
            next_piece: Piece::new(piece_generator.next_piece_class(), window),
            piece_generator,
        }
    }

    pub fn drop_time(&self) -> u8 {
        if self.level >= DROP_TIMES.len() {
            if self.level >= 29 {
                1
            } else {
                2
            }
        } else {
            DROP_TIMES[self.level]
        }
    }

    pub fn rotate_left(&mut self) {
        self.current_piece.as_mut().map(|current_piece| {
            current_piece.rotate_left();
            if !self.board.verify(current_piece) {
                current_piece.rotate_right();
            }
        });
    }

    pub fn rotate_right(&mut self) {
        self.current_piece.as_mut().map(|current_piece| {
            current_piece.rotate_right();
            if !self.board.verify(current_piece) {
                current_piece.rotate_left();
            }
        });
    }

    pub fn move_left(&mut self) {
        self.current_piece.as_mut().map(|current_piece| {
            current_piece.move_left();
            if !self.board.verify(current_piece) {
                current_piece.move_right();
            }
        });
    }

    pub fn move_right(&mut self) {
        self.current_piece.as_mut().map(|current_piece| {
            current_piece.move_right();
            if !self.board.verify(current_piece) {
                current_piece.move_left();
            }
        });
    }

    pub fn finish_are<I: Input>(&mut self, window: &mut Window<I>) -> bool {
        // Generate new piece
        let mut piece = Piece::new(self.piece_generator.next_piece_class(), window);

        // Set it as the next piece
        std::mem::swap(&mut self.next_piece, &mut piece);

        // Update the new current piece position
        piece.set_start_position();

        // Check for game over
        if !self.board.verify(&piece) {
            return true;
        }

        // Set the new current piece
        self.current_piece = Some(piece);

        false
    }

    pub fn move_down(&mut self, soft_drop: bool) -> Option<(u8, bool)> {
        match self.current_piece.as_mut() {
            Some(current_piece) => {
                current_piece.move_down();
                if self.board.verify(current_piece) {
                    return None;
                }
            }
            None => {
                if soft_drop {
                    self.add_score(1);
                }
                return None;
            }
        }

        // Get current piece
        let mut current_piece = self.current_piece.take().unwrap();
        current_piece.move_up();
        let line_locked = {
            let mut lowest_y = current_piece.get_tile_position(0).1;

            for i in 1..4 {
                let y = current_piece.get_tile_position(i).1;
                if y < lowest_y {
                    lowest_y = y;
                }
            }

            lowest_y
        };

        // Effect the board
        self.board.finalize(current_piece);
        let lines_cleared = self.board.check();

        // Update score
        self.add_score(
            (self.level + 1)
                * match lines_cleared {
                    0 => 0,
                    1 => 40,
                    2 => 100,
                    3 => 300,
                    4 => 1200,
                    _ => panic!("Impossible number of lines cleared!"),
                },
        );

        // Update level
        self.lines_cleared += lines_cleared;

        if self.lines_cleared >= self.lines_target {
            self.lines_cleared = 0;
            self.level += 1;
            self.lines_target += 10;
        }

        Some((
            if line_locked < 2 {
                10
            } else {
                10 + (((line_locked as u8 - 2) / 4) + 1) * 2
            },
            lines_cleared > 0,
        ))
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.board.render(window);
        match self.current_piece.as_mut() {
            Some(current_piece) => current_piece.render(window),
            None => {}
        }
        self.next_piece.render(window);
    }

    fn add_score(&mut self, score: usize) {
        self.score += score;
        if self.score > MAX_SCORE {
            self.score = MAX_SCORE;
        }
    }
}
