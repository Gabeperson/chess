#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]



mod movegen {
    use super::*;

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

    // bit 18-19: promotion piece type (0 if non promotion)
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
    #[derive(Debug, Clone)]
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
        pub fn white_queenside_castle_rights(&self) -> bool {
            ((self.0 >> 24) & 0b1) != 0
        }
        pub fn white_kingside_castle_rights(&self) -> bool {
            ((self.0 >> 25) & 0b1) != 0
        }
        pub fn black_queenside_castle_rights(&self) -> bool {
            ((self.0 >> 26) & 0b1) != 0
        }
        pub fn black_kingside_castle_rights(&self) -> bool {
            ((self.0 >> 27) & 0b1) != 0
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
        pub fn set_prev_is_ep(&mut self) {
            self.0 |= 1 << 20;
        }
        pub fn set_prev_ep_file(&mut self, file: u8) {
            self.0 |= ((file as u32) & 0b111) << 21;
        }
        pub fn set_white_queenside_castle_rights(&mut self) {
            self.0 |= 1 << 24;
        }
        pub fn set_white_kingside_castle_rights(&mut self) {
            self.0 |= 1 << 25;
        }
        pub fn set_black_queenside_castle_rights(&mut self) {
            self.0 |= 1 << 26;
        }
        pub fn set_black_kingside_castle_rights(&mut self) {
            self.0 |= 1 << 27;
        }
        pub fn set_is_ep(&mut self) {
            self.0 |= 1 << 28;
        }
    }

    impl Board {
        pub fn generate_moves(&self) -> Vec<u32> {
            let mut moves = Vec::new();
            todo!();
            moves
        }
        pub fn generate_sliding_moves(&self, moves: &mut Vec<u32>) {
            todo!();
        }
        pub fn generate_nonsliding_moves(&self, moves: &mut Vec<u32>) {
            todo!();
        }
    }
}

struct Board {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,

    white_pieces: u64,
    black_pieces: u64,

    occupied: u64,
    empty: u64,

    ep: Option<u64>,
    castling: CastlingRights,
    side_to_move: Color,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug)]
struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

#[derive(Clone, Debug)]
enum Color {
    White,
    Black,
}

mod sliding_attacks;
pub use sliding_attacks::{RookTable, BishopTable};

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

fn w_pawn_push_targets(pawns: u64, empty: u64) -> u64 {
    pawns << 8 & empty
}

fn w_pawn_double_push_targets(pawns: u64, empty: u64) -> u64 {
    let single_push = w_pawn_push_targets(pawns, empty);
    single_push << 8 & empty & RANK4
}

fn b_pawn_push_targets(pawns: u64, empty: u64) -> u64 {
    pawns >> 8 & empty
}

fn b_pawn_double_push_targets(pawns: u64, empty: u64) -> u64 {
    let single_push = b_pawn_push_targets(pawns, empty);
    single_push >> 8 & empty & RANK5
}

fn w_pawn_attacks(pawns: u64) -> u64 {
    (pawns << 9) & !FILE_A | (pawns << 7) & !FILE_H
}

fn w_pawn_attacks_target(pawns: u64, targets: u64) -> u64 {
    w_pawn_attacks(pawns) & targets
}

fn b_pawn_attacks(pawns: u64) -> u64 {
    (pawns >> 9) & !FILE_H | (pawns >> 7) & !FILE_A
}

fn b_pawn_attacks_target(pawns: u64, targets: u64) -> u64 {
    b_pawn_attacks(pawns) & targets
}

pub fn pawn_attack_table() -> [[u64; 64]; 2] {
    let mut arr = [[0u64; 64]; 2];
    for i in 0..64 {
        let bitboard = 1u64 << i;
        arr[0][i] = (bitboard << 9) & !FILE_A | (bitboard << 7) & !FILE_H;
        arr[1][i] = (bitboard >> 9) & !FILE_H | (bitboard >> 7) & !FILE_A;
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
