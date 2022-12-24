#![allow(clippy::new_ret_no_self)]

use std::path::Path;

use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod grid;
mod utils;

#[derive(Parser)]
struct Cli {
    day: u8,

    #[arg(short, long)]
    test: bool,

    #[arg(long, default_value = "data")]
    data_dir: String,
}

fn main() {
    let args = Cli::parse();

    let suffix = if args.test { "-test" } else { "" };
    let filename = format!("day{}{}.txt", &args.day, suffix);
    let filepath = Path::new(&args.data_dir).join(&filename);

    match &args.day {
        1 => day1::Day1::new(&filepath).output_solutions(),
        2 => day2::Day2::new(&filepath).output_solutions(),
        3 => day3::Day3::new(&filepath).output_solutions(),
        4 => day4::Day4::new(&filepath).output_solutions(),
        5 => day5::Day5::new(&filepath).output_solutions(),
        6 => day6::Day6::new(&filepath).output_solutions(),
        7 => day7::Day7::new(&filepath).output_solutions(),
        8 => day8::Day8::new(&filepath).output_solutions(),
        9 => day9::Day9::new(&filepath).output_solutions(),
        10 => day10::Day10::new(&filepath).output_solutions(),
        11 => day11::Day11::new(&filepath).output_solutions(),
        12 => day12::Day12::new(&filepath).output_solutions(),
        13 => day13::Day13::new(&filepath).output_solutions(),
        14 => day14::Day14::new(&filepath).output_solutions(),
        15 => day15::Day15::new(&filepath, args.test).output_solutions(),
        16 => day16::Day16::new(&filepath).output_solutions(),
        17 => day17::Day17::new(&filepath).output_solutions(),
        18 => day18::Day18::new(&filepath).output_solutions(),
        19 => day19::Day19::new(&filepath).output_solutions(),
        20 => day20::Day20::new(&filepath).output_solutions(),
        21 => day21::Day21::new(&filepath).output_solutions(),
        22 => day22::Day22::new(&filepath).output_solutions(),
        _ => panic!("Day {} not implemented", args.day),
    };
}
