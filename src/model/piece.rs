use colosseum::{Input, Window};
use std::time::SystemTime;

use super::{
    board::BOARD_WIDTH,
    tile::{Tile, TileColor},
};

#[derive(Debug)]
pub enum PieceClass {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

struct PieceTile {
    tile: Tile,
    offset: (isize, isize),
}

pub struct Piece {
    tiles: [PieceTile; 4],
    position: (isize, isize),
    even: bool,
    modified: bool,
}

pub struct PieceGenerator {
    mt: [u32; GEN_SIZE],
    mt_tempered: [u32; GEN_SIZE],
    index: usize,
    current_permutation: Vec<PieceClass>,
}

const GEN_SIZE: usize = 624;
const PERIOD: usize = 397;
const DIFF: usize = GEN_SIZE - PERIOD;
const MAGIC: u32 = 0x9908B0DF;

const PREVIEW_POSITION: (isize, isize) = (-(BOARD_WIDTH as isize) / 2 - 3, 3);
const DEFAULT_POSITION: (isize, isize) = (BOARD_WIDTH as isize / 2 - 1, 1);

fn convert_even_x(x: isize, even: bool) -> isize {
    if even {
        match x {
            -3 => -1,
            -1 => 0,
            1 => 1,
            3 => 2,
            _ => panic!("Invalid even x value"),
        }
    } else {
        x
    }
}

fn convert_even_y(y: isize, even: bool) -> isize {
    if even {
        match y {
            -3 => -2,
            -1 => -1,
            1 => 0,
            3 => 1,
            _ => panic!("Invalid even y value"),
        }
    } else {
        y
    }
}

fn sum_offsets(position: (isize, isize), offset: (isize, isize), even: bool) -> (isize, isize) {
    (
        position.0 + convert_even_x(offset.0, even),
        position.1 + convert_even_y(offset.1, even),
    )
}

impl Piece {
    pub fn new<I: Input>(class: PieceClass, window: &mut Window<I>) -> Self {
        let (offsets, even, color) = match class {
            PieceClass::I => (
                [(-3, -1), (-1, -1), (1, -1), (3, -1)],
                true,
                TileColor::Cyan,
            ),
            PieceClass::O => ([(-1, -1), (1, -1), (-1, 1), (1, 1)], true, TileColor::Blue),
            PieceClass::T => ([(-1, 0), (0, 0), (1, 0), (0, -1)], false, TileColor::Orange),
            PieceClass::S => (
                [(-1, 0), (0, 0), (0, -1), (1, -1)],
                false,
                TileColor::Yellow,
            ),
            PieceClass::Z => ([(-1, -1), (0, -1), (0, 0), (1, 0)], false, TileColor::Green),
            PieceClass::J => (
                [(-1, -1), (-1, 0), (0, 0), (1, 0)],
                false,
                TileColor::Purple,
            ),
            PieceClass::L => ([(-1, 0), (0, 0), (1, 0), (1, -1)], false, TileColor::Red),
        };

        Piece {
            tiles: offsets.map(|offset| PieceTile {
                tile: Tile::new(color, sum_offsets(PREVIEW_POSITION, offset, even), window),
                offset,
            }),
            position: PREVIEW_POSITION,
            even,
            modified: false,
        }
    }

    pub fn get_tile_position(&self, tile: usize) -> (isize, isize) {
        assert!(tile < 4);
        sum_offsets(self.position, self.tiles[tile].offset, self.even)
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        if self.modified {
            self.modified = false;
            self.update_positions();
        }

        for tile in &mut self.tiles {
            tile.tile.render(window);
        }
    }

    pub fn set_start_position(&mut self) {
        self.position = DEFAULT_POSITION;
        self.update_positions();
    }

    pub fn rotate_left(&mut self) {
        self.modified = true;
        for tile in &mut self.tiles {
            let old_x = tile.offset.0;
            tile.offset.0 = -tile.offset.1;
            tile.offset.1 = old_x;
        }
    }

    pub fn rotate_right(&mut self) {
        self.modified = true;
        for tile in &mut self.tiles {
            let old_x = tile.offset.0;
            tile.offset.0 = tile.offset.1;
            tile.offset.1 = -old_x;
        }
    }

    pub fn move_left(&mut self) {
        self.modified = true;
        self.position.0 -= 1;
    }

    pub fn move_right(&mut self) {
        self.modified = true;
        self.position.0 += 1;
    }

    pub fn move_down(&mut self) {
        self.modified = true;
        self.position.1 += 1;
    }

    pub fn move_up(&mut self) {
        self.modified = true;
        self.position.1 -= 1;
    }

    pub fn finalize(self) -> [(isize, isize, Tile); 4] {
        let mut result = [None, None, None, None];

        let mut i = 0;
        for tile in self.tiles {
            let (x, y) = sum_offsets(self.position, tile.offset, self.even);
            result[i] = Some((x, y, tile.tile));

            i += 1;
        }

        result.map(|val| val.unwrap())
    }

