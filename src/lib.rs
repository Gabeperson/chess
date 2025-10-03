// #![warn(clippy::all)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]

mod utils;

mod movegen {
    use crate::utils::lsb1;

    use super::*;

    pub const NONE_PIECE_TYPE: u8 = 0x0;
    pub const PAWN_PIECE_TYPE: u8 = 0x1;
    pub const KNIGHT_PIECE_TYPE: u8 = 0x2;
    pub const BISHOP_PIECE_TYPE: u8 = 0x3;
    pub const ROOK_PIECE_TYPE: u8 = 0x4;
    pub const QUEEN_PIECE_TYPE: u8 = 0x5;
    pub const KING_PIECE_TYPE: u8 = 0x6;
    

    pub const PROMO_KNIGHT: u8 = 0x0;
    pub const PROMO_BISHOP: u8 = 0x1;
    pub const PROMO_ROOK: u8 = 0x2;
    pub const PROMO_QUEEN: u8 = 0x3;

    // bits 0-5: "from" file/rank
    // bits 6-11: "to" file/rank

    // bits 12-14: moving piece type (1-6)
    // bit 15-17: captured piece type (1-5, 0 if no capture)
    //     1: pawn
    //     2: knight
    //     3: bishop
    //     4: rook
    //     5: queen
    //     6: king

    // bit 18-19: promotion piece type
    //     Promotion:
    //     0: knight
    //     1: bishop
    //     2: rook
    //     3: queen
    // Piece type:
    // bit 20: previous state ep possible?
    // bit 21-23: previous state ep file
    // bit 24-27: castling rights before move
    //    24: white queenside
    //    25: white kingside
    //    26: black queenside
    //    27: black kingside
    // bit 28: current move is en passant?
    #[repr(transparent)]
    #[derive(Debug, Clone, Default)]
    pub struct Move(u32);

    impl Move {
        pub fn new() -> Self {
            Self(0)
        }
        pub fn from_square(&self) -> usize {
            (self.0 & 0b111111) as usize
        }
        pub fn to_square(&self) -> usize {
            ((self.0 >> 6) & 0b111111) as usize
        }
        pub fn moving_piece(&self) -> u8 {
            ((self.0 >> 12) & 0b111) as u8
        }
        pub fn captured_piece(&self) -> u8 {
            ((self.0 >> 15) & 0b111) as u8
        }
        pub fn promotion_piece(&self) -> u8 {
            ((self.0 >> 18) & 0b11) as u8
        }
        pub fn is_prev_ep(&self) -> bool {
            ((self.0 >> 20) & 0b1) != 0
        }
        pub fn prev_ep_file(&self) -> u8 {
            ((self.0 >> 21) & 0b111) as u8
        }
        pub fn castling_rights(&self) -> CastlingRights {
            CastlingRights {
                white_queenside: ((self.0 >> 24) & 0b1) != 0,
                white_kingside: ((self.0 >> 25) & 0b1) != 0,
                black_queenside: ((self.0 >> 26) & 0b1) != 0,
                black_kingside: ((self.0 >> 27) & 0b1) != 0,
            }
        }
        pub fn is_ep(&self) -> bool {
            ((self.0 >> 28) & 0b1) != 0
        }
        pub fn set_from_square(&mut self, square: usize) {
            self.0 |= (square as u32) & 0b111111;
        }
        pub fn set_to_square(&mut self, square: usize) {
            self.0 |= ((square as u32) & 0b111111) << 6;
        }
        pub fn set_moving_piece(&mut self, piece: u8) {
            self.0 |= ((piece as u32) & 0b111) << 12;
        }
        pub fn set_captured_piece(&mut self, piece: u8) {
            self.0 |= ((piece as u32) & 0b111) << 15;
        }
        pub fn set_promotion_piece(&mut self, piece: u8) {
            self.0 |= ((piece as u32) & 0b11) << 18;
        }
        pub fn set_prev_ep(&mut self, ep: Option<u8>) {
            if let Some(file) = ep {
                self.0 |= 1 << 20;
                self.0 |= ((file as u32) & 0b111) << 21;
            }
        }
        pub fn set_castling_rights(&mut self, rights: CastlingRights) {
            if rights.white_queenside {
                self.0 |= 1 << 24;
            }
            if rights.white_kingside {
                self.0 |= 1 << 25;
            }
            if rights.black_queenside {
                self.0 |= 1 << 26;
            }
            if rights.black_kingside {
                self.0 |= 1 << 27;
            }
        }
        pub fn set_is_ep(&mut self) {
            self.0 |= 1 << 28;
        }
    }

    pub type Moves = tinyvec::ArrayVec<[Move; 218]>;

