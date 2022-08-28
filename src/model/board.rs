use super::{piece::Piece, tile::Tile};
use colosseum::{Input, Window};

pub struct Board {
    tiles: Box<[Option<Tile>]>,
}

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

impl Board {
    pub fn new() -> Self {
        let mut tiles = Vec::with_capacity(BOARD_WIDTH * BOARD_HEIGHT);
        for _ in 0..BOARD_WIDTH * BOARD_HEIGHT {
            tiles.push(None);
        }

        Board {
            tiles: tiles.into_boxed_slice(),
        }
    }

    pub fn verify(&self, piece: &Piece) -> bool {
        for i in 0..4 {
            let (x, y) = piece.get_tile_position(i);
            match self.get(x, y) {
                Ok(result) => match result {
                    Some(_) => return false,
                    None => {}
                },
                Err(()) => return false,
            }
        }

        true
    }

    pub fn get(&self, x: isize, y: isize) -> Result<Option<&Tile>, ()> {
        if x < 0 || x >= BOARD_WIDTH as isize || y < 0 || y >= BOARD_HEIGHT as isize {
            Err(())
        } else {
            Ok(self.tiles[x as usize + y as usize * BOARD_WIDTH].as_ref())
        }
    }

    pub fn take(&mut self, x: isize, y: isize) -> Result<Option<Tile>, ()> {
        if x < 0 || x >= BOARD_WIDTH as isize || y < 0 || y >= BOARD_HEIGHT as isize {
            Err(())
        } else {
            Ok(self.tiles[x as usize + y as usize * BOARD_WIDTH].take())
        }
    }

    pub fn set(&mut self, x: isize, y: isize, mut tile: Option<Tile>) {
        assert!(x >= 0 && x < BOARD_WIDTH as isize);
        assert!(y >= 0 && y < BOARD_HEIGHT as isize);

        tile.as_mut().map(|tile| tile.set_position((x, y)));
        self.tiles[x as usize + y as usize * BOARD_WIDTH] = tile;
    }

    pub fn finalize(&mut self, piece: Piece) {
        let tiles = piece.finalize();
        for (x, y, tile) in tiles {
            self.set(x, y, Some(tile));
        }
    }

    pub fn check(&self) -> Vec<isize> {
        let mut cleared = Vec::with_capacity(4);
        'main: for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.get(x as isize, y as isize).unwrap().is_none() {
                    continue 'main;
                }
            }

            cleared.push(y as isize);
        }

        cleared
    }

    pub fn collapse(&mut self, lines: &[isize]) {
        for line in lines {
            for y in (1..*line + 1).rev() {
                for x in 0..BOARD_WIDTH {
                    let tile = self.take(x as isize, y as isize - 1).unwrap();
                    self.set(x as isize, y as isize, tile);
                }
            }
        }
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        for tile in self.tiles.iter_mut() {
            match tile.as_mut() {
                Some(tile) => tile.render(window),
                None => {}
            }
        }
    }

    pub fn clear_animation(&mut self, step: isize, lines: &[isize]) {
        for line in lines {
            self.set(BOARD_WIDTH as isize / 2 - 1 - step, *line, None);
            self.set(BOARD_WIDTH as isize / 2 + step, *line, None);
        }
    }
}
