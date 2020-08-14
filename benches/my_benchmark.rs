#[macro_use]
extern crate criterion;

use criterion::Criterion;

pub fn test_iter() -> Vec<usize> {
    (0..12)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect()
}

pub fn test_for() -> Vec<usize> {
    let n = 12;
    let mut result = Vec::with_capacity(n);
    for x in 0..n {
        if x % 2 == 0 {
            result.push(x * x);
        }
    }
    result
}

fn bench_iter(c: &mut Criterion) {
    c.bench_function("iter", |b| b.iter(|| test_iter()));
}

fn bench_for(c: &mut Criterion) {
		assert_eq!(test_iter(), test_for());
    c.bench_function("for", |b| b.iter(|| test_for()));
}

criterion_group!(benches, bench_iter, bench_for);
criterion_main!(benches);
