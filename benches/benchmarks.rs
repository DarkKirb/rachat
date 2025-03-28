#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

use criterion::{Criterion, criterion_group, criterion_main};
use rachat::utils::id_generator;

fn test_id_generator(c: &mut Criterion) {
    c.bench_function("id generator", |b| b.iter(id_generator::generate));
}

criterion_group!(benches, test_id_generator);
criterion_main!(benches);
