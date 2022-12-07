use std::{num::ParseIntError, ops::RangeInclusive, path::Path, str::FromStr};

use crate::utils::{self, Day};

struct U64RangeInclusive(RangeInclusive<u64>);

impl U64RangeInclusive {
    fn includes(&self, other: &U64RangeInclusive) -> bool {
        self.0.start() <= other.0.start() && self.0.end() >= other.0.end()
    }

    fn overlaps(&self, other: &U64RangeInclusive) -> bool {
        self.0.start() <= other.0.end() && self.0.end() >= other.0.start()
    }
}

impl FromStr for U64RangeInclusive {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = utils::split2::<String, String>(s, "-");
        let x = x_str.parse::<u64>()?;
        let y = y_str.parse::<u64>()?;

        Ok(U64RangeInclusive(x..=y))
    }
}

pub struct Day4 {
    tasks: Vec<(U64RangeInclusive, U64RangeInclusive)>,
}

impl Day4 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let lines = utils::read_lines(filepath);
        let tasks = lines.iter().map(|l| utils::split2(l, ",")).collect();
        Box::new(Day4 { tasks })
    }

    fn solve<P>(&self, mut predicate: P) -> u64
    where
        P: FnMut(&U64RangeInclusive, &U64RangeInclusive) -> bool,
    {
        self.tasks
            .iter()
            .filter(|(r1, r2)| predicate(r1, r2) || predicate(r2, r1))
            .count() as u64
    }
}

impl Day for Day4 {
    fn solve_a(&self) -> u64 {
        self.solve(U64RangeInclusive::includes)
    }

    fn solve_b(&self) -> u64 {
        self.solve(U64RangeInclusive::overlaps)
    }
}
