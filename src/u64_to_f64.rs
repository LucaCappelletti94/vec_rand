

#[inline(always)]
/// Convet a u64 to a f64 between 0 and 1. This method only use the higher 53-bits.
/// method proposed by vigna on http://prng.di.unimi.it/ 
pub fn u64_to_f64_mul(x: u64) -> f64{
    (x >> 11) as f64 * 2.0f64.powf(-53.0)
}


#[inline(always)]
/// Convet a u64 to a f64 between 0 and 1. This method only use the higher 53-bits.
/// https://experilous.com/1/blog/post/perfect-fast-random-floating-point-numbers
/// http://prng.di.unimi.it/xoshiro256plus.c
pub fn u64_to_f64_no_mul(x: u64) -> f64{
    let v: u64 = (x >> 11) | (1023 << 52);
    let r: f64 = f64::from_le_bytes(v.to_le_bytes());
    r - 1f64
}
