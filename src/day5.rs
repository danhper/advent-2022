use regex::Regex;
use std::path::Path;

use crate::utils::{self, Day};

fn parse_instruction(s: &str) -> Instruction {
    let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let caps = r.captures(s).unwrap();
    Instruction {
        count: caps.get(1).unwrap().as_str().parse().unwrap(),
        from: caps.get(2).unwrap().as_str().parse().unwrap(),
        to: caps.get(3).unwrap().as_str().parse().unwrap(),
    }
}

fn parse_elements(s: &str) -> Vec<char> {
    (0..s.len())
        .step_by(4)
        .map(|i| s.chars().nth(i + 1).unwrap())
        .collect()
}

fn parse_input(filepath: &Path) -> Day5 {
    let mut instructions = vec![];
    let lines = utils::read_lines(filepath);
    let elems_count = (lines[0].len() + 1) / 4;
    let mut stacks = vec![vec![]; elems_count];
    for line in lines {
        if line.starts_with("move") {
            instructions.push(parse_instruction(&line))
        } else if line.contains('[') {
            for (i, c) in parse_elements(&line).iter().enumerate() {
                if *c != ' ' {
                    stacks[i].insert(0, *c);
                }
            }
        }
    }

    Day5 {
        stacks,
        instructions,
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    count: u64,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
pub struct Day5 {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Day5 {
    pub fn new(filepath: &Path) -> Box<dyn Day<String, String>> {
        Box::new(parse_input(filepath))
    }

    fn execute_instruction_a(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.count {
            let elem = self.stacks[instruction.from - 1].pop().unwrap();
            self.stacks[instruction.to - 1].push(elem);
        }
    }

    fn execute_instruction_b(&mut self, instruction: &Instruction) {
        let mut popped_elems = vec![];
        for _ in 0..instruction.count {
            popped_elems.push(self.stacks[instruction.from - 1].pop().unwrap());
        }
        popped_elems.reverse();
        self.stacks[instruction.to - 1].append(&mut popped_elems);
    }

    fn solve(&self, problem: char) -> String {
        let mut state = self.clone();
        for instruction in self.instructions.iter() {
            if problem == 'a' {
                state.execute_instruction_a(instruction);
            } else {
                state.execute_instruction_b(instruction);
            }
        }
        state.stack_top()
    }

    fn stack_top(&self) -> String {
        self.stacks.iter().map(|s| *s.last().unwrap()).collect()
    }
}

impl Day<String, String> for Day5 {
    fn solve_a(&self) -> String {
        self.solve('a')
    }

    fn solve_b(&self) -> String {
        self.solve('b')
    }
}
