use std::{collections::HashSet, path::Path};

use crate::utils::{self, Day};

type Bound3D = ((i32, i32), (i32, i32), (i32, i32));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn get_neighbors(&self) -> Vec<Point3D> {
        vec![
            Self::new(self.x - 1, self.y, self.z),
            Self::new(self.x + 1, self.y, self.z),
            Self::new(self.x, self.y - 1, self.z),
            Self::new(self.x, self.y + 1, self.z),
            Self::new(self.x, self.y, self.z - 1),
            Self::new(self.x, self.y, self.z + 1),
        ]
    }

    fn is_within_bounds(&self, bounds: &Bound3D) -> bool {
        let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = bounds;
        self.x >= *min_x
            && self.x <= *max_x
            && self.y >= *min_y
            && self.y <= *max_y
            && self.z >= *min_z
            && self.z <= *max_z
    }
}

pub struct Day18 {
    points: HashSet<Point3D>,
    bounds: ((i32, i32), (i32, i32), (i32, i32)),
}

fn parse_line(line: &str) -> Point3D {
    let (x, y, z) = utils::split3(line, ",");
    Point3D { x, y, z }
}

fn get_min(points: &HashSet<Point3D>, f: fn(&Point3D) -> i32) -> i32 {
    points.iter().map(f).min().unwrap()
}

fn get_max(points: &HashSet<Point3D>, f: fn(&Point3D) -> i32) -> i32 {
    points.iter().map(f).max().unwrap()
}

impl Day18 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let points = utils::read_lines(filepath)
            .iter()
            .map(|line| parse_line(line))
            .collect();
        let (min_x, max_x) = (get_min(&points, |p| p.x), get_max(&points, |p| p.x));
        let (min_y, max_y) = (get_min(&points, |p| p.y), get_max(&points, |p| p.y));
        let (min_z, max_z) = (get_min(&points, |p| p.z), get_max(&points, |p| p.z));
        Box::new(Self {
            points,
            bounds: ((min_x, max_x), (min_y, max_y), (min_z, max_z)),
        })
    }

    fn is_trapped(&self, point: &Point3D) -> bool {
        let mut open_set = vec![*point];
        let mut seen = HashSet::new();
        while !open_set.is_empty() {
            let current = open_set.pop().unwrap();
            seen.insert(current);
            if !current.is_within_bounds(&self.bounds) {
                return false;
            }
            for neighbor in current.get_neighbors() {
                if !seen.contains(&neighbor) && !self.points.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }

        true
    }

    fn solve<P>(&self, predicate: P) -> u64
    where
        P: Fn(&Point3D) -> bool,
    {
        self.points
            .iter()
            .map(|p| p.get_neighbors().iter().filter(|p| predicate(p)).count())
            .sum::<usize>() as u64
    }
}

impl Day for Day18 {
    fn solve_a(&self) -> u64 {
        self.solve(|p| !self.points.contains(p))
    }

    fn solve_b(&self) -> u64 {
        self.solve(|p| !self.points.contains(p) && !self.is_trapped(p))
    }
}
