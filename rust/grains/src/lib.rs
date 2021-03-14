pub fn square(s: u32) -> u64 {
    assert!(s >= 1 && s <= 64, "Square must be between 1 and 64");

    u64::pow(2, s - 1)
}

pub fn total() -> u64 {
    // cheat way
    // u64::MAX
    square(63) + square(64)
}
