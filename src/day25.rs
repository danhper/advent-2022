use std::path::Path;

use crate::utils::{self, Day};

pub struct Day25 {
    numbers: Vec<String>,
}

fn snafu_to_decimal(number: &str) -> i64 {
    let mut result = 0;
    let mut multiplier = 1;
    for c in number.chars().rev() {
        let value = match c {
            '0' | '1' | '2' => c as i64 - '0' as i64,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid character: {}", c),
        };
        result += value * multiplier;
        multiplier *= 5;
    }
    result
}

fn decimal_to_snafu(mut number: u64) -> String {
    let mut result = String::new();
    let mut leftover = 0;
    while number > 0 {
        let n = (number % 5) + leftover;
        match n % 5 {
            r@(0 | 1 | 2) => result.push_str(format!("{}", r).as_str()),
            3 => result.push('='),
            4 => result.push('-'),
            r => panic!("Invalid number: {}", r),
        };
        leftover = if n <= 2 { 0 } else { 1 };
        number /= 5;
    }

    if leftover > 0 {
        result.push('1');
    }
    result.chars().rev().collect()
}

impl Day25 {
    pub fn new(filepath: &Path) -> Box<dyn Day<String, u64>> {
        let numbers = utils::read_lines(filepath);
        Box::new(Self { numbers })
    }
}

impl Day<String, u64> for Day25 {
    fn solve_a(&self) -> String {
        let sum = self
            .numbers
            .iter()
            .map(|n| snafu_to_decimal(n))
            .sum::<i64>();
        decimal_to_snafu(sum as u64)
    }

    fn solve_b(&self) -> u64 {
        0
    }
}

#[test]
fn to_snafu() {
    assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0");
}
