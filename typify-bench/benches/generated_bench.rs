use criterion::{criterion_group, criterion_main};

mod regex_long {
    include!(concat!(env!("OUT_DIR"), "/regex_long.rs"));

    pub fn bench(c: &mut criterion::Criterion) {
        c.bench_function("long regex", |b| b.iter(|| {
            let _result: TypifyValidationBenchmark = serde_json::from_str("{\"name\": \"hello_world\"}").unwrap();
        }));
    }
}

mod regex_short {
    include!(concat!(env!("OUT_DIR"), "/regex_short.rs"));

    pub fn bench(c: &mut criterion::Criterion) {
        c.bench_function("short regex", |b| b.iter(|| {
            let _result: IdOrName = "hello-worlD".parse().unwrap();
        }));
    }
}

criterion_group!(benches, regex_long::bench, regex_short::bench);
criterion_main!(benches);