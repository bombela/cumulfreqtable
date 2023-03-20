use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use cumulfreqtable::*;
use rand::{distributions::Uniform, prelude::*};

macro_rules! bench_tables {
    ($c:ident, $name:expr, $f:expr) => {
        let mut group = $c.benchmark_group($name);
        let rand_pos = StdRng::from_entropy();
        for i in (2..=16).step_by(2) {
            let len = 1 << i;
            group.throughput(criterion::Throughput::Elements(len as u64));
            let dist_pos = Uniform::from(0..len);
            group.bench_with_input(
                BenchmarkId::new("freq_array", len),
                &(freq_array::FreqTable::new(len), rand_pos.clone(), dist_pos),
                $f,
            );
            group.bench_with_input(
                BenchmarkId::new("freq_array", len),
                &(
                    cumulfreq_array::CumulFreqTable::new(len),
                    rand_pos.clone(),
                    dist_pos,
                ),
                $f,
            );
            group.bench_with_input(
                BenchmarkId::new("freq_array", len),
                &(
                    binary_indexed_tree::CumulFreqTable::new(len),
                    rand_pos.clone(),
                    dist_pos,
                ),
                $f,
            );
        }
    };
}

fn inc(c: &mut Criterion) {
    bench_tables!(c, "inc", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        b.iter(|| {
            table.inc(rand_pos.sample(dist_pos));
        })
    });
}

fn getcumul(c: &mut Criterion) {
    bench_tables!(c, "getcumul", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        for _ in 0..table.len() {
            table.inc(rand_pos.sample(dist_pos));
        }
        b.iter(|| {
            table.cumfreq(rand_pos.sample(dist_pos));
        })
    });
}

fn config() -> Criterion {
    use std::time::Duration;
    Criterion::default().warm_up_time(Duration::from_secs(1))
}

criterion_group!(name = benches; config = config(); targets = inc, getcumul);
criterion_main!(benches);
