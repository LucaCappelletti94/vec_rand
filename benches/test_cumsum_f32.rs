#![feature(test)]
extern crate test;
use test::Bencher;

use vec_rand::cumsum_f32::*;

mod utils;
use utils::*;

const NUMBER: u64 = 10000;

#[bench]
fn test_cumsum_f32(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);
    b.iter(|| {
        cumsum_f32(&random_vec)
    });
}

#[bench]
fn test_cumsum_f32_scan(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);
    b.iter(|| {
        cumsum_f32_scan(&random_vec)
    });
}

#[bench]
fn test_cumsum_f32_sse_intrinsics(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);
    b.iter(|| {
        cumsum_f32_sse_intrinsics(&random_vec)
    });
}

#[bench]
fn test_cumsum_f32_unrolled(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);
    b.iter(|| {
        cumsum_f32_unrolled(&random_vec)
    });
}
