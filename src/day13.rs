use std::{path::Path, str::FromStr};

use crate::utils::{self, Day};

#[derive(Debug, Clone, Eq)]
enum Value {
    Integer(u64),
    List(Vec<Value>),
}

fn parse_value(s: &[char]) -> Result<(Value, &[char]), String> {
    match s.iter().next() {
        None => Err("empty string".to_owned()),
        Some('[') => {
            let mut children = vec![];
            let mut rest = &s[1..];
            while rest[0] != ']' {
                let (child, rest2) = parse_value(rest)?;
                rest = rest2;
                children.push(child);
                if rest[0] == ',' {
                    rest = &rest[1..];
                }
            }
            Ok((Value::List(children), &rest[1..]))
        }
        Some(c) if c.is_ascii_digit() => {
            let mut value = 0;
            let mut rest = s;
            while rest[0].is_ascii_digit() {
                value = value * 10 + (rest[0] as u64 - '0' as u64);
                rest = &rest[1..];
            }
            Ok((Value::Integer(value), rest))
        }
        _ => Err(format!("invalid string: {}", s.iter().collect::<String>())),
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (v1 @ Value::List(_), v2 @ Value::Integer(_)) => v1 == &Value::List(vec![v2.clone()]),
            (v1 @ Value::Integer(_), v2 @ Value::List(_)) => v2 == v1,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.partial_cmp(b),
            (Value::List(a), Value::List(b)) => a.partial_cmp(b),
            (v1 @ Value::List(_), v2 @ Value::Integer(_)) => {
                v1.partial_cmp(&Value::List(vec![v2.clone()]))
            }
            (v1 @ Value::Integer(_), v2 @ Value::List(_)) => {
                Value::List(vec![v1.clone()]).partial_cmp(v2)
            }
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_value(s.chars().collect::<Vec<_>>().as_slice()).map(|(v, _)| v)
    }
}

pub struct Day13 {
    inputs: Vec<Value>,
}

impl Day13 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let inputs = utils::read_lines(filepath)
            .iter()
            .filter(|l| !l.is_empty())
            .map(|v| v.parse().unwrap())
            .collect();
        Box::new(Day13 { inputs })
    }
}

impl Day for Day13 {
    fn solve_a(&self) -> u64 {
        self.inputs
            .chunks(2)
            .enumerate()
            .map(|(i, v)| if v[0] <= v[1] { i as u64 + 1 } else { 0 })
            .sum()
    }

    fn solve_b(&self) -> u64 {
        let mut inputs_with_packets = self.inputs.clone();
        let first_packet: Value = "[[2]]".parse().unwrap();
        let second_packet: Value = "[[6]]".parse().unwrap();
        inputs_with_packets.push(first_packet.clone());
        inputs_with_packets.push(second_packet.clone());
        inputs_with_packets.sort();
        let get_index = |p| inputs_with_packets.iter().position(|v| v == p).unwrap() as u64 + 1;
        let index_first = get_index(&first_packet);
        let index_second = get_index(&second_packet);
        index_first * index_second
    }
}
