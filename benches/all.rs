use criterion::{criterion_group, criterion_main, Criterion};
use partial_mixed_markdown::Document;

criterion_main!(benches);
criterion_group!(benches, functions);

fn functions(c: &mut Criterion) {
    let mixed = include_str!("../tests/mixed.md");
    let large = include_str!("../tests/large.md");
    let mut group = c.benchmark_group("small");
    group.bench_function("full", |b| full(b, mixed));
    group.bench_function("without post-processing", |b| {
        without_post_process(b, mixed);
    });
    group.finish();
    let mut group = c.benchmark_group("large");
    group.sample_size(10);
    group.bench_function("full", |b| full(b, large));
    group.bench_function("without post-processing", |b| {
        without_post_process(b, large);
    });
    group.finish();
}

fn full(b: &mut criterion::Bencher, input: &str) {
    b.iter(|| {
        let doc = Document::parse(&input).unwrap();
        let mut out = Vec::new();
        doc.to_writer(&mut out).unwrap();
    });
}

fn without_post_process(b: &mut criterion::Bencher, input: &str) {
    b.iter(|| {
        let doc = Document::parse_without_post_process(&input).unwrap();
        let mut out = Vec::new();
        doc.to_writer(&mut out).unwrap();
    });
}
