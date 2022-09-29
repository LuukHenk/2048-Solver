
pub struct Utils;

impl Utils {
    pub fn freaking_pow_is_not_possible_with_an_u64(exponent: u64) -> u64 {
        if exponent < 1 {return 0_u64}
        let mut real_tile_value: u64 = 2;
        for _ in 1 .. exponent {
            real_tile_value *= 2
        }
        real_tile_value
    }
}