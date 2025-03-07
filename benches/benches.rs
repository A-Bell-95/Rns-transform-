use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rns_rs::convert::{from_rns, to_rns};
use rns_rs::modulus::find_suitable_modulus;

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
        b.iter(|| black_box(to_rns(num)))
    });
}

fn bench_from_rns(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_rns".to_string());
    group.sample_size(10);

    do_bench_from_rns(
        &mut group,
        &[2, 0, 2, 6, 8, 2, 3, 9, 12, 9, 5, 39, 12, 18, 18, 44, 59, 42],
    );
    do_bench_from_rns(
        &mut group,
        &[
            2, 0, 5, 1, 11, 2, 10, 5, 15, 23, 15, 29, 19, 1, 15, 34, 24, 14,
        ],
    );
}

fn do_bench_from_rns(c: &mut BenchmarkGroup<WallTime>, num: &[u8]) {
    c.bench_function(format!("from_rns({num:?})"), |b| {
        b.iter(|| black_box(from_rns(num)))
    });
}

criterion_group!(
    modulus,
    bench_find_suitable_modulus,
    bench_to_rns,
    bench_from_rns
);
criterion_main!(modulus);
