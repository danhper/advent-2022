use std::path::Path;

use crate::utils::Day;
use clap::Parser;

mod day1;
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

    let day = match &args.day {
        1 => day1::Day1::new(&filepath),
        _ => panic!("Day {} not implemented", args.day),
    };

    println!("Result A: {}", day.solve_a());
    println!("Result B: {}", day.solve_b());
}
