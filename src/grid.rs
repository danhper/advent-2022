use std::{
    collections::{HashMap, HashSet},
    fmt,
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: u64,
    pub height: u64,
    pub cells: HashMap<Point, T>,
}

impl<T> Grid<T> {
    pub fn get_neighbors(&self, point: &Point, include_diagonals: bool) -> HashSet<Point> {
        let (width, height) = (self.width as i64, self.height as i64);
        let mut neighbors = HashSet::new();
        for x in point.x.saturating_sub(1)..=point.x + 1 {
            for y in point.y.saturating_sub(1)..=point.y + 1 {
                if (x == point.x && y == point.y)
                    || !(include_diagonals || x == point.x || y == point.y)
                {
                    continue;
                }
                if x < width && y < height {
                    neighbors.insert(Point::new(x, y));
                }
            }
        }
        neighbors
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
            width = line.len() as u64;
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
        }
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.cells.get(&Point::new(x as i64, y as i64)).unwrap();
                write!(f, "{}", c)?;
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
}
