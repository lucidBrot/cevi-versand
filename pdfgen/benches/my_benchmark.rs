#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use pdfgen;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple sample", |b| b.iter(pdfgen::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
