use std::collections::HashSet;
use std::path::Path;

use crate::grid::{Grid, Point};
use crate::utils;
use crate::utils::Day;

pub struct Day8 {
    grid: Grid<i64>,
}

impl Day8 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let lines = utils::read_lines(filepath);
        let grid = Grid::from(&lines);
        Box::new(Self { grid })
    }

    fn compute_visible<T, U>(&self, visible: &mut HashSet<Point>, range_x: T, range_y: U)
    where
        T: IntoIterator<Item = u64>,
        U: IntoIterator<Item = u64> + std::clone::Clone,
    {
        for x in range_x {
            let mut maxes = vec![-1, -1];
            for y in range_y.clone() {
                for (i, point) in vec![Point::new(x, y), Point::new(y, x)].iter().enumerate() {
                    let cell = self.grid.cells.get(point).unwrap();
                    if *cell > maxes[i] {
                        visible.insert(*point);
                        maxes[i] = *cell;
                    }
                }
            }
        }
    }

    fn compute_scenic_score_unidirectional(
        &self,
        mut point: Point,
        get_next: fn(Point) -> Point,
    ) -> u64 {
        let height = self.grid.cells.get(&point).unwrap();
        let mut score = 0;
        let (max_x, max_y) = (self.grid.width - 1, self.grid.height - 1);
        loop {
            score += 1;
            point = get_next(point);
            let current_height = self.grid.cells.get(&point);
            if point.x == 0
                || point.y == 0
                || point.x >= max_x
                || point.y >= max_y
                || current_height.unwrap() >= height
            {
                break score;
            }
        }
    }

    fn compute_scenic_score(&self, point: Point) -> u64 {
        let score_right =
            self.compute_scenic_score_unidirectional(point, |p| Point::new(p.x + 1, p.y));
        let score_left =
            self.compute_scenic_score_unidirectional(point, |p| Point::new(p.x - 1, p.y));
        let score_up =
            self.compute_scenic_score_unidirectional(point, |p| Point::new(p.x, p.y - 1));
        let score_down =
            self.compute_scenic_score_unidirectional(point, |p| Point::new(p.x, p.y + 1));
        score_up * score_left * score_right * score_down
    }
}

impl Day for Day8 {
    fn solve_a(&self) -> u64 {
        let mut visible = HashSet::new();
        self.compute_visible(&mut visible, 0..self.grid.width, 0..self.grid.height);
        self.compute_visible(
            &mut visible,
            (0..self.grid.width).rev(),
            (0..self.grid.height).rev(),
        );
        visible.len() as u64
    }

    fn solve_b(&self) -> u64 {
        self.grid
            .cells
            .keys()
            .filter(|point| point.x > 0 && point.y > 0)
            .map(|point| self.compute_scenic_score(*point))
            .max()
            .unwrap()
    }
}
