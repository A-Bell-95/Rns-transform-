use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{black_box, criterion_group, Criterion};

use rns_rs::Rns;

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add".to_string());
    group.sample_size(10);

    let a = Rns::<18>::new(976010000);
    let b = Rns::<18>::new(120067);
    do_bench_add(&mut group, a, b);
}

fn do_bench_add(c: &mut BenchmarkGroup<WallTime>, a: Rns<18>, b: Rns<18>) {
    c.bench_function(format!("{} + {}", a.into_u64(), b.into_u64()), |bencher| {
        bencher.iter(|| black_box(a - b))
    });
}

fn bench_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub".to_string());
    group.sample_size(10);

    let a = Rns::<18>::new(976010000);
    let b = Rns::<18>::new(120067);
    do_bench_sub(&mut group, a, b);
}

fn do_bench_sub(c: &mut BenchmarkGroup<WallTime>, a: Rns<18>, b: Rns<18>) {
    c.bench_function(format!("{} - {}", a.into_u64(), b.into_u64()), |bencher| {
        bencher.iter(|| black_box(a - b))
    });
}

criterion_group!(ops, bench_add, bench_sub);
