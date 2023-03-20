use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use cumfreq::*;
use rand::{distributions::Uniform, prelude::*};

//pub fn criterion_benchmark(c: &mut Criterion) {
//let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//c.bench_function("cumfreq", |b| b.iter(|| cumfreq(black_box(&mut x))));
//}

pub fn add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");
    let rand_pos = StdRng::from_entropy();
    let rand_freq = StdRng::from_entropy();
    let dist_freq = Uniform::from(1..1 << 27);
    for i in 2..=16 {
        let len = 1 << i;
        group.throughput(criterion::Throughput::Elements(len as u64));
        let dist_pos = Uniform::from(0..len);
        let input = (
            rand_pos.clone(),
            rand_freq.clone(),
            dist_freq.clone(),
            dist_pos,
            len,
        );
        group.bench_with_input(
            BenchmarkId::new("freq_table", len),
            &input,
            bench_add::<freq_table::FreqTable>,
        );
        group.bench_with_input(
            BenchmarkId::new("cumfreq_table", len),
            &input,
            bench_add::<cumfreq_table::CumFreqTable>,
        );
        group.bench_with_input(
            BenchmarkId::new("binary_indexed_tree", len),
            &input,
            bench_add::<binary_indexed_tree::CumFreqTable>,
        );
    }
}

fn bench_add<Table: CumFreqTable>(
    b: &mut Bencher,
    input: &(
        rand::prelude::StdRng,
        rand::prelude::StdRng,
        Uniform<usize>,
        Uniform<usize>,
        usize,
    ),
) {
    let (mut rand_pos, mut rand_freq, dist_freq, dist_pos, len) = input.clone();
    let mut table = Table::new(len);
    b.iter(|| {
        table.add(rand_pos.sample(dist_pos), rand_freq.sample(dist_freq));
    })
}

criterion_group!(benches, add);
criterion_main!(benches);
