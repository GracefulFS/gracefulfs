//! Criterion benchmarks for end-to-end scanning.

use std::{hint::black_box, path::Path};

use criterion::{Criterion, criterion_group, criterion_main};

fn scan(criterion: &mut Criterion) {
    // Currently measures the unsupported error path, not filesystem traversal.
    criterion.bench_function("scan", |b| {
        b.iter(|| gracefulfs::scan(black_box(Path::new("."))))
    });
}

criterion_group!(benches, scan);
criterion_main!(benches);
