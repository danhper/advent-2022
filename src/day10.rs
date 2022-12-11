use std::{path::Path, str::FromStr};

use crate::utils::{self, Day};

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            s if s.starts_with("addx") => Ok(Self::Addx(utils::split2::<String, i64>(s, " ").1)),
            _ => Err(format!("Unknown instruction {}", s)),
        }
    }
}

pub struct Day10 {
    instructions: Vec<Instruction>,
}

impl Day10 {
    pub fn new(filepath: &Path) -> Box<dyn Day<i64, String>> {
        let instructions = utils::parse_lines(&utils::read_lines(filepath));
        Box::new(Self { instructions })
    }

    fn run_pipeline<F>(&self, at_cycle: &mut F)
    where
        F: FnMut(u64, i64),
    {
        let mut cycle = 1;
        let mut register = 1;
        for instr in self.instructions.iter() {
            at_cycle(cycle, register);
            cycle += 1;
            if let Instruction::Addx(x) = instr {
                at_cycle(cycle, register);
                cycle += 1;
                register += x;
            }
        }
    }
}

impl Day<i64, String> for Day10 {
    fn solve_a(&self) -> i64 {
        let mut result = 0;
        self.run_pipeline(&mut |cycle, register| {
            if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
                result += (cycle as i64) * register;
            }
        });
        result
    }

    fn solve_b(&self) -> String {
        let mut result = vec![];
        self.run_pipeline(&mut |cycle, register| {
            let in_range = (register - 1..=register + 1).contains(&((cycle as i64 - 1) % 40));
            result.push(if in_range { "#" } else { "." });
        });
        result
            .chunks(40)
            .map(|s| s.join(""))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
