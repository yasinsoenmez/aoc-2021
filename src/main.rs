use aoc2021_lib::days::*;
use std::path::Path;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

fn main() {
    let day1_path = Path::new("input/day1.txt");
    let day2_path = Path::new("input/day2.txt");

    let day1_str = std::fs::read_to_string(day1_path).unwrap();
    let day2_str = std::fs::read_to_string(day2_path).unwrap();

    let day1_parsed = day1::day1_parse(&day1_str);
    let day2_parsed = day2::day2_parse(&day2_str);

    let day1_1_result = day1::day1_1(&day1_parsed);
    let day1_2_result = day1::day1_2(&day1_parsed);

    let table = vec![
        vec!["Day 1".cell(),
             day1_1_result.cell().justify(Justify::Right),
             day1_2_result.cell().justify(Justify::Right)
        ],
        vec!["Day 2".cell(),
             day2::day2_1(&day2_parsed).cell().justify(Justify::Right),
             day2::day2_2(&day2_parsed).cell().justify(Justify::Right)
        ],
    ]
        .table()
        .title(vec![
            "Day".cell().bold(true),
            "Part 1".cell().bold(true),
            "Part 2".cell().bold(true)
        ]).bold(true);

    print_stdout(table).unwrap();
}
