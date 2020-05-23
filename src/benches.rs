use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fmt;
use std::time::Duration;
use xactor_benchmarks::{actix_test, xactor_test, Result as BenchResult, Spec};

criterion_group!(benches, bench);
criterion_main!(benches);

fn bench(c: &mut Criterion) {
    let tests = gen_tests();
    let mut group = c.benchmark_group("actor_tests");

    for (i, spec) in tests.into_iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("actix", i), &spec, |b, spec| {
            b.iter(|| actix_test::run(spec))
        });

        group.bench_with_input(BenchmarkId::new("xactor", i), &spec, |b, spec| {
            // See https://github.com/async-rs/async-std/issues/770#issuecomment-633011171
            b.iter(|| smol::run(async { xactor_test::run(spec).await }))
        });
    }
    group.finish();
}

// Generate the benchmark specifications
fn gen_tests() -> Vec<Spec> {
    let max = num_cpus::get() as u32;
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
