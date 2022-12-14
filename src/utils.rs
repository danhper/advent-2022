use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

use regex::{Captures, Regex};

pub trait Day<A: std::fmt::Display = u64, B: std::fmt::Display = u64> {
    fn solve_a(&self) -> A;
    fn solve_b(&self) -> B;

    fn output_solutions(&self) {
        let before_a = Instant::now();
        let result_a = self.solve_a();
        println!("Part A ({:.2?}):\n{}", before_a.elapsed(), result_a);
        let before_b = Instant::now();
        let result_b = self.solve_b();
        println!("Part B ({:.2?}):\n{}", before_b.elapsed(), result_b);
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

pub fn split3<T, U, V>(s: &str, pat: &str) -> (T, U, V)
where
    T: std::str::FromStr,
    U: std::str::FromStr,
    V: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <U as std::str::FromStr>::Err: std::fmt::Debug,
    <V as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut parts = s.split(pat);
    let a = parts.next().unwrap().parse().unwrap();
    let b = parts.next().unwrap().parse().unwrap();
    let c = parts.next().unwrap().parse().unwrap();
    (a, b, c)
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

pub fn captures<'t>(re: &str, s: &'t str) -> Captures<'t> {
    Regex::new(re).unwrap().captures(s).unwrap()
}

pub fn get_cap<T>(caps: &Captures, i: usize) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    caps.get(i).unwrap().as_str().parse().unwrap()
}

pub fn get_cap_str(caps: &Captures, i: usize) -> String
{
    caps.get(i).unwrap().as_str().to_string()
}
