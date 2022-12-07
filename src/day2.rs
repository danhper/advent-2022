use std::path::Path;

use crate::utils::{self, Day};

fn score(hand: &char) -> u64 {
    match hand {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid hand: {}", hand),
    }
}

fn score_round(player: &char, other: &char) -> u64 {
    match (player, other) {
        ('X', 'A') | ('Y', 'B') | ('Z', 'C') => 3,
        ('X', 'C') | ('Y', 'A') | ('Z', 'B') => 6,
        _ => 0,
    }
}

fn get_hand(player: &char, other: &char) -> char {
    match (player, other) {
        ('X', 'A') | ('Y', 'C') | ('Z', 'B') => 'Z',
        ('X', 'B') | ('Y', 'A') | ('Z', 'C') => 'X',
        _ => 'Y',
    }
}

fn read_input(filepath: &Path) -> Vec<(char, char)> {
    utils::read_lines(filepath)
        .iter()
        .map(|line| utils::split2(line, " "))
        .collect()
}

fn compute_score(input: &[(char, char)]) -> u64 {
    input
        .iter()
        .map(|(a, b)| score_round(b, a) + score(b))
        .sum()
}

pub struct Day2 {
    input: Vec<(char, char)>,
}

impl Day2 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        Box::new(Day2 {
            input: read_input(filepath),
        })
    }
}

impl Day for Day2 {
    fn solve_a(&self) -> u64 {
        compute_score(&self.input)
    }

    fn solve_b(&self) -> u64 {
        let input: Vec<(char, char)> = self
            .input
            .iter()
            .map(|(a, b)| (*a, get_hand(b, a)))
            .collect();
        compute_score(&input)
    }
}
