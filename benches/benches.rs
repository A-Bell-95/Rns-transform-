mod ops;
use ops::ops;

use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rns_rs::modulus::find_suitable_modulus;
use rns_rs::Rns;

fn bench_find_suitable_modulus(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_suitable_modulus".to_string());
    group.sample_size(10);

    do_bench_find_suitable_modulus(&mut group, 256);
    do_bench_find_suitable_modulus(&mut group, 512);
    do_bench_find_suitable_modulus(&mut group, 1024);
    do_bench_find_suitable_modulus(&mut group, 12345);
}

fn do_bench_find_suitable_modulus(c: &mut BenchmarkGroup<WallTime>, num: u64) {
    c.bench_function(format!("find_suitable_modulus({num})"), |b| {
        b.iter(|| black_box(find_suitable_modulus(num)))
    });
}

fn bench_to_rns(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_rns".to_string());
    group.sample_size(10);

    do_bench_to_rns(&mut group, 32);
    do_bench_to_rns(&mut group, 15386);
    do_bench_to_rns(&mut group, 163879197);
    do_bench_to_rns(&mut group, 360287981029);
}

fn do_bench_to_rns(c: &mut BenchmarkGroup<WallTime>, num: u64) {
    c.bench_function(format!("to_rns({num})"), |b| {
        b.iter(|| black_box(Rns::<18>::new(num)))
    });
}

fn bench_from_rns(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_rns".to_string());
    group.sample_size(10);

    let rns = Rns::<18>::new(5000);
    do_bench_from_rns(&mut group, rns);

    let rns = Rns::<18>::new(69000123);
    do_bench_from_rns(&mut group, rns);
}

fn do_bench_from_rns(c: &mut BenchmarkGroup<WallTime>, rns: Rns<18>) {
    c.bench_function(format!("from_rns({rns:?})"), |b| {
        b.iter(|| black_box(rns.try_into_u64()))
    });
}

criterion_group!(
    modulus,
    bench_find_suitable_modulus,
    bench_to_rns,
    bench_from_rns
);
criterion_main!(modulus, ops);