    impl Board {
        pub fn generate_moves(&self, moves: &mut Moves, side: Side) {
            todo!();
        }
        pub fn king_moves(&self, moves: &mut Moves, side: Side) {
            let king = self.kings[side];
            let king = lsb1(king);
            let move_bb = self.king_attack_table[king];
            let opponent_attacks = self.attacks(!side);
            // King can move to squares where
            // 1. It can reach it
            // 2. It is not under attack by opponent pieces
            // 3. It is not occupied by friendly pieces
            let mut allowed_bb = move_bb & !opponent_attacks & self.pieces[side ];
            while allowed_bb != 0 {
                let lsb = lsb1(allowed_bb);
                allowed_bb &= !(1 << lsb);
                let mut mv = Move::new();
                mv.set_from_square(king);
                mv.set_to_square(lsb);
                mv.set_moving_piece(KING_PIECE_TYPE);
                if (self.pieces[!side] & (1 << lsb)) != 0 {
                    mv.set_captured_piece(self.arr[lsb]);
                }
                mv.set_prev_ep(self.ep);
                mv.set_castling_rights(self.castling);
                moves.push(mv);
            }
        }
        pub fn other_moves(&self, moves: &mut Moves, side: Side) {
            let king = lsb1(self.kings[side]);
            let king_bishop_attacks = self.bishop_table.lookup(king, self.occupied);
            let king_rook_attacks = self.rook_table.lookup(king, self.occupied);
            let mut pawns = self.pawns[side];
            let mut knights = self.knights[side];
            let mut bishops = self.bishops[side];
            let mut rooks = self.rooks[side];
            let mut queens = self.queens[side];

            todo!()
        }
        // pub fn diagonal_moves(&self, moves: &mut Moves, side: Side) {
        //     let king = lsb1(self.kings[side]);
        //     let king_bishop_attacks = self.bishop_table.lookup(king, self.occupied);
            
        // }
        // pub fn straight_moves(&self, moves: &mut Moves, side: Side) {
        //     todo!()
        // }
        // pub fn pinned(&self, side: Side) -> u64 {
        //     let king = lsb1(self.kings[side]);
        //     let king_bishop_attacks = self.bishop_table.lookup(king, self.occupied);
        //     let king_rook_attacks = self.rook_table.lookup(king, self.occupied);
            
        //     let removed_king_bishop_attacks = self.bishop_table.lookup(king, self.occupied & !king_bishop_attacks);
        //     let removed_king_rook_attacks = self.rook_table.lookup(king, self.occupied & !king_rook_attacks);
        //     pinned
        // }
        pub fn attacks(&self, side: Side) -> u64 {
            let pawn_attacks = if side == WHITE{
                self.w_pawn_attacks()
            } else {
                self.b_pawn_attacks()
            };
            let king = self.kings[side];
            pawn_attacks
                & self.knight_attacks(side)
                & self.bishop_attacks(side)
                & self.rook_attacks(side)
                & self.queen_attacks(side)
                & self.king_attack_table[lsb1(king)]
        }
    }
}

#[derive(Clone, Debug, Hash, Copy, PartialEq, Eq)]
pub enum Side {
    WHITE = 0,
    BLACK = 1,
}
use Side::*;
use std::ops::{Index, IndexMut, Not};
impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            WHITE => BLACK,
            BLACK => WHITE,
        }
    }
}
impl<T> Index<Side> for [T; 2] {
    type Output = T;

    fn index(&self, index: Side) -> &Self::Output {
        &self[index]
    }
}
impl<T> IndexMut<Side> for [T; 2] {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        &mut self[index]
    }
}


struct Board {
    pawns: [u64; 2],
    knights: [u64; 2],
    bishops: [u64; 2],
    rooks: [u64; 2],
    queens: [u64; 2],
    kings: [u64; 2],

    pieces: [u64; 2],

    arr: [u8; 64],

    occupied: u64,
    empty: u64,

    ep: Option<u8>,
    castling: CastlingRights,
    side_to_move: Side,

    pawn_attack_table: [[u64; 64]; 2],
    knight_attack_table: [u64; 64],
    king_attack_table: [u64; 64],

    rook_table: RookTable,
    bishop_table: BishopTable,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Copy)]
struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

mod sliding_attacks;
pub use sliding_attacks::{BishopTable, RookTable};
use utils::lsb1;

const RANK8: u64 = 0xFF00_0000_0000_0000;
const RANK7: u64 = 0x00FF_0000_0000_0000;
const RANK6: u64 = 0x0000_FF00_0000_0000;
const RANK5: u64 = 0x0000_00FF_0000_0000;
const RANK4: u64 = 0x0000_0000_FF00_0000;
const RANK3: u64 = 0x0000_0000_00FF_0000;
const RANK2: u64 = 0x0000_0000_0000_FF00;
const RANK1: u64 = 0x0000_0000_0000_00FF;

const FILE_A: u64 = 0x0101_0101_0101_0101;
const FILE_B: u64 = 0x0202_0202_0202_0202;
const FILE_C: u64 = 0x0404_0404_0404_0404;
const FILE_D: u64 = 0x0808_0808_0808_0808;
const FILE_E: u64 = 0x1010_1010_1010_1010;
const FILE_F: u64 = 0x2020_2020_2020_2020;
const FILE_G: u64 = 0x4040_4040_4040_4040;
const FILE_H: u64 = 0x8080_8080_8080_8080;

