use std::fs::{metadata, read_to_string};

use crate::{
    day::Day,
    day_01::Day01,
    day_02::Day02,
    // day_03::Day03,
    // day_04::Day04,
    // day_05::Day05,
    // day_06::Day06,
    // day_07::Day07,
    // day_08::Day08,
    // day_09::Day09,
    // day_10::Day10,
    // day_11::Day11,
    // day_12::Day12,
};

mod day;
mod day_01;
mod day_02;
// mod day_03;
// mod day_04;
// mod day_05;
// mod day_06;
// mod day_07;
// mod day_08;
// mod day_09;
// mod day_10;
// mod day_11;
// mod day_12;

fn run_day<DAY: Day>(input: &str) {
    let parsed_input = DAY::parse_input(input);
    let part_1_output = DAY::part_1(&parsed_input);
    let part_2_output = DAY::part_2(&parsed_input, &part_1_output);
    println!(
        "[ Day {} ] || [ Part 1: \x1b[95m{:16}\x1b[0m ] [ Part 2: \x1b[95m{:16}\x1b[0m ]",
        DAY::id(),
        part_1_output,
        part_2_output
    );
}

fn run_day_number(number: u8, input: &str) {
    match number {
        1 => run_day::<Day01>(&input),
        2 => run_day::<Day02>(&input),
        3 => {}  // run_day::<Day03>(&input),
        4 => {}  // run_day::<Day04>(&input),
        5 => {}  // run_day::<Day05>(&input),
        6 => {}  // run_day::<Day06>(&input),
        7 => {}  // run_day::<Day07>(&input),
        8 => {}  // run_day::<Day08>(&input),
        9 => {}  // run_day::<Day09>(&input),
        10 => {} // run_day::<Day10>(&input),
        11 => {} // run_day::<Day11>(&input),
        12 => {} // run_day::<Day12>(&input),
        _ => println!("Usage: aoc2025 (<input> <day> | <inputs_dir>)"),
    }
}

fn main() {
    println!(
        "\n========================= \x1b[96mAdvent of Code 2025\x1b[0m =========================\n"
    );

    let start_ts = std::time::Instant::now();

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: aoc2025 (<input> <day> | <inputs_dir>)");
        return;
    }

    let input = if args.len() > 2 {
        let Ok(input) = std::fs::read_to_string(&args[1]) else {
            eprintln!("Couldn't read input file");
            return;
        };
        std::fs::canonicalize(input)
    } else {
        let input = args[1].to_owned();
        if !metadata(&input).expect("Couldn't read input dir").is_dir() {
            eprintln!("{} is not a directory", input);
        }
        std::fs::canonicalize(input)
    }
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned();

    let day = args.get(2).and_then(|r| r.parse::<u8>().ok());

    if let Some(day) = day {
        run_day_number(day, &input.trim());
    } else {
        for n in 1..=12 {
            let Ok(day_input) = read_to_string(format!("{}/{:02}.txt", input, n)) else {
                eprintln!("Couldn't read input file for day {}", n);
                continue;
            };
            run_day_number(n, &day_input.trim());
        }
    }

    println!(
        "\nTotal runtime: {}µs\n",
        (std::time::Instant::now() - start_ts).as_micros()
    );

    println!("=======================================================================\n");
}
