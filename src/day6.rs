use std::fs;
use std::path::Path;

use crate::utils::Day;
use std::collections::HashMap;

fn count_non_zero(h: &HashMap<char, u64>) -> usize {
    h.values().filter(|v| **v > 0).count()
}

pub struct Day6 {
    line: String,
}

impl Day6 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        Box::new(Day6 {
            line: fs::read_to_string(filepath).unwrap(),
        })
    }

    fn solve(&self, count: usize) -> u64 {
        let mut m: HashMap<char, u64> = HashMap::new();
        for (i, c) in self.line.chars().enumerate() {
            if i >= count {
                let char_to_remove = self.line.chars().nth(i - count).unwrap();
                m.entry(char_to_remove).and_modify(|n| *n -= 1);
            }
            m.entry(c).and_modify(|f| *f += 1).or_insert(1);
            if count_non_zero(&m) == count {
                return i as u64 + 1;
            }
        }
        panic!("No solution found");
    }
}

impl Day for Day6 {
    fn solve_a(&self) -> u64 {
        self.solve(4)
    }

    fn solve_b(&self) -> u64 {
        self.solve(14)
    }
}
