use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{fmt, time::Duration};
use xactor_benchmarks::{actix_test, xactor_test, Result as BenchResult, Spec};

criterion_group!(benches, bench_actix, bench_xactor);
criterion_main!(benches);

fn bench_actix(c: &mut Criterion) {
    let tests = gen_tests(Some(2));

    let mut group = c.benchmark_group("actix");
    for spec in tests.into_iter() {
        group.bench_with_input(BenchmarkId::from_parameter(&spec), &spec, |b, spec| {
            b.iter(|| actix_test::run(black_box(spec)))
        });
    }
    group.finish();
}

fn bench_xactor(c: &mut Criterion) {
    let tests = gen_tests(Some(2));

    let mut group = c.benchmark_group("xactor");
    for spec in tests.into_iter() {
        group.bench_with_input(BenchmarkId::from_parameter(&spec), &spec, |b, spec| {
            // See https://github.com/async-rs/async-std/issues/770#issuecomment-633011171
            b.iter(|| smol::run(async { xactor_test::run(black_box(spec)).await }))
        });
    }
    group.finish();
}

// Generate the benchmark specifications
fn gen_tests(max: Option<u32>) -> Vec<Spec> {
    let max = max.unwrap_or(num_cpus::get() as u32 + 1);
    let mut v = Vec::new();
    for procs in 1..max {
        for msgs in 1..max {
            for parallel in 0..(max + 1) {
                for size in 1..(max + 1) {
                    v.push(Spec {
                        procs: 10_u32.pow(procs),
                        messages: 10_u32.pow(msgs),
                        parallel: 2_u32.pow(parallel),
                        size: 10_u32.pow(size),
                    })
                }
            }
        }
    }
    v
}
