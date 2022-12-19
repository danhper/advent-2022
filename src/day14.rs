use std::{collections::HashMap, path::Path};

use crate::{
    grid::Point,
    utils::{self, Day},
};

pub struct Day14 {
    grid: HashMap<Point, char>,
}

struct Env {
    grid: HashMap<Point, char>,
    bottom: i64,
    left: i64,
    right: i64,
}

impl Env {
    fn new(grid: HashMap<Point, char>) -> Self {
        let bottom = grid.keys().map(|p| p.y).max().unwrap();
        let left = grid.keys().map(|p| p.x).min().unwrap();
        let right = grid.keys().map(|p| p.x).max().unwrap();
        Self {
            grid,
            bottom,
            left,
            right,
        }
    }

    fn get_sand_next_position(&self, sand: Point) -> Option<Point> {
        let one_below = Point::new(sand.x, sand.y + 1);
        if !self.grid.contains_key(&one_below) {
            return Some(one_below);
        }

        let one_diagonal_left = Point::new(sand.x - 1, sand.y + 1);
        if !self.grid.contains_key(&one_diagonal_left) {
            return Some(one_diagonal_left);
        }

        let one_diagonal_right = Point::new(sand.x + 1, sand.y + 1);
        if !self.grid.contains_key(&one_diagonal_right) {
            return Some(one_diagonal_right);
        }

        None
    }

    fn add_sand(&mut self) -> bool {
        let mut sand = Point::new(500, 0);
        while sand.y <= self.bottom {
            if let Some(new_sand) = self.get_sand_next_position(sand) {
                sand = new_sand;
            } else {
                self.grid.insert(sand, '.');
                return sand != Point::new(500, 0);
            }
        }
        false
    }

    fn fill_with_sand(&mut self) {
        while self.add_sand() {}
    }

    fn count_sand(&self) -> u64 {
        self.grid.values().filter(|c| **c == '.').count() as u64
    }

    fn fill_bottom(&mut self) {
        for x in self.left-self.bottom..=self.right+self.bottom {
            let point = Point::new(x, self.bottom + 2);
            self.grid.insert(point, '#');
        }
        self.bottom += 2;
    }
}

fn parse_line(line: &str) -> Vec<Point> {
    line.split(" -> ")
        .map(|s| {
            let (x, y) = utils::split2(s, ",");
            Point::new(x, y)
        })
        .collect()
}

fn generate_grid(lines: &[String]) -> HashMap<Point, char> {
    let mut grid = HashMap::new();
    let lines: Vec<_> = lines.iter().map(|s| parse_line(s)).collect();
    for line in lines.iter() {
        let mut from = line[0];
        for to in line.iter().skip(1) {
            for x in from.x.min(to.x)..=from.x.max(to.x) {
                for y in from.y.min(to.y)..=from.y.max(to.y) {
                    grid.insert(Point::new(x, y), '#');
                }
            }
            from = *to;
        }
    }
    grid
}

impl Day14 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let grid = generate_grid(&utils::read_lines(filepath));

        Box::new(Day14 { grid })
    }
}

impl Day for Day14 {
    fn solve_a(&self) -> u64 {
        let mut env = Env::new(self.grid.clone());
        env.fill_with_sand();
        env.count_sand()
    }

    fn solve_b(&self) -> u64 {
        let mut env = Env::new(self.grid.clone());
        env.fill_bottom();
        env.fill_with_sand();
        env.count_sand()
    }
}
