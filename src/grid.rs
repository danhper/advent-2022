use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u64
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: u64,
    pub height: u64,
    pub going_down: bool,
    pub cells: HashMap<Point, T>,
    pub empty_cell: char,
}

pub fn get_neighbors(point: &Point, include_diagonals: bool) -> HashSet<Point> {
    let mut neighbors = HashSet::new();
    for x in point.x - 1..=point.x + 1 {
        for y in point.y - 1..=point.y + 1 {
            if (x == point.x && y == point.y)
                || !(include_diagonals || x == point.x || y == point.y)
            {
                continue;
            }
            neighbors.insert(Point::new(x, y));
        }
    }
    neighbors
}

impl<T> Grid<T> {
    pub fn new(width: u64, height: u64, going_down: bool) -> Self {
        Self {
            width,
            height,
            going_down,
            cells: HashMap::new(),
            empty_cell: '.',
        }
    }

    pub fn get_neighbors(&self, point: &Point, include_diagonals: bool) -> HashSet<Point> {
        get_neighbors(point, include_diagonals).into_iter().filter(|p| {
            p.x < self.width as i64 && p.y < self.height as i64
        }).collect()
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        self.cells.get(point)
    }
}

impl<T> Grid<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    pub fn from(lines: &[String]) -> Self {
        let mut cells = HashMap::new();
        let height = lines.len() as u64;
        let mut width = 0;
        for (y, line) in lines.iter().enumerate() {
            width = width.max(line.len() as u64);
            for (x, c) in line.chars().enumerate() {
                cells.insert(
                    Point::new(x as i64, y as i64),
                    c.to_string().parse().unwrap(),
                );
            }
        }

        Self {
            width,
            height,
            cells,
            going_down: true,
            empty_cell: '.',
        }
    }
}

impl<T: fmt::Display + Clone> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for mut y in 0..self.height {
            if !self.going_down {
                y = self.height - y - 1;
            }
            for x in 0..self.width {
                let c = self.cells.get(&Point::new(x as i64, y as i64)).cloned();
                if let Some(c) = c {
                    write!(f, "{}", c)?;
                } else {
                    write!(f, "{}", self.empty_cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Grid, Point};

    fn get_grid() -> Grid<char> {
        Grid::from(&[
            String::from("abcd"),
            String::from("efgh"),
            String::from("ijkl"),
            String::from("mnop"),
        ])
    }

    #[test]
    fn get_neighbors_no_diagonals() {
        let grid = get_grid();
        assert_eq!(
            grid.get_neighbors(&Point::new(2, 1), false),
            HashSet::from([
                Point::new(1, 1),
                Point::new(2, 0),
                Point::new(2, 2),
                Point::new(3, 1),
            ])
        );
    }

    #[test]
    fn get_neighbors_diagonals() {
        let grid = get_grid();
        assert_eq!(
            grid.get_neighbors(&Point::new(2, 1), true),
            HashSet::from([
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(1, 1),
                Point::new(3, 1),
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(3, 2),
            ])
        );
    }

    #[test]
    fn add_points() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        assert_eq!(p1 + p2, Point::new(4, 6));
    }
}
