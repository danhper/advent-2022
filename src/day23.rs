use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::{
    grid::{get_neighbors, Grid as BaseGrid, Point},
    utils::{self, Day},
};

type Grid = BaseGrid<char>;

pub struct Day23 {
    elves: HashSet<Point>,
}

struct Env {
    elves: HashSet<Point>,
    directions: Vec<Direction>,
}

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_points_required(&self) -> Vec<(i64, i64)> {
        match self {
            Direction::North => vec![(-1, -1), (0, -1), (1, -1)],
            Direction::South => vec![(-1, 1), (0, 1), (1, 1)],
            Direction::West => vec![(-1, -1), (-1, 0), (-1, 1)],
            Direction::East => vec![(1, -1), (1, 0), (1, 1)],
        }
    }

    fn get_proposed_position(&self, elf: &Point) -> Point {
        match self {
            Direction::North => Point::new(elf.x, elf.y - 1),
            Direction::South => Point::new(elf.x, elf.y + 1),
            Direction::West => Point::new(elf.x - 1, elf.y),
            Direction::East => Point::new(elf.x + 1, elf.y),
        }
    }
}

impl Env {
    fn new(elves: HashSet<Point>) -> Env {
        Env {
            elves,
            directions: vec![
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
        }
    }
    fn should_move(&self, elf: &Point) -> bool {
        let neighbors = get_neighbors(elf, true);
        neighbors.iter().any(|p| self.elves.contains(p))
    }

    fn contains_any_around(&self, elf: &Point, points: &[(i64, i64)]) -> bool {
        points.iter().any(|(x, y)| {
            let p = Point::new(elf.x + x, elf.y + y);
            self.elves.contains(&p)
        })
    }

    fn get_proposed_position(&self, elf: &Point) -> Point {
        for direction in self.directions.iter() {
            if !self.contains_any_around(elf, &direction.get_points_required()) {
                return direction.get_proposed_position(elf);
            }
        }
        *elf
    }

    fn get_new_positions(&self) -> HashSet<Point> {
        let mut positions_count = HashMap::new();
        let mut computed_positions = HashMap::new();
        for elf in self.elves.iter() {
            if self.should_move(elf) {
                let new_position = self.get_proposed_position(elf);
                let current_count = positions_count.get(&new_position).unwrap_or(&0);
                positions_count.insert(new_position, 1 + *current_count);
                computed_positions.insert(elf, new_position);
            }
        }
        let mut new_positions = HashSet::new();
        for elf in self.elves.iter() {
            let new_position = match computed_positions.get(elf) {
                Some(p) if positions_count.get(p).unwrap() == &1 => p,
                _ => elf,
            };
            new_positions.insert(*new_position);
        }
        new_positions
    }

    fn run_round(&mut self) {
        let new_positions = self.get_new_positions();
        self.elves = new_positions;
        let removed = self.directions.remove(0);
        self.directions.push(removed);
    }

    fn get_score(&self) -> u64 {
        let min_x = self.elves.iter().map(|p| p.x).min().unwrap();
        let min_y = self.elves.iter().map(|p| p.y).min().unwrap();
        let max_x = self.elves.iter().map(|p| p.x).max().unwrap();
        let max_y = self.elves.iter().map(|p| p.y).max().unwrap();
        let area = (max_x - min_x + 1) * (max_y - min_y + 1);
        (area as usize - self.elves.len()) as u64
    }
}

impl Day23 {
    pub fn new(input: &Path) -> Box<dyn Day> {
        let grid = Grid::from(&utils::read_lines(input));
        let elves = grid
            .cells
            .iter()
            .filter(|(_, c)| **c == '#')
            .map(|(p, _)| *p)
            .collect();
        Box::new(Self { elves })
    }
}

impl Day for Day23 {
    fn solve_a(&self) -> u64 {
        let mut env = Env::new(self.elves.clone());
        for _ in 0..10 {
            env.run_round();
        }
        env.get_score()
    }

    fn solve_b(&self) -> u64 {
        let mut env = Env::new(self.elves.clone());
        let mut i = 0;
        loop {
            i += 1;
            let elves = env.elves.clone();
            env.run_round();
            if env.elves == elves {
                break;
            }
        }
        i
    }
}
