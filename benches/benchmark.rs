use criterion::async_executor::AsyncStdExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use std::convert::TryInto;
use yeast_rs::{async_std::yeast as async_std_yeast, tokio::yeast as tokio_yeast, yeast, Yeast};

pub fn bench_yeast(c: &mut Criterion) {
    c.bench_function("yeast", |b| {
        b.iter(|| {
            let _: Yeast = yeast().to_string().try_into().unwrap();
        })
    });
}

pub fn bench_async_std_yeast(c: &mut Criterion) {
    c.bench_function("async_std_yeast", |b| {
        b.to_async(AsyncStdExecutor).iter(|| async {
            let _: Yeast = async_std_yeast().await.to_string().try_into().unwrap();
        })
    });
}
pub fn bench_tokio_yeast(c: &mut Criterion) {
    c.bench_function("tokio_yeast", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let _: Yeast = tokio_yeast().await.to_string().try_into().unwrap();
            })
    });
}

criterion_group!(
    benches,
    bench_async_std_yeast,
    bench_yeast,
    bench_tokio_yeast
);
criterion_main!(benches);
