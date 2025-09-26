#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::vec;

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

    ep: Option<Rank>,
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

#[derive(Clone, Debug)]
enum Rank {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

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

// Don't know exactly how this works but it does
pub fn gen_subsets(mask: u64) -> Vec<u64> {
    let mut subsets = Vec::new();
    let mut subset = mask;
    loop {
        subsets.push(subset);
        if subset == 0 {
            break;
        }
        subset = (subset - 1) & mask;
    }
    subsets
}

pub fn gen_empty_slide_rook_inner(square: usize) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut attacks = 0;
    for r in 1..7 {
        if r == rank {
            continue;
        }
        attacks |= 1 << (r * 8 + file);
    }
    for f in 1..7 {
        if f == file {
            continue;
        }
        attacks |= 1 << (rank * 8 + f);
    }
    attacks
}

pub fn random_u64_few_bits() -> u64 {
    // https://www.chessprogramming.org/index.php?title=Looking_for_Magics&oldid=2272
    fastrand::u64(..) & fastrand::u64(..) & fastrand::u64(..)
}

pub fn find_magic_rook(square: usize) -> u64 {
    let mask = gen_empty_slide_rook_inner(square);
    let subsets = gen_subsets(mask);
    let n = mask.count_ones();
    let mut used = vec![0u64; 1 << n];
    loop {
        let magic = random_u64_few_bits();
        if (mask.wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones() < 6 {
            // https://www.chessprogramming.org/index.php?title=Looking_for_Magics&oldid=2272
            continue;
        }
        for i in used.iter_mut() {
            *i = 0;
        }
        let mut success = true;
        for &occ in subsets.iter() {
            let index = (occ.wrapping_mul(magic) >> (64 - n)) as usize;
            let atk = gen_slide_rook(square, occ);
            if used[index] == 0 {
                used[index] = atk;
            } else if used[index] != atk {
                success = false;
                break;
            }
        }
        if success {
            return magic;
        }
    }
}

pub fn gen_slide_rook(square: usize, blockers: u64) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut attacks = 0;
    // Up
    for r in rank + 1..8 {
        let sq = r * 8 + file;
        attacks |= 1 << sq;
        if (blockers & (1 << sq)) != 0 {
            break;
        }
    }
    // Down
    for r in (0..rank).rev() {
        let sq = r * 8 + file;
        attacks |= 1 << sq;
        if (blockers & (1 << sq)) != 0 {
            break;
        }
    }
    // Right
    for f in file + 1..8 {
        let sq = rank * 8 + f;
        attacks |= 1 << sq;
        if (blockers & (1 << sq)) != 0 {
            break;
        }
    }
    // Left
    for f in (0..file).rev() {
        let sq = rank * 8 + f;
        attacks |= 1 << sq;
        if (blockers & (1 << sq)) != 0 {
            break;
        }
    }
    attacks
}



// pub fn gen_empty_slide_rook(square: usize) -> u64 {
//     let rank = square / 8;
//     let file = square % 8;
//     let mut attacks = 0u64;
//     for r in rank + 1..8 {
//         attacks |= 1u64 << (r * 8 + file);
//     }
//     for r in (0..rank).rev() {
//         attacks |= 1u64 << (r * 8 + file);
//     }
//     for f in file + 1..8 {
//         attacks |= 1u64 << (rank * 8 + f);
//     }
//     for f in (0..file).rev() {
//         attacks |= 1u64 << (rank * 8 + f);
//     }
//     attacks
// }


pub fn print_bb(bitboard: u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;
            if (bitboard >> square) & 1 == 1 {
                print!("\x1b[32m1 \x1b[0m");
            } else {
                print!(" 0");
            }
        }
        println!();
    }
    println!();
}
