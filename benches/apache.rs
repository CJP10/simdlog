use criterion::*;
use jemallocator::Jemalloc;
use simdlog::apache::access::avx2::{Stage1 as AccessStage1, Stage2 as AccessStage2};
use simdlog::apache::access::parse as access_parse;
use simdlog::apache::error::avx2::{Stage1 as ErrorStage1, Stage2 as ErrorStage2};
use simdlog::apache::error::parse as error_parse;
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
                AccessStage1::new(line.as_bytes()).find();
            }
        })
    });
    common.bench_function("stage2", |b| {
        let common_lines: Vec<(&str, Vec<u32>)> = common_lines
            .iter()
            .map(|s| (*s, AccessStage1::new(s.as_bytes()).find()))
            .collect();
        b.iter(|| {
            for (line, structurals) in &common_lines {
                AccessStage2::new_with_structurals(line.as_bytes(), structurals.clone());
            }
        })
    });
    common.bench_function("total", |b| {
        b.iter(|| {
            for line in &common_lines {
                access_parse(line);
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
                AccessStage1::new(line.as_bytes()).find();
            }
        })
    });
    combined.bench_function("stage2", |b| {
        let combined_lines: Vec<(&str, Vec<u32>)> = combined_lines
            .iter()
            .map(|s| (*s, AccessStage1::new(s.as_bytes()).find()))
            .collect();
        b.iter(|| {
            for (line, structurals) in &combined_lines {
                AccessStage2::new_with_structurals(line.as_bytes(), structurals.clone());
            }
        })
    });
    combined.bench_function("total", |b| {
        b.iter(|| {
            for line in &combined_lines {
                access_parse(line);
            }
        })
    });
    combined.finish();

    let mut buf = String::new();
    File::open("samples/apache_error.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let error_lines: Vec<&str> = buf.lines().collect();

    let mut error = c.benchmark_group("apache/error");
    error.throughput(Throughput::Bytes(buf.len() as u64));
    error.bench_function("stage1", |b| {
        b.iter(|| {
            for line in &error_lines {
                ErrorStage1::new(line.as_bytes()).find();
            }
        })
    });
    error.bench_function("stage2", |b| {
        let error_lines: Vec<(&str, Vec<u32>)> = error_lines
            .iter()
            .map(|s| (*s, ErrorStage1::new(s.as_bytes()).find()))
            .collect();
        b.iter(|| {
            for (line, structurals) in &error_lines {
                ErrorStage2::new_with_structurals(line.as_bytes(), structurals.clone());
            }
        })
    });
    error.bench_function("total", |b| {
        b.iter(|| {
            for line in &error_lines {
                error_parse(line);
            }
        })
    });
    error.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
