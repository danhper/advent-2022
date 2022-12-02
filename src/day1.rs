use std::path::Path;

use crate::utils;
use crate::utils::Day;

fn read_input(filepath: &Path) -> Vec<u64> {
    let mut result = vec![0];

    for line in utils::read_lines(filepath) {
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
pub struct Day1 {
    sorted_elfs: Vec<u64>,
}

impl Day1 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        Box::new(Day1 {
            sorted_elfs: read_input(filepath),
        })
    }
}

impl Day for Day1 {
    fn solve_a(&self) -> u64 {
        self.sorted_elfs[0]
    }

    fn solve_b(&self) -> u64 {
        self.sorted_elfs.iter().take(3).sum()
    }
}
