use criterion::{Criterion, criterion_group, criterion_main};
use dispocheck::{is_disposable_domain, is_disposable_email};
use std::hint::black_box;

fn bench_lookup(c: &mut Criterion) {
    c.bench_function("is_disposable_email (hit)", |b| {
        b.iter(|| is_disposable_email(black_box("someone@mailinator.com")))
    });

    c.bench_function("is_disposable_email (miss)", |b| {
        b.iter(|| is_disposable_email(black_box("someone@example.com")))
    });

    c.bench_function("is_disposable_domain (deep subdomain hit)", |b| {
        b.iter(|| is_disposable_domain(black_box("a.b.c.d.mailinator.com")))
    });
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
