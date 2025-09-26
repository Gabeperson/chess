use std::time::Instant;

use chess::{find_magic_rook, print_bb};

fn main() {
    let now = Instant::now();
    for i in 0..64 {
        let rook_magics = std::hint::black_box(find_magic_rook(std::hint::black_box(i)));
        std::hint::black_box(rook_magics);
    }
    let end = now.elapsed();
    println!("Elapsed: {:.2?}", end);

}
