use aoc2021_lib::days::*;
use std::fs;
use std::path::Path;

fn main() {
    let day1_path = Path::new("input/day1.txt");
    let day2_path = Path::new("input/day2.txt");

    let day1_str = fs::read_to_string(day1_path).unwrap();
    let day2_str = fs::read_to_string(day2_path).unwrap();

    let day1_parsed = day1::parse(&day1_str);

    println!("{}", day1::day1_1(&day1_parsed));
    println!("{}\n", day1::day1_2(&day1_parsed));

    println!("{}", day2::day2_1(&day2_str));
    println!("{}\n", day2::day2_2(&day2_str));
}
