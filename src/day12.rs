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

    fn compute_cheapest_path(&self, start: &Point) -> Option<u64> {
        let mut open_set = HashSet::from([*start]);
        let mut cheapest_cost = HashMap::from([(*start, 0)]);
        let mut estimated_cost = HashMap::from([(*start, start.manhattan_distance(&self.end))]);
        while !open_set.is_empty() {
            let current = *open_set
                .iter()
                .min_by_key(|p| estimated_cost.get(p).unwrap())
                .unwrap();
            if current == self.end {
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
        self.compute_cheapest_path(start).unwrap()
    }

    fn solve_b(&self) -> u64 {
        self.grid
            .cells
            .iter()
            .filter(|(_, c)| **c == 'a')
            .filter_map(|(p, _)| self.compute_cheapest_path(p))
            .min()
            .unwrap()
    }
}
