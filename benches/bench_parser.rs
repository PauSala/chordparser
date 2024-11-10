use chordparser::parsing::Parser;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn parse(n: &str, parser: &mut Parser) {
    let result = parser.parse(&n);
    result.unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut parser = Parser::new();
    c.bench_function("C", |b| {
        b.iter(|| parse(black_box("C#+Maj76omit59#11"), black_box(&mut parser)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
