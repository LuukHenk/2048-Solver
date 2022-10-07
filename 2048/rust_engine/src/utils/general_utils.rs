pub fn pow_unsafe(exponent: u64) -> u64 {
    if exponent < 1 {
        return 0_u64;
    }
    let mut result: u64 = 2;
    for _ in 1..exponent {
        result *= 2
    }
    result
}
