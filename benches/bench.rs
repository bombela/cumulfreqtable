use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use cumulfreqtable::*;
use rand::{distributions::Uniform, prelude::*};

//pub fn criterion_benchmark(c: &mut Criterion) {
//let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//c.bench_function("cumulfreq", |b| b.iter(|| cumulfreq(black_box(&mut x))));
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
            BenchmarkId::new("freq_array", len),
            &input,
            bench_add::<freq_array::FreqTable>,
        );
        group.bench_with_input(
            BenchmarkId::new("cumulfreq_array", len),
            &input,
            bench_add::<cumulfreq_array::CumulFreqTable>,
        );
        group.bench_with_input(
            BenchmarkId::new("binary_indexed_tree", len),
            &input,
            bench_add::<binary_indexed_tree::CumulFreqTable>,
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
