use criterion::{black_box, criterion_group, criterion_main, Criterion};
use for_let::for_let;

fn primes_iter(n: usize) -> usize {
    (0..n).filter(|x| x % 2 == 1).sum::<usize>()
}

fn primes_for_let(n: usize) -> usize {
    let mut sum = 0usize;
    for_let!(x if x % 2 == 1 in 0..n {
        sum += x;
    });
    sum
}

fn count_foos_iter(foos: &[Option<&str>]) -> usize {
    foos.iter().filter(|foo| matches!(foo, Some("foo"))).count()
}

fn count_foos_for_let(foos: &[Option<&str>]) -> usize {
    let mut count = 0usize;
    for_let!(Some("foo") in foos {
        count += 1;
    });
    count
}

fn runner(c: &mut Criterion) {
    let foos_50 = black_box(vec![None::<&str>; 50]);
    let foos_200 = black_box(vec![None::<&str>; 200]);

    c.benchmark_group("50")
        .bench_function("primes_iter", |b| b.iter(|| primes_iter(black_box(50))))
        .bench_function("primes_for_let", |b| {
            b.iter(|| primes_for_let(black_box(50)))
        })
        .bench_function("count_foos_iter", |b| b.iter(|| count_foos_iter(&foos_50)))
        .bench_function("count_foos_for_let", |b| {
            b.iter(|| count_foos_for_let(&foos_50))
        });

    c.benchmark_group("200")
        .bench_function("primes_iter", |b| b.iter(|| primes_iter(black_box(200))))
        .bench_function("primes_for_let", |b| {
            b.iter(|| primes_for_let(black_box(200)))
        })
        .bench_function("count_foos_iter", |b| b.iter(|| count_foos_iter(&foos_200)))
        .bench_function("count_foos_for_let", |b| {
            b.iter(|| count_foos_for_let(&foos_200))
        });
}

criterion_group!(benchmarks, runner);
criterion_main!(benchmarks);