    fn update_positions(&mut self) {
        for tile in &mut self.tiles {
            tile.tile
                .set_position(sum_offsets(self.position, tile.offset, self.even))
        }
    }
}

impl PieceGenerator {
    pub fn new(seed: u32) -> Self {
        let mut mt = [0u32; GEN_SIZE];
        mt[0] = seed;

        for i in 1..GEN_SIZE {
            mt[i] = 0x6C078965u32
                .wrapping_mul(mt[i - 1] ^ mt[i - 1].wrapping_shr(30))
                .wrapping_add(i as u32);
        }

        PieceGenerator {
            mt,
            mt_tempered: [0; GEN_SIZE],
            index: GEN_SIZE,
            current_permutation: Vec::with_capacity(7),
        }
    }

    pub fn from_time() -> Self {
        PieceGenerator::new(
            (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                & 0xFFFFFFFF) as u32,
        )
    }

    pub fn next_piece_class(&mut self) -> PieceClass {
        if self.current_permutation.len() == 0 {
            self.generate_permuation();
        }

        self.current_permutation.pop().unwrap()
    }

    fn generate_permuation(&mut self) {
        const OPTIONS_2: u32 = 2;
        const OPTIONS_3: u32 = 3 * OPTIONS_2;
        const OPTIONS_4: u32 = 4 * OPTIONS_3;
        const OPTIONS_5: u32 = 5 * OPTIONS_4;
        const OPTIONS_6: u32 = 6 * OPTIONS_5;
        const OPTIONS_7: u32 = 7 * OPTIONS_6;

        let mut value = self.next_number() % OPTIONS_7;
        let index_6 = value / OPTIONS_6;
        value -= index_6 * OPTIONS_6;
        let index_5 = value / OPTIONS_5;
        value -= index_5 * OPTIONS_5;
        let index_4 = value / OPTIONS_4;
        value -= index_4 * OPTIONS_4;
        let index_3 = value / OPTIONS_3;
        value -= index_3 * OPTIONS_3;
        let index_2 = value / OPTIONS_2;
        value -= index_2 * OPTIONS_2;
        let index_1 = value;

        let mut remaining_pieces = vec![
            PieceClass::I,
            PieceClass::O,
            PieceClass::T,
            PieceClass::S,
            PieceClass::Z,
            PieceClass::J,
            PieceClass::L,
        ];

        self.current_permutation
            .push(remaining_pieces.remove(index_6 as usize));
        self.current_permutation
            .push(remaining_pieces.remove(index_5 as usize));
        self.current_permutation
            .push(remaining_pieces.remove(index_4 as usize));
        self.current_permutation
            .push(remaining_pieces.remove(index_3 as usize));
        self.current_permutation
            .push(remaining_pieces.remove(index_2 as usize));
        self.current_permutation
            .push(remaining_pieces.remove(index_1 as usize));
        self.current_permutation
            .push(remaining_pieces.pop().unwrap());
    }

    // Mersenne Twister 19937 Psuedo Random Number Generator
    fn next_number(&mut self) -> u32 {
        if self.index == GEN_SIZE {
            self.generate_numbers();
        }

        self.index += 1;
        self.mt_tempered[self.index - 1]
    }

    fn generate_numbers(&mut self) {
        for i in 0..DIFF {
            let y = (0x80000000 & self.mt[i]) | (0x7FFFFFFF & self.mt[i + 1]);
            self.mt[i] = self.mt[i + PERIOD]
                ^ y.wrapping_shr(1)
                ^ (y.wrapping_shl(31).wrapping_shr(31) & MAGIC);
        }

        for i in DIFF..GEN_SIZE - 1 {
            let y = (0x80000000 & self.mt[i]) | (0x7FFFFFFF & self.mt[i + 1]);
            self.mt[i] = self.mt[i - DIFF]
                ^ y.wrapping_shr(1)
                ^ (y.wrapping_shl(31).wrapping_shr(31) & MAGIC);
        }

        let y = (0x80000000 & self.mt[GEN_SIZE - 1]) | (0x7FFFFFFF & self.mt[0]);
        self.mt[GEN_SIZE - 1] =
            self.mt[PERIOD - 1] ^ y.wrapping_shr(1) ^ (y.wrapping_shl(31).wrapping_shr(31) & MAGIC);

        for i in 0..GEN_SIZE {
            let mut y = self.mt[i];
            y ^= y.wrapping_shr(11);
            y ^= y.wrapping_shl(7) & 0x9D2C5680;
            y ^= y.wrapping_shr(15) & 0xEFC60000;
            y ^= y.wrapping_shl(18);
            self.mt_tempered[i] = y;
        }

        self.index = 0;
    }
}
