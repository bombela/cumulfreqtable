use criterion::{ criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration, };
use cumulfreqtable::*;
use rand::{distributions::Uniform, prelude::*};

macro_rules! bench_tables {
    ($c:ident, $name:expr, $f:expr) => {
        let mut group = $c.benchmark_group($name);
        group.plot_config(PlotConfiguration::default()
            .summary_scale(AxisScale::Logarithmic)
        );
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
                BenchmarkId::new("cumulfreq_array", len),
                &(cumulfreq_array::CumulFreqTable::new(len), rand_pos.clone(), dist_pos),
                $f,
            );
            group.bench_with_input(
                BenchmarkId::new("binary_indexed_tree", len),
                &(binary_indexed_tree::CumulFreqTable::new(len), rand_pos.clone(), dist_pos),
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

fn inc_cumul(c: &mut Criterion) {
    bench_tables!(c, "inc+cumul", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        b.iter(|| {
            table.inc(rand_pos.sample(dist_pos));
            table.sum(rand_pos.sample(dist_pos));
        })
    });
}

fn inc_total(c: &mut Criterion) {
    bench_tables!(c, "inc+total", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        b.iter(|| {
            table.inc(rand_pos.sample(dist_pos));
            table.total()
        })
    });
}

fn inc_cumul_total(c: &mut Criterion) {
    bench_tables!(c, "inc+cumul+total", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        b.iter(|| {
            table.inc(rand_pos.sample(dist_pos));
            (table.sum(rand_pos.sample(dist_pos)), table.total())
        })
    });
}

fn inc_freq(c: &mut Criterion) {
    bench_tables!(c, "inc+freq", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        b.iter(|| {
            table.inc(rand_pos.sample(dist_pos));
            table.freq(rand_pos.sample(dist_pos));
        })
    });
}

fn config() -> Criterion {
    use std::time::Duration;
    Criterion::default()
        .warm_up_time(Duration::from_millis(50))
        .measurement_time(Duration::from_millis(500))
}

criterion_group!(name = benches; config = config();
    targets = inc, inc_cumul, inc_total, inc_cumul_total, inc_freq
);
criterion_main!(benches);
