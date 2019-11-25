use criterion::*;
use simdlog::apache::{Stage2, Structurals};

use jemallocator::Jemalloc;
use std::fs::File;
use std::io::Read;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn bench(c: &mut Criterion) {
    let mut buf = String::new();
    File::open("apache_logs.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let lines: Vec<&str> = buf.lines().collect();

    let mut group = c.benchmark_group("apache");
    group.throughput(Throughput::Bytes(buf.len() as u64));
    group.bench_function("random/find_structurals", |b| {
        b.iter(|| {
            for line in &lines {
                Structurals::new(line.as_bytes()).find();
            }
        })
    });
    group.bench_function("random/parse", |b| {
        b.iter(|| {
            for line in &lines {
                Stage2::new(line.as_bytes()).parse();
            }
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
