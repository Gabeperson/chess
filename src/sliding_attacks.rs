#[derive(Debug, Clone)]
pub struct RookTable {
    pub table: Vec<u64>,
    pub magics: [u64; 64],
    pub masks: [u64; 64],
    pub offsets: [usize; 64],
}

impl RookTable {
    pub fn lookup(&self, square: usize, blockers: u64) -> u64 {
        let magic = self.magics[square];
        let mask = self.masks[square];
        let occ = blockers & mask;
        let n = mask.count_ones();
        let index = (occ.wrapping_mul(magic) >> (64 - n)) as usize;
        self.table[self.offsets[square] + index]
    }
    pub fn new() -> Self {
        rook_table()
    }
}

#[derive(Debug, Clone)]
pub struct BishopTable {
    pub table: Vec<u64>,
    pub magics: [u64; 64],
    pub masks: [u64; 64],
    pub offsets: [usize; 64],
}

impl BishopTable {
    pub fn lookup(&self, square: usize, blockers: u64) -> u64 {
        let magic = self.magics[square];
        let mask = self.masks[square];
        let occ = blockers & mask;
        let n = mask.count_ones();
        let index = (occ.wrapping_mul(magic) >> (64 - n)) as usize;
        self.table[self.offsets[square] + index]
    }
    pub fn new() -> Self {
        bishop_table()
    }
}

fn gen_subsets(mask: u64) -> Vec<u64> {
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

fn gen_empty_slide_rook_inner(square: usize) -> u64 {
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

fn gen_empty_slide_bishop_inner(square: usize) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut attacks = 0;
    // Up Right
    for (r, f) in (rank + 1..7).zip(file + 1..7) {
        attacks |= 1 << (r * 8 + f);
    }
    // Up Left
    for (r, f) in (rank + 1..7).zip(1..file) {
        attacks |= 1 << (r * 8 + f);
    }
    // Down Right
    for (r, f) in (1..rank).rev().zip(file + 1..7) {
        attacks |= 1 << (r * 8 + f);
    }
    // Down Left
    for (r, f) in (1..rank).rev().zip(1..file) {
        attacks |= 1 << (r * 8 + f);
    }
    attacks
}

fn rook_table() -> RookTable {
    let mut table = Vec::new();
    let mut rook_magics = [0u64; 64];
    let mut offsets = [0usize; 64];
    let mut masks = [0u64; 64];
    for (i, it) in rook_magics.iter_mut().enumerate() {
        *it = find_magic(i, PieceType::Rook);
    }
    for square in 0..64 {
        let mask = gen_empty_slide_rook_inner(square);
        let subsets = gen_subsets(mask);
        let n = mask.count_ones();
        let size = 1 << n;

        offsets[square] = table.len();
        masks[square] = mask;
        table.resize(table.len() + size as usize, 0);

        for &occ in subsets.iter() {
            let index = (occ.wrapping_mul(rook_magics[square]) >> (64 - n)) as usize;
            let atk = gen_slide_rook(square, occ);
            table[offsets[square] + index] = atk;
        }
    }
    RookTable {
        table,
        magics: rook_magics,
        masks,
        offsets,
    }
}

fn bishop_table() -> BishopTable {
    let mut table = Vec::new();
    let mut bishop_magics = [0u64; 64];
    let mut offsets = [0usize; 64];
    let mut masks = [0u64; 64];
    for (i, it) in bishop_magics.iter_mut().enumerate() {
        *it = find_magic(i, PieceType::Bishop);
    }
    for square in 0..64 {
        let mask = gen_empty_slide_bishop_inner(square);
        let subsets = gen_subsets(mask);
        let n = mask.count_ones();
        let size = 1 << n;

        offsets[square] = table.len();
        masks[square] = mask;
        table.resize(table.len() + size as usize, 0);

        for &occ in subsets.iter() {
            let index = (occ.wrapping_mul(bishop_magics[square]) >> (64 - n)) as usize;
            let atk = gen_slide_bishop(square, occ);
            table[offsets[square] + index] = atk;
        }
    }
    BishopTable {
        table,
        magics: bishop_magics,
        masks,
        offsets,
    }
}

fn random_u64_few_bits() -> u64 {
    // https://www.chessprogramming.org/index.php?title=Looking_for_Magics&oldid=2272
    fastrand::u64(..) & fastrand::u64(..) & fastrand::u64(..)
}

enum PieceType {
    Bishop,
    Rook,
}

fn find_magic(square: usize, piece_type: PieceType) -> u64 {
    let mask = match piece_type {
        PieceType::Bishop => gen_empty_slide_bishop_inner(square),
        PieceType::Rook => gen_empty_slide_rook_inner(square),
    };
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
            let atk = match piece_type {
                PieceType::Bishop => gen_slide_bishop(square, occ),
                PieceType::Rook => gen_slide_rook(square, occ),
            };
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

fn gen_slide_rook(square: usize, blockers: u64) -> u64 {
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

fn gen_slide_bishop(square: usize, blockers: u64) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut attacks = 0;
    // Up Right
    for (r, f) in (rank + 1..8).zip(file + 1..8) {
        let sq = r * 8 + f;
        attacks |= 1 << sq;
        if (blockers & (1 << sq)) != 0 {
            break;
        }
    }
    // Up Left
    for (r, f) in (rank + 1..8).zip((0..file).rev()) {
        let sq = r * 8 + f;
        attacks |= 1 << sq;
        if (blockers & (1 << sq)) != 0 {
            break;
        }
    }
    // Down Right
    for (r, f) in ((0..rank).rev()).zip(file + 1..8) {
        let sq = r * 8 + f;
        attacks |= 1 << sq;
        if (blockers & (1 << sq)) != 0 {
            break;
        }
    }
    // Down Left
    for (r, f) in ((0..rank).rev()).zip((0..file).rev()) {
        let sq = r * 8 + f;
        attacks |= 1 << sq;
        if (blockers & (1 << sq)) != 0 {
            break;
        }
    }
    attacks
}
