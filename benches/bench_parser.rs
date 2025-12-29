use chordparser::{
    chord::Chord,
    parsing::{Parser, parser_error::ParserErrors},
};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn parse(n: &str, parser: &mut Parser) -> Result<Chord, ParserErrors> {
    parser.parse(&n)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut parser = Parser::new();
    c.bench_function("C", |b| {
        b.iter(|| {
            let chord = parse(black_box("CMaj7#9#11b6Omit5"), black_box(&mut parser)).unwrap();
            let _ = black_box(chord);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
