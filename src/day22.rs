use std::path::Path;

use crate::{
    grid::{Grid as GenericGrid, Point},
    utils::{self, Day},
};

type Grid = GenericGrid<char>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    Turn(Direction),
    Advance(u64),
}

#[derive(Debug)]
struct Player {
    position: Point,
    direction: Point,
}

#[derive(Debug)]
struct Env {
    grid: Grid,
    player: Player,
}

impl Env {
    fn get_next_point(&self, position: Point, direction: Point) -> (Point, char) {
        let mut next_point = position;
        loop {
            let next_x = (next_point.x + direction.x).rem_euclid(self.grid.width as i64);
            let next_y = (next_point.y + direction.y).rem_euclid(self.grid.height as i64);
            next_point = Point::new(next_x, next_y);
            if let Some(c) = self.grid.get(&next_point) {
                break (next_point, *c);
            }
        }
    }

    fn find_next_point(&self) -> Option<Point> {
        match self.get_next_point(self.player.position, self.player.direction) {
            (p, '.') => Some(p),
            (_, '#') => None,
            (_, c) => panic!("Unexpected character: {}", c),
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Turn(dir) => self.player.turn(*dir),
            Instruction::Advance(dist) => {
                for _ in 0..*dist {
                    match self.find_next_point() {
                        Some(next_point) => self.player.position = next_point,
                        None => break,
                    }
                }
            }
        }
    }
}

impl Player {
    fn new(grid: &Grid) -> Player {
        let x_start = grid
            .cells
            .iter()
            .filter(|(p, c)| **c == '.' && p.y == 0)
            .map(|(p, _)| p.x)
            .min()
            .unwrap();
        Player {
            position: Point::new(x_start, 0),
            direction: Point::new(1, 0),
        }
    }

    fn turn(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.direction = Point::new(self.direction.y, -self.direction.x),
            Direction::Right => self.direction = Point::new(-self.direction.y, self.direction.x),
        }
    }

    fn compute_score(&self) -> u64 {
        // 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)
        let direction_score = match self.direction {
            Point { x: 1, y: 0 } => 0,
            Point { x: 0, y: 1 } => 1,
            Point { x: -1, y: 0 } => 2,
            Point { x: 0, y: -1 } => 3,
            _ => panic!("Unexpected position: {:?}", self.position),
        };
        (1000 * (self.position.y + 1) + 4 * (self.position.x + 1) + direction_score) as u64
    }
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut current_num = 0;
    for c in line.chars() {
        match c {
            'L' | 'R' => {
                instructions.push(Instruction::Advance(current_num));
                current_num = 0;
                let dir = match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unexpected character: {}", c),
                };
                instructions.push(Instruction::Turn(dir));
            }
            '0'..='9' => {
                current_num = current_num * 10 + (c as u64 - '0' as u64);
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }
    instructions.push(Instruction::Advance(current_num));
    instructions
}

pub struct Day22 {
    grid: Grid,
    instructions: Vec<Instruction>,
}

impl Day22 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let lines = utils::read_lines(filepath);
        let (grid_lines, rest) = lines.split_at(lines.len() - 2);
        let instructions = parse_instructions(&rest[1]);
        let mut grid = Grid::from(grid_lines);
        grid.cells.retain(|_, c| *c == '.' || *c == '#');
        grid.empty_cell = ' ';
        Box::new(Day22 { grid, instructions })
    }
}

impl Day for Day22 {
    fn solve_a(&self) -> u64 {
        let player = Player::new(&self.grid);
        let mut env = Env {
            grid: self.grid.clone(),
            player,
        };
        for instr in self.instructions.iter() {
            env.execute_instruction(instr);
        }
        env.player.compute_score()
    }

    fn solve_b(&self) -> u64 {
        0
    }
}
