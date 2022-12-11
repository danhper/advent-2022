use std::{collections::HashMap, path::Path};

use crate::utils::{self, get_caps, Day};

#[derive(Debug, Clone)]
pub enum Value {
    Old,
    Constant(u64),
}

impl Value {
    fn eval(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Constant(c) => *c,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    lhs: Value,
    op: String,
    rhs: Value,
}

impl Operation {
    fn eval(&self, old: u64) -> u64 {
        let (lhs, rhs) = (self.lhs.eval(old), self.rhs.eval(old));
        match self.op.as_str() {
            "+" => lhs + rhs,
            "*" => lhs * rhs,
            _ => panic!("Unknown operatoroperation: {}", self.op),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    divisble_by: u64,
    monkey_true: usize,
    monkey_false: usize,
}

#[derive(Debug)]
pub struct Env {
    monkeys: Vec<Monkey>,
    divisor: u64,
    common_denominator: u64,
    items: HashMap<usize, Vec<u64>>,
    inspections: HashMap<usize, u64>,
}

impl Env {
    fn new(monkeys: Vec<Monkey>, divisor: u64) -> Env {
        let common_denominator = monkeys.iter().map(|m| m.divisble_by).product::<u64>();
        let items = HashMap::from_iter(monkeys.iter().map(|m| (m.id, m.items.clone())));
        let inspections = HashMap::from_iter(monkeys.iter().map(|m| (m.id, 0)));
        Env {
            monkeys,
            items,
            inspections,
            divisor,
            common_denominator,
        }
    }

    fn run_round(&mut self) {
        for monkey in self.monkeys.iter() {
            for item in self.items.get(&monkey.id).unwrap().clone() {
                *self.inspections.get_mut(&monkey.id).unwrap() += 1;
                let (mut worriness, monkey_id) = monkey.process_item(item, self.divisor);
                worriness %= self.common_denominator;
                self.items.get_mut(&monkey_id).unwrap().push(worriness);
            }
            self.items.get_mut(&monkey.id).unwrap().clear();
        }
    }

    fn result(&self) -> u64 {
        let mut inspections = self.inspections.values().collect::<Vec<_>>();
        inspections.sort_by(|a, b| b.cmp(a));
        inspections[0] * inspections[1]
    }
}

impl Monkey {
    fn process_item(&self, item: u64, div: u64) -> (u64, usize) {
        let worriness = self.operation.eval(item) / div;
        if worriness % self.divisble_by == 0 {
            (worriness, self.monkey_true)
        } else {
            (worriness, self.monkey_false)
        }
    }
}

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

fn parse_value(value: &str) -> Value {
    if value == "old" {
        Value::Old
    } else {
        Value::Constant(value.parse().unwrap())
    }
}

fn parse_monkey(lines: &[String]) -> Monkey {
    let items_str = lines[1].split_at("  Starting items: ".len()).1;
    let id_caps = get_caps(r"Monkey (\d+):", &lines[0]);
    let op_caps = get_caps(r"Operation: new = (old|\d+) ([*+]) (old|\d+)", &lines[2]);
    let div_caps = get_caps(r"Test: divisible by (\d+)", &lines[3]);
    let monkey_re = r"If (?:true|false): throw to monkey (\d+)";

    Monkey {
        id: id_caps.get(1).unwrap().as_str().parse().unwrap(),
        items: items_str.split(", ").map(|s| s.parse().unwrap()).collect(),
        operation: Operation {
            lhs: parse_value(op_caps.get(1).unwrap().as_str()),
            op: utils::get_cap(&op_caps, 2),
            rhs: parse_value(op_caps.get(3).unwrap().as_str()),
        },
        divisble_by: utils::get_cap(&div_caps, 1),
        monkey_true: utils::get_cap(&get_caps(monkey_re, &lines[4]), 1),
        monkey_false: utils::get_cap(&get_caps(monkey_re, &lines[5]), 1),
    }
}

impl Day11 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let lines = utils::read_lines(filepath);
        let monkeys = lines.chunks(7).map(parse_monkey).collect();
        Box::new(Day11 { monkeys })
    }

    fn solve(&self, iterations: usize, divisor: u64) -> u64 {
        let mut env = Env::new(self.monkeys.clone(), divisor);
        for _ in 0..iterations {
            env.run_round();
        }
        env.result()
    }
}

impl Day for Day11 {
    fn solve_a(&self) -> u64 {
        self.solve(20, 3)
    }

    fn solve_b(&self) -> u64 {
        self.solve(10_000, 1)
    }
}