impl Board {
    fn w_pawn_attacks(&self) -> u64 {
        let pawns = self.pawns[WHITE];
        (pawns << 9) & !FILE_A | (pawns << 7) & !FILE_H
    }
    fn b_pawn_attacks(&self) -> u64 {
        let pawns = self.pawns[BLACK];
        (pawns >> 9) & !FILE_H | (pawns >> 7) & !FILE_A
    }
    fn knight_attacks(&self, side: Side) -> u64 {
        let mut knights = self.knights[side];
        let mut attacks = 0u64;
        while knights != 0 {
            let lsb = lsb1(knights);
            knights &= !(1 << lsb);
            attacks |= self.knight_attack_table[lsb];
        }
        attacks
    }
    fn bishop_attacks(&self, side: Side) -> u64 {
        let mut bishops = self.bishops[side];
        let mut attacks = 0u64;
        while bishops != 0 {
            let lsb = lsb1(bishops);
            bishops &= !(1 << lsb);
            attacks |= self.bishop_table.lookup(lsb, self.occupied);
        }
        attacks
    }
    fn rook_attacks(&self, side: Side) -> u64 {
        let mut rooks = self.rooks[side];
        let mut attacks = 0u64;
        while rooks != 0 {
            let lsb = lsb1(rooks);
            rooks &= !(1 << lsb);
            attacks |= self.rook_table.lookup(lsb, self.occupied);
        }
        attacks
    }
    fn queen_attacks(&self, side: Side) -> u64 {
        let mut queens = self.queens[side];
        let mut attacks = 0u64;
        while queens != 0 {
            let lsb = lsb1(queens);
            queens &= !(1 << lsb);
            attacks |= self.rook_table.lookup(lsb, self.occupied)
                | self.bishop_table.lookup(lsb, self.occupied);
        }
        attacks
    }
}

impl Board {
    // fn w_pawn_push_targets(pawns: u64, empty: u64) -> u64 {
    //     pawns << 8 & empty
    // }

    // fn w_pawn_double_push_targets(pawns: u64, empty: u64) -> u64 {
    //     let single_push = w_pawn_push_targets(pawns, empty);
    //     single_push << 8 & empty & RANK4
    // }

    // fn b_pawn_push_targets(pawns: u64, empty: u64) -> u64 {
    //     pawns >> 8 & empty
    // }

    // fn b_pawn_double_push_targets(pawns: u64, empty: u64) -> u64 {
    //     let single_push = b_pawn_push_targets(pawns, empty);
    //     single_push >> 8 & empty & RANK5
    // }

    // fn w_pawn_attacks_target(pawns: u64, targets: u64) -> u64 {
    //     w_pawn_attacks(pawns) & targets
    // }

    // fn b_pawn_attacks_target(pawns: u64, targets: u64) -> u64 {
    //     b_pawn_attacks(pawns) & targets
    // }
}

pub fn pawn_attack_table() -> [[u64; 64]; 2] {
    let mut arr = [[0u64; 64]; 2];
    for i in 0..64 {
        let bitboard = 1u64 << i;
        arr[WHITE ][i] = (bitboard << 9) & !FILE_A | (bitboard << 7) & !FILE_H;
        arr[BLACK ][i] = (bitboard >> 9) & !FILE_H | (bitboard >> 7) & !FILE_A;
    }
    arr
}

pub fn knight_attack_table() -> [u64; 64] {
    let mut arr = [0u64; 64];
    for i in 0..64 {
        let bitboard = 1u64 << i;
        arr[i] = (bitboard << 17) & !FILE_A // up right
            | (bitboard << 15) & !FILE_H // up left
            | (bitboard << 10) & !(FILE_A | FILE_B) // left up
            | (bitboard << 6) & !(FILE_H | FILE_G) // right up
            | (bitboard >> 6) & !(FILE_A | FILE_B) // left down
            | (bitboard >> 10) & !(FILE_H | FILE_G) // right down
            | (bitboard >> 15) & !FILE_A // down right
            | (bitboard >> 17) & !FILE_H; // down left
    }
    arr
}

pub fn king_attack_table() -> [u64; 64] {
    let mut arr = [0u64; 64];
    for i in 0..64 {
        let bitboard = 1u64 << i;
        arr[i] = (bitboard << 8) // up
            | (bitboard >> 8) // down
            | (bitboard << 1) & !FILE_A // right
            | (bitboard >> 1) & !FILE_H // left
            | (bitboard << 9) & !FILE_A // up right
            | (bitboard << 7) & !FILE_H // up left
            | (bitboard >> 9) & !FILE_H // down left
            | (bitboard >> 7) & !FILE_A; // down right
    }
    arr
}

pub fn print_bb(bitboard: u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;
            if (bitboard >> square) & 1 == 1 {
                print!("\x1b[32m1 \x1b[0m");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
    println!();
}
