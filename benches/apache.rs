use criterion::*;
use jemallocator::Jemalloc;
use simdlog::parsers::apache::{
    ApacheCombinedParser, ApacheCommonParser, ApacheErrorParser, ApacheParser,
};
use simdlog::parsers::Parser;
use simdlog::stage1::Stage1;
use std::fs::File;
use std::io::Read;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn bench(c: &mut Criterion) {
    let mut common_lines_buf = String::new();
    File::open("samples/apache_common.txt")
        .unwrap()
        .read_to_string(&mut common_lines_buf)
        .unwrap();
    let common_lines: Vec<&str> = common_lines_buf.lines().collect();

    let mut common = c.benchmark_group("apache/common");
    common.throughput(Throughput::Bytes(common_lines_buf.len() as u64));
    common.bench_function("stage1", |b| {
        b.iter(|| {
            for line in &common_lines {
                Stage1::new(line).parse();
            }
        })
    });
    common.bench_function("stage2", |b| {
        let common_lines: Vec<(&str, Vec<u32>)> = common_lines
            .iter()
            .map(|s| (*s, Stage1::new(s).parse()))
            .collect();
        b.iter(|| {
            for (line, structurals) in &common_lines {
                ApacheCommonParser::new(structurals).parse(line);
            }
        })
    });
    common.bench_function("total", |b| {
        b.iter(|| {
            for line in &common_lines {
                ApacheCommonParser::new(&Stage1::new(line).parse()).parse(line);
            }
        })
    });
    common.finish();

    let mut combined_lines_buf = String::new();
    File::open("samples/apache_combined.txt")
        .unwrap()
        .read_to_string(&mut combined_lines_buf)
        .unwrap();
    let combined_lines: Vec<&str> = combined_lines_buf.lines().collect();

    let mut combined = c.benchmark_group("apache/combined");
    combined.throughput(Throughput::Bytes(combined_lines_buf.len() as u64));
    combined.bench_function("stage1", |b| {
        b.iter(|| {
            for line in &combined_lines {
                Stage1::new(line).parse();
            }
        })
    });
    combined.bench_function("stage2", |b| {
        let combined_lines: Vec<(&str, Vec<u32>)> = combined_lines
            .iter()
            .map(|s| (*s, Stage1::new(s).parse()))
            .collect();
        b.iter(|| {
            for (line, structurals) in &combined_lines {
                ApacheCombinedParser::new(structurals).parse(line);
            }
        })
    });
    combined.bench_function("total", |b| {
        b.iter(|| {
            for line in &combined_lines {
                ApacheCombinedParser::new(&Stage1::new(line).parse()).parse(line);
            }
        })
    });
    combined.finish();

    let mut error_lines_buf = String::new();
    File::open("samples/apache_error.txt")
        .unwrap()
        .read_to_string(&mut error_lines_buf)
        .unwrap();
    let error_lines: Vec<&str> = error_lines_buf.lines().collect();

    let mut error = c.benchmark_group("apache/error");
    error.throughput(Throughput::Bytes(error_lines_buf.len() as u64));
    error.bench_function("stage1", |b| {
        b.iter(|| {
            for line in &error_lines {
                Stage1::new(line).parse();
            }
        })
    });
    error.bench_function("stage2", |b| {
        let error_lines: Vec<(&str, Vec<u32>)> = error_lines
            .iter()
            .map(|s| (*s, Stage1::new(s).parse()))
            .collect();
        b.iter(|| {
            for (line, structurals) in &error_lines {
                ApacheErrorParser::new(structurals).parse(line);
            }
        })
    });
    error.bench_function("total", |b| {
        b.iter(|| {
            for line in &error_lines {
                ApacheErrorParser::new(&Stage1::new(line).parse()).parse(line);
            }
        })
    });
    error.finish();

    let mut all = c.benchmark_group("apache/all");
    all.throughput(Throughput::Bytes(
        (common_lines_buf.len() + combined_lines_buf.len() + error_lines_buf.len()) as u64,
    ));
    all.bench_function("total", |b| {
        b.iter(|| {
            for line in &common_lines {
                ApacheParser::new(&Stage1::new(line).parse()).parse(line);
            }
            for line in &combined_lines {
                ApacheParser::new(&Stage1::new(line).parse()).parse(line);
            }
            for line in &error_lines {
                ApacheParser::new(&Stage1::new(line).parse()).parse(line);
            }
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
