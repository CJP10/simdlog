use criterion::*;
use jemallocator::Jemalloc;
use simdlog::apache::access::avx2::{Stage2, Stage1};
use simdlog::apache::access::parse;
use std::fs::File;
use std::io::Read;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn bench(c: &mut Criterion) {
    let mut buf = String::new();
    File::open("samples/apache_common.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let common_lines: Vec<&str> = buf.lines().collect();

    let mut common = c.benchmark_group("apache/common");
    common.throughput(Throughput::Bytes(buf.len() as u64));
    common.bench_function("stage1", |b| {
        b.iter(|| {
            for line in &common_lines {
                Stage1::new(line.as_bytes()).find();
            }
        })
    });
    common.bench_function("stage2", |b| {
        let common_lines: Vec<(&str, Vec<u32>)> = common_lines
            .iter()
            .map(|s| (*s, Stage1::new(s.as_bytes()).find()))
            .collect();
        b.iter(|| {
            for (line, structurals) in &common_lines {
                Stage2::new_with_structurals(line.as_bytes(), structurals.clone());
            }
        })
    });
    common.bench_function("total", |b| {
        b.iter(|| {
            for line in &common_lines {
                parse(line);
            }
        })
    });
    common.finish();

    let mut buf = String::new();
    File::open("samples/apache_combined.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let combined_lines: Vec<&str> = buf.lines().collect();

    let mut combined = c.benchmark_group("apache/combined");
    combined.throughput(Throughput::Bytes(buf.len() as u64));
    combined.bench_function("stage1", |b| {
        b.iter(|| {
            for line in &combined_lines {
                Stage1::new(line.as_bytes()).find();
            }
        })
    });
    combined.bench_function("stage2", |b| {
        let combined_lines: Vec<(&str, Vec<u32>)> = combined_lines
            .iter()
            .map(|s| (*s, Stage1::new(s.as_bytes()).find()))
            .collect();
        b.iter(|| {
            for (line, structurals) in &combined_lines {
                Stage2::new_with_structurals(line.as_bytes(), structurals.clone());
            }
        })
    });
    combined.bench_function("total", |b| {
        b.iter(|| {
            for line in &combined_lines {
                parse(line);
            }
        })
    });
    combined.finish();

}

criterion_group!(benches, bench);
criterion_main!(benches);
