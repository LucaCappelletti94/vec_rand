#![feature(test)]
extern crate test;
use test::Bencher;

use vec_rand::cumsum_f64::*;

mod utils;
use utils::*;

const NUMBER: u64 = 10000;

#[bench]
fn test_cumsum_f64(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        cumsum_f64(&random_vec)
    });
}

#[bench]
fn test_cumsum_f64_scan(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        cumsum_f64_scan(&random_vec)
    });
}

#[bench]
fn test_cumsum_f64_sse_intrinsics(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        cumsum_f64_sse_intrinsics(&random_vec)
    });
}

#[bench]
fn test_cumsum_f64_avx_intrinsics(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        cumsum_f64_avx_intrinsics(&random_vec)
    });
}

#[bench]
fn test_cumsum_f64_unrolled(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        cumsum_f64_unrolled(&random_vec)
    });
}
