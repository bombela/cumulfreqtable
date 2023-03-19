use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cumfreq::fibonacci;

//pub fn criterion_benchmark(c: &mut Criterion) {
    //let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //c.bench_function("cumfreq", |b| b.iter(|| cumfreq(black_box(&mut x))));
//}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
