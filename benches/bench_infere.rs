use chordparser::inference::descriptors_from_midi_codes;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn infere(midi_codes: &[u8]) -> Vec<String> {
    descriptors_from_midi_codes(midi_codes)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench infere", |b| {
        b.iter(|| {
            let res = infere(black_box(&[16, 55, 72, 93]));
            let _ = black_box(res);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
