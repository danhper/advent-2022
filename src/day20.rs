use std::path::Path;

use crate::utils::{self, Day};

pub struct Day20 {
    input: Vec<i64>,
}

#[derive(Debug)]
struct TrackedList {
    list: Vec<(usize, i64)>,
}

impl std::fmt::Display for TrackedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.list
            .iter()
            .for_each(|(_, v)| s.push_str(&format!("{} ", v)));
        write!(f, "{}", s)
    }
}

impl TrackedList {
    fn new(numbers: Vec<i64>) -> Self {
        let list = numbers.into_iter().enumerate().collect();
        Self { list }
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn nth_after(&self, i: usize, n: usize) -> i64 {
        let index = (i + n % self.len()) % self.len();
        self.list[index].1
    }

    fn move_elem(&mut self, i: usize) {
        let (index, (_, value)) = self.list.iter().enumerate().find(|v| v.1 .0 == i).unwrap();
        let v = *value;
        self.list.remove(index);
        let target = (index + ((v.rem_euclid(self.len() as i64)) as usize)) % self.len();
        self.list.insert(target, (i, v));
    }

    fn move_all(&mut self) {
        for i in 0..self.list.len() {
            self.move_elem(i);
        }
    }
}

impl Day20 {
    pub fn new(filepath: &Path) -> Box<dyn Day<i64, i64>> {
        let input = utils::parse_lines(&utils::read_lines(filepath));
        Box::new(Day20 { input })
    }
}

fn solve(input: Vec<i64>, moves: usize) -> i64 {
    let mut list = TrackedList::new(input);
    for _ in 0..moves {
        list.move_all();
    }
    let index_0 = list.list.iter().enumerate().find(|v| v.1.1 == 0).unwrap().0;
    [1000, 2000, 3000].iter().map(|n| list.nth_after(index_0, *n)).sum()
}

impl Day<i64, i64> for Day20 {
    fn solve_a(&self) -> i64 {
        solve(self.input.clone(), 1)
    }

    fn solve_b(&self) -> i64 {
        let input = self.input.iter().map(|v| v * 811589153).collect();
        solve(input, 10)
    }
}
