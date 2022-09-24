use criterion::{black_box, criterion_group, criterion_main, Criterion};

use train_helper_lib;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("skm", |b| {
        b.iter(|| {
            let messages = train_helper_lib::get_messages();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
