pub fn square(s: u32) -> u64 {
    1 << (s - 1)
}

pub fn total() -> u64 {
    (1_u64..=64).fold(0, |acc, x| acc + square(x as u32))
}
