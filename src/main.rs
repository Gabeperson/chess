use std::time::Instant;

use chess::{print_bb, BishopTable, RookTable};

fn main() {
    let now = Instant::now();
    let table = BishopTable::new();
    let elapsed = now.elapsed();
    println!("Rook attack table generated in: {:.2?}", elapsed);
    dbg!(table.table.len());

    let rook = 24;

    let blockers = &[5, 9, 10, 4, 12];

    let mut blockers_bb = 0;
    for b in blockers {
        blockers_bb |= 1u64 << b;
    }
    let attacks = table.lookup(rook, blockers_bb);

    println!("Rook:");
    print_bb(1u64 << rook);
    println!("Blockers:");
    print_bb(blockers_bb);
    println!("Attacks:");
    print_bb(attacks);
}
