#[macro_use]
extern crate criterion;

use criterion::Criterion;

use rust_trie::trie::Trie;

fn create_trie() -> Trie {
    let mut trie = Trie::new();
    trie.add("sdfjaasdkhsdfhasdkfjsdjfhskdfjhskdfhskdf");
    trie.contains("sdfjaasdkhsdfhasdkfjsdjfhskdfjhskdfhskdf");

    trie
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create trie", |b| b.iter(create_trie));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
