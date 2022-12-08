use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct Grid {
    pub width: u64,
    pub height: u64,
    pub cells: HashMap<(u64, u64), i64>,
}

impl Grid {
    pub fn from(lines: &[String]) -> Self {
        let mut cells = HashMap::new();
        let height = lines.len() as u64;
        let mut width = 0;
        for (y, line) in lines.iter().enumerate() {
            width = line.len() as u64;
            for (x, c) in line.chars().enumerate() {
                cells.insert((x as u64, y as u64), c.to_digit(10).unwrap() as i64);
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.cells.get(&(x as u64, y as u64)).unwrap();
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
