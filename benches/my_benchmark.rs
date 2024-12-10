use criterion::{black_box, criterion_group, criterion_main, Criterion};
use template_exercisme::fib::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("fib 30", |b| b.iter(|| fibonacci(black_box(30))));
    c.bench_function("fib 40", |b| b.iter(|| fibonacci(black_box(40))));
    c.bench_function("fib 50", |b| b.iter(|| fibonacci(black_box(50))));
    c.bench_function("fib 60", |b| b.iter(|| fibonacci(black_box(60))));
    c.bench_function("fib 70", |b| b.iter(|| fibonacci(black_box(70))));
    c.bench_function("fib 80", |b| b.iter(|| fibonacci(black_box(80))));
    c.bench_function("fib 90", |b| b.iter(|| fibonacci(black_box(90))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
