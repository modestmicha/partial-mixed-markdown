use criterion::{criterion_group, criterion_main, Criterion};
use partial_mixed_markdown::Document;

criterion_main!(benches);
criterion_group!(benches, functions);

fn functions(c: &mut Criterion) {
    c.bench_function("full", full);
    c.bench_function("without post-processing", without_post_process);
}

fn full(b: &mut criterion::Bencher) {
    let input = include_str!("../tests/mixed.md");
    b.iter(|| {
        let doc = Document::parse(&input).unwrap();
        let mut out = Vec::new();
        doc.to_writer(&mut out).unwrap();
    });
}

fn without_post_process(b: &mut criterion::Bencher) {
    let input = include_str!("../tests/mixed.md");
    b.iter(|| {
        let doc = Document::parse_without_post_process(&input).unwrap();
        let mut out = Vec::new();
        doc.to_writer(&mut out).unwrap();
    });
}
