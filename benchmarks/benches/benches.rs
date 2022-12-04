use std::{fs};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let day_1_input = fs::read_to_string("../day_1/input.txt").unwrap();
    let day_1_lines: Vec<&str> = day_1_input.lines().collect();
    c.bench_function("day_1_part_1", |b| b.iter(|| day_1::part_1(&day_1_lines)));
    c.bench_function("day_1_part_2", |b| b.iter(|| day_1::part_2(&day_1_lines)));

    let day_3_input = fs::read_to_string("../day_3/input.txt").unwrap();
    let day_3_lines: Vec<&str> = day_3_input.lines().collect();
    c.bench_function("day_3_part_1", |b| b.iter(|| day_3::part_1(&day_3_lines)));
    c.bench_function("day_3_part_2", |b| b.iter(|| day_3::part_2(&day_3_lines)));

    let day_4_input = fs::read_to_string("../day_4/input.txt").unwrap();
    let day_4_lines: Vec<&str> = day_4_input.lines().collect();
    c.bench_function("day_4_part_1", |b| b.iter(|| day_4::part_1(&day_4_lines)));
    c.bench_function("day_4_part_2", |b| b.iter(|| day_4::part_2(&day_4_lines)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);