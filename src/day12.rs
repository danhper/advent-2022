use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::grid::{Grid, Point};
use crate::utils;
use crate::utils::Day;

pub struct Day12 {
    grid: Grid<char>,
    end: Point,
}

impl Day12 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let lines = utils::read_lines(filepath);
        let grid = Grid::from(&lines);
        let end = *grid.cells.iter().find(|(_, c)| **c == 'E').unwrap().0;
        Box::new(Self { grid, end })
    }
}

pub struct Env {
    grid: Grid<char>,
    cache: HashMap<Point, u64>,
    end: Point,
}

impl Env {
    fn new(grid: Grid<char>, end: Point) -> Self {
        Self {
            grid,
            cache: HashMap::new(),
            end,
        }
    }

    fn get_cell(&self, point: &Point) -> Option<char> {
        match self.grid.get(point) {
            Some('E') => Some('E'),
            Some('S') => Some('a'),
            Some(c) => Some(*c),
            None => None,
        }
    }

    fn get_valid_neighbors(&self, point: &Point) -> HashSet<Point> {
        let mut neighbors = self.grid.get_neighbors(point, false);
        let current_val = self.get_cell(point).unwrap();
        neighbors.retain(|p| match self.get_cell(p) {
            Some(next_val) => {
                (next_val == 'E' && current_val == 'z')
                    || (next_val != 'E' && next_val as u64 <= current_val as u64 + 1)
            }
            None => false,
        });
        neighbors
    }
    fn update_cache(&mut self, start: &Point, origin: &HashMap<Point, Point>) {
        let mut current = &self.end;
        let mut i = 0;
        while current != start {
            self.cache.insert(*current, i);
            current = origin.get(current).unwrap();
            i += 1;
        }
    }

    fn compute_cheapest_path(&mut self, start: &Point) -> Option<u64> {
        let mut open_set = HashSet::from([*start]);
        let mut cheapest_cost = HashMap::from([(*start, 0)]);
        let mut estimated_cost = HashMap::from([(*start, start.manhattan_distance(&self.end))]);
        let mut origin = HashMap::new();
        while !open_set.is_empty() {
            let current = *open_set
                .iter()
                .min_by_key(|p| estimated_cost.get(p).unwrap())
                .unwrap();

            if let Some(v) = self.cache.get(&current) {
                return Some(cheapest_cost.get(&current).unwrap() + *v);
            }

            if current == self.end {
                self.update_cache(start, &origin);
                return Some(*cheapest_cost.get(&current).unwrap());
            }

            open_set.remove(&current);
            let neighbors = self.get_valid_neighbors(&current);
            for neighbor in neighbors.iter() {
                let new_score = cheapest_cost.get(&current).unwrap() + 1;
                if &new_score <= cheapest_cost.get(neighbor).unwrap_or(&u64::MAX) {
                    cheapest_cost.insert(*neighbor, new_score);
                    estimated_cost.insert(
                        *neighbor,
                        new_score + neighbor.manhattan_distance(&self.end),
                    );
                    origin.insert(*neighbor, current);
                    open_set.insert(*neighbor);
                }
            }
        }
        None
    }

}

impl Day for Day12 {
    fn solve_a(&self) -> u64 {
        let start = self.grid.cells.iter().find(|(_, c)| **c == 'S').unwrap().0;
        let mut env = Env::new(self.grid.clone(), self.end);
        env.compute_cheapest_path(start).unwrap()
    }

    fn solve_b(&self) -> u64 {
        let mut env = Env::new(self.grid.clone(), self.end);
        self.grid
            .cells
            .iter()
            .filter(|(_, c)| **c == 'a')
            .filter_map(|(p, _)| env.compute_cheapest_path(p))
            .min()
            .unwrap()
    }
}
