#![allow(clippy::new_ret_no_self)]

use std::path::Path;

use clap::Parser;

mod day1;
mod day2;
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
        _ => panic!("Day {} not implemented", args.day),
    };
}
