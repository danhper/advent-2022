use std::path::Path;

use crate::{
    grid::{Grid, Point},
    utils::{self, Day},
};

#[rustfmt::skip]
static ROCKS_PATTERNS: [&[Point]; 5] = [
    &[Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 2, y: 0}, Point{x: 3, y: 0}],
    &[Point{x: 1, y: 0}, Point{x: 0, y: -1}, Point{x: 1, y: -1}, Point{x: 2, y: -1}, Point{x: 1, y: -2}],
    &[Point{x: 2, y: 0}, Point{x: 2, y: -1}, Point{x: 0, y: -2}, Point{x: 1, y: -2}, Point{x: 2, y: -2}],
    &[Point{x: 0, y: 0}, Point{x: 0, y: -1}, Point{x: 0, y: -2}, Point{x: 0, y: -3}],
    &[Point{x: 0, y: 0}, Point{x: 0, y: -1}, Point{x: 1, y: 0}, Point{x: 1, y: -1}],
];

#[derive(Debug, Clone)]
struct Rock {
    pattern: &'static [Point],
    position: Point,
}

fn get_bottom_edge(points: &[Point]) -> i64 {
    points.iter().map(|p| p.y).min().unwrap()
}

impl Rock {
    fn new(pattern: &'static [Point], highest_point: i64) -> Self {
        let bottom_edge = get_bottom_edge(pattern);
        Self {
            pattern,
            position: Point::new(2, highest_point + 3 - bottom_edge),
        }
    }

    fn update_map(&self, map: &mut Grid<char>) {
        for p in self.pattern {
            map.cells.insert(self.position + *p, '#');
        }
    }

    fn move_if_possible(&mut self, map: &Grid<char>, f: fn(&mut Self)) -> bool {
        let mut copy = self.clone();
        f(&mut copy);
        let can_move = !copy.is_colliding(map);
        if can_move {
            *self = copy;
        }
        can_move
    }

    fn move_left(&mut self, map: &Grid<char>) -> bool {
        self.move_if_possible(map, |r| r.position.x -= 1)
    }

    fn move_right(&mut self, map: &Grid<char>) -> bool {
        self.move_if_possible(map, |r| r.position.x += 1)
    }

    fn move_down(&mut self, map: &Grid<char>) -> bool {
        self.move_if_possible(map, |r| r.position.y -= 1)
    }

    fn is_colliding(&self, map: &Grid<char>) -> bool {
        self.pattern.iter().any(|p| {
            let p = self.position + *p;
            map.cells.contains_key(&p) || p.x < 0 || p.x > 6 || p.y < 0
        })
    }
}

struct Env {
    map: Grid<char>,
    jet_pattern: Vec<char>,
    jet_position: i64,
    rocks_thrown: u64,
    laps: Vec<(u64, u64)>,
}

impl Env {
    fn new(jet_pattern: Vec<char>) -> Self {
        let map = Grid::new(7, 0, false);
        Self {
            map,
            jet_pattern,
            jet_position: 0,
            rocks_thrown: 0,
            laps: vec![],
        }
    }

    fn drop_rock(&mut self, pattern: usize) {
        self.rocks_thrown += 1;
        let mut rock = Rock::new(ROCKS_PATTERNS[pattern], self.map.height as i64);
        loop {
            let c = self.jet_pattern[self.jet_position as usize];
            match c {
                '<' => rock.move_left(&self.map),
                '>' => rock.move_right(&self.map),
                _ => panic!("Unknown jet pattern"),
            };
            self.jet_position += 1;
            if self.jet_position == self.jet_pattern.len() as i64 {
                self.laps.push((self.rocks_thrown, self.map.height));
                self.jet_position = 0;
            }
            if !rock.move_down(&self.map) {
                break;
            }
        }
        self.map.height = self.map.height.max(rock.position.y as u64 + 1);
        rock.update_map(&mut self.map);
    }
}

pub struct Day17 {
    jet_pattern: Vec<char>,
}

impl Day17 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let jet_pattern = utils::read_lines(filepath)[0].chars().collect();
        Box::new(Day17 { jet_pattern })
    }
}

impl Day for Day17 {
    fn solve_a(&self) -> u64 {
        let mut env = Env::new(self.jet_pattern.clone());
        for i in 0..2022 {
            env.drop_rock(i % 5);
        }
        env.map.height
    }

    fn solve_b(&self) -> u64 {
        let mut env = Env::new(self.jet_pattern.clone());
        let mut i = 0;
        while env.laps.len() < 10 {
            env.drop_rock(i % 5);
            i += 1;
        }
        let rocks_per_lap = env.laps[9].0 - env.laps[4].0;
        let height_per_lap = env.laps[9].1 - env.laps[4].1;
        let iterations_left = 1_000_000_000_000_u64 - env.rocks_thrown;
        let laps_performed = iterations_left / rocks_per_lap;
        let laps_left = iterations_left % rocks_per_lap;
        for _ in 0..laps_left {
            env.drop_rock(i % 5);
            i += 1;
        }

        env.map.height + laps_performed * height_per_lap
    }
}
