use aoc2021_lib::days::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::Path;
//noinspection ALL,RsUnresolvedReference
fn criterion_benchmark(c: &mut Criterion) {

    let day1_path = Path::new("../input/day1.txt");
    let day1_str = fs::read_to_string(day1_path).unwrap();
    let day1_parsed = day1::day1_parse(&day1_str);
    c.bench_function("Day1 Parsing:", |b| b.iter(|| day1::day1_parse(&day1_str)));

    c.bench_function("Day1 Part1:", |b| b.iter(|| day1::day1_1(&day1_parsed)));
    c.bench_function("Day1 Part2:", |b| b.iter(|| day1::day1_2(&day1_parsed)));

    let day2_path = Path::new("../input/day2.txt");
    let day2_str = fs::read_to_string(day2_path).unwrap();
    let day2_parsed = day2::day2_parse(&day2_str);
    c.bench_function("Day2 Parsing:", |b| b.iter(|| day2::day2_parse(&day2_str)));

    c.bench_function("Day2 Part1:", |b| b.iter(|| day2::day2_1(&day2_parsed)));
    c.bench_function("Day2 Part2:", |b| b.iter(|| day2::day2_2(&day2_parsed)));

    let day3_path = Path::new("../input/day3.txt");
    let day3_str = fs::read_to_string(day3_path).unwrap();
    let day3_parsed = day3::day3_parse(&day3_str);
    c.bench_function("Day3 Parsing:", |b| b.iter(|| day3::day3_parse(&day3_str)));
    c.bench_function("Day3 Part1:", |b| b.iter(|| day3::day3_1(&day3_parsed)));
    c.bench_function("Day3 Part2:", |b| b.iter(|| day3::day3_2(&day3_parsed)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
