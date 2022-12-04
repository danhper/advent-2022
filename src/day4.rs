use std::{ops::RangeInclusive, path::Path};

use crate::utils::{self, Day};


fn parse_range(s: &str) -> RangeInclusive<u64> {
    let (start, end) = utils::split2(s, "-", |v| v.parse().unwrap());
    start..=end
}

fn range_includes(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.start() <= r2.start() && r1.end() >= r2.end()
}

fn range_overlaps(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.start() <= r2.end() && r1.end() >= r2.start()
}

pub struct Day4 {
    tasks: Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>,
}

impl Day4 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let lines = utils::read_lines(filepath);
        let tasks = lines.iter().map(|l| utils::split2(l, ",", parse_range)).collect();
        Box::new(Day4 { tasks })
    }

    fn solve<P>(&self, mut predicate: P) -> u64
    where
        P: FnMut(&RangeInclusive<u64>, &RangeInclusive<u64>) -> bool,
    {
        self.tasks
            .iter()
            .filter(|(r1, r2)| predicate(r1, r2) || predicate(r2, r1))
            .count() as u64
    }
}

impl Day for Day4 {
    fn solve_a(&self) -> u64 {
        self.solve(range_includes)
    }

    fn solve_b(&self) -> u64 {
        self.solve(range_overlaps)
    }
}
