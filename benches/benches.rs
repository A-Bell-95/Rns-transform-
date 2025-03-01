use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

criterion_group!(modulus, bench_find_suitable_modulus);
criterion_main!(modulus);
