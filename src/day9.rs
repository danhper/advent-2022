use std::{collections::HashSet, path::Path};

use crate::utils::{self, Day};

pub struct Instruction {
    direction: char,
    distance: i64,
}

pub struct Day9 {
    instructions: Vec<Instruction>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn move_by(&mut self, direction: char, distance: i64) {
        match direction {
            'U' => self.y += distance,
            'D' => self.y -= distance,
            'R' => self.x += distance,
            'L' => self.x -= distance,
            _ => panic!("Unknown direction"),
        }
    }

    fn follow(&mut self, head: &Point) {
        if (self.x - head.x).abs() <= 1 && (self.y - head.y).abs() <= 1 {
            return;
        }
        self.x += (head.x - self.x).signum();
        self.y += (head.y - self.y).signum();
    }
}

impl Day9 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let instructions = utils::read_lines(filepath)
            .iter()
            .map(|line| {
                let (direction, distance) = utils::split2(line, " ");
                Instruction {
                    direction,
                    distance,
                }
            })
            .collect();

        Box::new(Self { instructions })
    }

    fn solve(&self, knots_count: usize) -> u64 {
        let mut seen = HashSet::new();
        let mut knots = vec![Point { x: 0, y: 0 }; knots_count];
        for instruction in &self.instructions {
            for _ in 0..instruction.distance {
                knots[0].move_by(instruction.direction, 1);
                for i in 1..knots.len() {
                    let head = knots[i - 1];
                    knots[i].follow(&head);
                }
                seen.insert(*knots.last().unwrap());
            }
        }
        seen.len() as u64
    }
}

impl Day for Day9 {
    fn solve_a(&self) -> u64 {
        self.solve(2)
    }

    fn solve_b(&self) -> u64 {
        self.solve(10)
    }
}
