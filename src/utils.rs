use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait Day<A: std::fmt::Display = u64, B: std::fmt::Display = u64> {
    fn solve_a(&self) -> A;
    fn solve_b(&self) -> B;

    fn output_solutions(&self) {
        println!("Part A: {}", self.solve_a());
        println!("Part B: {}", self.solve_b());
    }
}

pub fn read_lines<P>(filepath: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut result = vec![];
    for line in lines {
        result.push(line.unwrap());
    }
    result
}
