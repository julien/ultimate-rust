use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

pub fn sploosh_speed(c: &mut Criterion) {
    c.bench_function("sploosh", |b| {
        b.iter(|| sploosh(black_box(8), black_box(9), black_box(10)))
    });
}

criterion_group!(benches, sploosh_speed);
criterion_main!(benches);
