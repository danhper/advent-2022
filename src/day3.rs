use std::{collections::HashSet, path::Path};

use crate::utils::{self, Day};

pub struct Day3 {
    bags: Vec<String>,
}

impl Day3 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        Box::new(Day3 {
            bags: utils::read_lines(filepath),
        })
    }
}

fn intersect(s1: &str, s2: &str) -> String {
    let s1_set: HashSet<char> = HashSet::from_iter(s1.chars());
    s1_set.intersection(&HashSet::from_iter(s2.chars())).collect()
}

fn get_value(c: char) -> u64 {
    if ('a'..='z').contains(&c) {
        c as u64 - 'a' as u64 + 1
    } else {
        c as u64 - 'A' as u64 + 27
    }
}

fn compute_result(s: &str) -> u64 {
    s.chars().map(get_value).sum()
}

impl Day for Day3 {
    fn solve_a(&self) -> u64 {
        self.bags
            .iter()
            .map(|line| {
                let (a, b) = line.split_at(line.len() / 2);
                compute_result(&intersect(a, b))
            })
            .sum()
    }

    fn solve_b(&self) -> u64 {
        self.bags
            .chunks(3)
            .map(|chunk| compute_result(&intersect(&intersect(&chunk[0], &chunk[1]), &chunk[2])))
            .sum()
    }
}
