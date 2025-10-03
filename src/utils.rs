pub fn lsb1(n: u64) -> usize {
    n.trailing_zeros() as usize
}
