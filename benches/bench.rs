use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use cumulfreqtable::*;
use rand::{distributions::Uniform, prelude::*};
use std::convert::From;

macro_rules! group_bench_all_tables {
    ($range:expr, $group:ident, $f:expr, $t:ty) => {
        let rand_pos = StdRng::from_entropy();
        for i in $range {
            let len = 1 << i;
            $group.throughput(criterion::Throughput::Elements(len as u64));
            let dist_pos = Uniform::from(0..len);
            $group.bench_with_input(
                BenchmarkId::new("freq_array", len),
                &(
                    freq_array::FreqTable::<$t>::new(len),
                    rand_pos.clone(),
                    dist_pos,
                ),
                $f,
            );
            $group.bench_with_input(
                BenchmarkId::new("cumulfreq_array", len),
                &(
                    cumulfreq_array::CumulFreqTable::<$t>::new(len),
                    rand_pos.clone(),
                    dist_pos,
                ),
                $f,
            );
            $group.bench_with_input(
                BenchmarkId::new("binary_indexed_tree", len),
                &(
                    binary_indexed_tree::CumulFreqTable::<$t>::new(len),
                    rand_pos.clone(),
                    dist_pos,
                ),
                $f,
            );
        }
    };
}

macro_rules! bench_all_tables {
    ($range:expr, $c:expr, $name:expr, $f:expr, $t:ty) => {{
        let mut group = $c.benchmark_group(concat!($name, "_", stringify!($t)));
        group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        group_bench_all_tables!($range, group, $f, $t);
    }};
    ($range:expr, $c:ident, $name:expr, $f:expr) => {
        bench_all_tables!($range, $c, $name, $f, usize);
    };
    ($c:ident, $name:expr, $f:expr) => {
        bench_all_tables!((2..=16).step_by(2), $c, $name, $f);
    };
}

fn inc(c: &mut Criterion) {
    bench_all_tables!(c, "inc", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        b.iter(|| {
            table.inc(rand_pos.sample(dist_pos));
        })
    });
}

fn inc_cumul(c: &mut Criterion) {
    bench_all_tables!(c, "inc+cumul", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        b.iter(|| {
            table.inc(rand_pos.sample(dist_pos));
            table.sum(rand_pos.sample(dist_pos));
        })
    });
}

fn inc_total(c: &mut Criterion) {
    bench_all_tables!(c, "inc+total", |b, input| {
        let (mut table, mut rand_pos, dist_pos) = input.clone();
        b.iter(|| {
            table.inc(rand_pos.sample(dist_pos));
            table.total()
        })
    });
}

fn inc_cumul_total(c: &mut Criterion) {
    bench_all_tables!(
        (2..=16).step_by(1),
        c,
        "inc+cumul+total",
        |b, input| {
            let (mut table, mut rand_pos, dist_pos) = input.clone();
            b.iter(|| {
                table.inc(rand_pos.sample(dist_pos));
                (table.sum(rand_pos.sample(dist_pos)), table.total())
            })
        },
        usize
    );
    bench_all_tables!(
        (2..=10).step_by(1),
        c,
        "inc+cumul+total",
        |b, input| {
            let (mut table, mut rand_pos, dist_pos) = input.clone();
            b.iter(|| {
                table.inc(rand_pos.sample(dist_pos));
                (table.sum(rand_pos.sample(dist_pos)), table.total())
            })
        },
        u32
    );
    bench_all_tables!(
        (2..=10).step_by(1),
        &mut *c,
        "inc+cumul+total",
        |b, input| {
            let (mut table, mut rand_pos, dist_pos) = input.clone();
            b.iter(|| {
                table.inc(rand_pos.sample(dist_pos));
                (table.sum(rand_pos.sample(dist_pos)), table.total())
            })
        },
        u16
    );
}

fn inc_freq(c: &mut Criterion) {
    bench_all_tables!(c, "inc+freq", |b, input| {
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
    targets = inc, inc_cumul, inc_total, inc_cumul_total, inc_freq,
);
criterion_main!(benches);
