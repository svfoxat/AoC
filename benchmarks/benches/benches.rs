use std::fs;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let day_3_input = fs::read_to_string("input_day_3.txt").unwrap();
    let day_3_lines: Vec<&str> = day_3_input.lines().collect();

    c.bench_function("day_3_part_1", |b| b.iter(|| day_3::part_1(&day_3_lines)));
    c.bench_function("day_3_part_2", |b| b.iter(|| day_3::part_2(&day_3_lines)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);