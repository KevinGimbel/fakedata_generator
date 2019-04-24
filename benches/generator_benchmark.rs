#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate generator;

use generator::*;

fn corpora_benchmark(c: &mut Criterion) {
    c.bench_function("gen: username", |b|b.iter(|| gen_username()));
    c.bench_function("gen: domain", |b|b.iter(|| gen_domain()));
    c.bench_function("gen: email", |b|b.iter(|| gen_email()));
    c.bench_function("gen: enum", |b|b.iter(|| gen_enum("hello,hola,hallo".to_string())));
    c.bench_function("gen: http.method", |b|b.iter(|| gen_http_method()));
    c.bench_function("gen: int", |b|b.iter(|| gen_int("10,10000".to_string())));
    c.bench_function("gen: ipv4", |b|b.iter(|| gen_ipv4()));
}

criterion_group!(benches, corpora_benchmark);
criterion_main!(benches);