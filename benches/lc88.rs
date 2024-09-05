use leet_code::lc88;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn million_bm(c: &mut Criterion) {
    let mut n1 = vec![0; 2_000_000];
    let mut n2 = vec![1; 1_000_000];
    c.bench_function("the million",
                     |b| b.iter(||
                                lc88::Solution::merge(&mut n1, 1_000_000, &mut n2, 1_000_000)));
}

pub fn leet_1_bm(c: &mut Criterion) {
    let mut n1 = vec![1, 2, 3, 0, 0, 0];
    let mut n2 = vec![4, 5, 6];
    c.bench_function("leet 1",
                     |b| b.iter(||
                                lc88::Solution::merge(&mut n1, 3, &mut n2, 3)));
}

criterion_group!(benches, million_bm, leet_1_bm);
criterion_main!(benches);
