use std::{collections::HashMap, path::Path};

use crate::utils::{self, captures, get_cap_str, Day};

#[derive(Clone)]
enum Expr {
    Bin(String, Box<Expr>, Box<Expr>),
    Variable(String),
    Constant(u64),
}

impl Expr {
    fn eval(&self, env: &Environment) -> i64 {
        match self {
            Expr::Constant(n) => *n as i64,
            Expr::Variable(s) => env.variables.get(s).unwrap().eval(env),
            Expr::Bin(op, lhs, rhs) => {
                let (lhs_v, rhs_v) = (lhs.eval(env), rhs.eval(env));
                match op.as_str() {
                    "+" => lhs_v + rhs_v,
                    "-" => lhs_v - rhs_v,
                    "*" => lhs_v * rhs_v,
                    "/" => lhs_v / rhs_v,
                    _ => panic!("Unknown operator: {}", op),
                }
            }
        }
    }
}

#[derive(Clone)]
struct Environment {
    variables: HashMap<String, Expr>,
}

pub struct Day21 {
    env: Environment,
}

fn parse_line(line: &str) -> (String, Expr) {
    let (name, expr) = line.split_once(": ").unwrap();
    if let Ok(n) = expr.parse() {
        (name.to_string(), Expr::Constant(n))
    } else {
        let caps = captures(r"([a-z]+) ([\+\*/-]) ([a-z]+)", expr);
        (
            name.to_string(),
            Expr::Bin(
                get_cap_str(&caps, 2),
                Box::new(Expr::Variable(get_cap_str(&caps, 1))),
                Box::new(Expr::Variable(get_cap_str(&caps, 3))),
            ),
        )
    }
}

impl Day21 {
    pub fn new(filepath: &Path) -> Box<dyn Day<i64, i64>> {
        let variables = utils::read_lines(filepath)
            .iter()
            .map(|line| parse_line(line))
            .collect();
        let env = Environment { variables };
        Box::new(Self { env })
    }
}

impl Day<i64, i64> for Day21 {
    fn solve_a(&self) -> i64 {
        self.env.variables.get("root").unwrap().eval(&self.env)
    }

    fn solve_b(&self) -> i64 {
        let mut env = self.env.clone();
        let (lhs, rhs) = match self.env.variables.get("root").unwrap() {
            Expr::Bin(_, lhs, rhs) => (lhs, rhs),
            _ => panic!("No root"),
        };
        let (mut left, mut right) = (0, u64::MAX / 1_000_000); // lower to avoid overflows
        loop {
            let i = (left + right) / 2;
            env.variables.insert("humn".to_string(), Expr::Constant(i));
            match lhs.eval(&env).cmp(&rhs.eval(&env)) {
                std::cmp::Ordering::Equal => break (i as i64),
                std::cmp::Ordering::Greater => left = i,
                std::cmp::Ordering::Less => right = i,
            }
        }
    }
}
