use std::fs::File;
use std::io::{BufRead, self};

fn read_input(filepath: &str) -> Vec<u64> {
    let file = File::open(filepath).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut result = vec![0];

    for line_ in lines {
        let line = line_.unwrap();
        if line.is_empty() {
            result.push(0);
        } else {
            let index = result.len() - 1;
            result[index] += line.parse::<u64>().unwrap();
        }
    }
    result.sort_by(|a, b| b.cmp(a));
    result
}

pub fn solve_a() -> u64 {
    let elfs = read_input("data/day1.txt");
    elfs[0]
}

pub fn solve_b() -> u64 {
    let elfs = read_input("data/day1.txt");
    elfs[0] + elfs[1] + elfs[2]
}
