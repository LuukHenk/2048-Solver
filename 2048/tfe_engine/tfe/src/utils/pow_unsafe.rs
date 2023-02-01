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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_unsafe_with_zero_exponent() {
        assert_eq!(pow_unsafe(0), 0);
    }

    #[test]
    #[should_panic(expected="attempt to multiply with overflow")]
    fn test_pow_unsafe_with_too_large_exponent() {
        assert_eq!(pow_unsafe(1000000000), 0);
    }

    #[test]
    fn test_pow_unsafe() {
        let exponent = 2;
        assert_eq!(pow_unsafe(exponent), 4);
    }
}