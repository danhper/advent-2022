use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::{Regex, Captures};

pub trait Day<A: std::fmt::Display = u64, B: std::fmt::Display = u64> {
    fn solve_a(&self) -> A;
    fn solve_b(&self) -> B;

    fn output_solutions(&self) {
        println!("Part A:\n{}", self.solve_a());
        println!("Part B:\n{}", self.solve_b());
    }
}

pub fn read_lines<P>(filepath: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap()).collect()
}

pub fn split2<T, U>(s: &str, pat: &str) -> (T, U)
where
    T: std::str::FromStr,
    U: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <U as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut parts = s.split(pat);
    let a = parts.next().unwrap().parse().unwrap();
    let b = parts.next().unwrap().parse().unwrap();
    (a, b)
}

pub fn parse_lines<T>(lines: &[String]) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    lines.iter().map(|l| l.parse().unwrap()).collect()
}

pub fn get_caps<'a>(re: &str, s: &'a str) -> Captures<'a> {
    Regex::new(re).unwrap().captures(s).unwrap()
}

pub fn get_cap<T>(caps: &Captures, i: usize) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    caps.get(i).unwrap().as_str().parse().unwrap()
}
