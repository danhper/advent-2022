use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::{
    grid::{Grid as BaseGrid, Point},
    utils::{self, Day},
};

type Grid = BaseGrid<char>;

pub struct Day24 {
    grid: Grid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from(c: char) -> Option<Self> {
        match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => None,
        }
    }
}

struct Env {
    height: u64,
    width: u64,
    origin: Point,
    end: Point,
    blizzards: HashMap<Point, HashSet<Direction>>,
}

impl Env {
    fn new(grid: &Grid) -> Self {
        let blizzards = grid
            .cells
            .iter()
            .filter_map(|(p, c)| {
                Direction::from(*c).map(|d| (Point::new(p.x - 1, p.y - 1), HashSet::from([d])))
            })
            .collect();
        let height = grid.height - 2;
        let width = grid.width - 2;
        let origin = Point::new(0, -1);
        let end = Point::new(width as i64 - 1, height as i64);
        Self {
            height,
            width,
            origin,
            end,
            blizzards,
        }
    }

    fn find_shortest_path(&mut self, forward: bool) -> u64 {
        let origin = if forward { self.origin } else { self.end };
        let end = if forward { self.end } else { self.origin };
        let mut current_positions = HashSet::from([origin]);
        let mut steps = 0;
        let diffs = [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)];
        loop {
            self.move_blizzards();
            let mut next_postions = HashSet::new();

            for p in current_positions.iter() {
                for diff in diffs.iter() {
                    let new_point = Point::new(p.x + diff.0, p.y + diff.1);
                    if new_point == end {
                        return steps + 1;
                    }
                    if self.blizzards.contains_key(&new_point)
                        || (new_point != origin
                            && (new_point.x < 0
                                || new_point.y < 0
                                || new_point.x >= self.width as i64
                                || new_point.y >= self.height as i64))
                    {
                        continue;
                    }
                    next_postions.insert(new_point);
                }
            }
            current_positions = next_postions;
            steps += 1;
        }
    }

    fn move_blizzards(&mut self) {
        let mut new_blizzards: HashMap<Point, HashSet<Direction>> = HashMap::new();
        for (p, directions) in self.blizzards.iter() {
            for direction in directions {
                let new_point = match direction {
                    Direction::Left => Point::new((p.x - 1).rem_euclid(self.width as i64), p.y),
                    Direction::Right => Point::new((p.x + 1).rem_euclid(self.width as i64), p.y),
                    Direction::Up => Point::new(p.x, (p.y - 1).rem_euclid(self.height as i64)),
                    Direction::Down => Point::new(p.x, (p.y + 1).rem_euclid(self.height as i64)),
                };
                new_blizzards
                    .entry(new_point)
                    .or_default()
                    .insert(*direction);
            }
        }
        self.blizzards = new_blizzards;
    }
}

impl Day24 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let grid = Grid::from(&utils::read_lines(filepath));
        Box::new(Day24 { grid })
    }
}

impl Day for Day24 {
    fn solve_a(&self) -> u64 {
        let mut env = Env::new(&self.grid);
        env.find_shortest_path(true)
    }

    fn solve_b(&self) -> u64 {
        let mut env = Env::new(&self.grid);
        env.find_shortest_path(true) + env.find_shortest_path(false) + env.find_shortest_path(true)
    }
}
