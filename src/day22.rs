use std::path::Path;

use crate::{
    grid::{Grid as GenericGrid, Point},
    utils::{self, Day},
};

type Grid = GenericGrid<char>;

const ZONE_SIZE: i64 = 50;
const RIGHT: Point = Point { x: 1, y: 0 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const UP: Point = Point { x: 0, y: -1 };

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
    zones: Vec<Point>,
}

impl Env {
    fn new(grid: Grid) -> Env {
        let player = Player::from(&grid);
        let zones = Self::get_zones(&grid);
        Env {
            grid,
            player,
            zones,
        }
    }

    fn get_zones(_grid: &Grid) -> Vec<Point> {
        vec![
            Point::new(50, 0),
            Point::new(0, 150),
            Point::new(50, 100),
            Point::new(100, 0),
            Point::new(50, 50),
            Point::new(0, 100),
        ]
    }

    fn get_zone(&self, point: &Point) -> Option<usize> {
        self.zones.iter().position(|p| {
            (p.x..p.x + ZONE_SIZE).contains(&point.x) && (p.y..p.y + ZONE_SIZE).contains(&point.y)
        })
    }

    fn to_relative(&self, point: &Point, zone_id: usize) -> Point {
        let zone = self.zones[zone_id];
        Point::new(point.x - zone.x, point.y - zone.y)
    }

    fn to_absolute(&self, point: &Point, zone_id: usize) -> Point {
        let zone = self.zones[zone_id];
        Point::new(point.x + zone.x, point.y + zone.y)
    }

    fn get_transition(
        &self,
        zone_id: usize,
        direction: Point,
    ) -> (usize, Point, fn(Point) -> Point) {
        match (zone_id, direction) {
            (0, UP) => (1, RIGHT, |p| Point::new(p.y, p.x)),
            (0, LEFT) => (5, RIGHT, |p| Point::new(p.x, ZONE_SIZE - 1 - p.y)),
            (1, LEFT) => (0, DOWN, |p| Point::new(p.y, p.x)),
            (1, DOWN) => (3, DOWN, |p| Point::new(p.x, ZONE_SIZE - 1 - p.y)),
            (1, RIGHT) => (2, UP, |p| Point::new(p.y, p.x)),
            (2, DOWN) => (1, LEFT, |p| Point::new(p.y, p.x)),
            (2, RIGHT) => (3, LEFT, |p| Point::new(p.x, ZONE_SIZE - 1 - p.y)),
            (3, RIGHT) => (2, LEFT, |p| Point::new(p.x, ZONE_SIZE - 1 - p.y)),
            (3, DOWN) => (4, LEFT, |p| Point::new(p.y, p.x)),
            (3, UP) => (1, UP, |p| Point::new(p.x, ZONE_SIZE - 1 - p.y)),
            (4, RIGHT) => (3, UP, |p| Point::new(p.y, p.x)),
            (4, LEFT) => (5, DOWN, |p| Point::new(p.y, p.x)),
            (5, UP) => (4, RIGHT, |p| Point::new(p.y, p.x)),
            (5, LEFT) => (0, RIGHT, |p| Point::new(p.x, ZONE_SIZE - 1 - p.y)),
            _ => panic!("Unexpected transition: {} {}", zone_id, direction),
        }
    }

    fn get_next_state(&self, position: Point, direction: Point) -> (Point, char, Point) {
        let mut next_point = position;
        loop {
            let next_x = (next_point.x + direction.x).rem_euclid(self.grid.width as i64);
            let next_y = (next_point.y + direction.y).rem_euclid(self.grid.height as i64);
            next_point = Point::new(next_x, next_y);
            if let Some(c) = self.grid.get(&next_point) {
                break (next_point, *c, direction);
            }
        }
    }

    fn get_next_state_3d(&self, position: Point, direction: Point) -> (Point, char, Point) {
        let next_point_candidate = position + direction;
        let current_zone = self.get_zone(&position).unwrap();
        if self.get_zone(&next_point_candidate).is_some() {
            let value = self.grid.get(&next_point_candidate).unwrap();
            return (next_point_candidate, *value, direction);
        }
        let relative_point = self.to_relative(&position, current_zone);
        let (next_zone, new_direction, f) = self.get_transition(current_zone, direction);
        let next_point = self.to_absolute(&f(relative_point), next_zone);
        let value = self.grid.get(&next_point).unwrap();
        (next_point, *value, new_direction)
    }

    fn compute_next_state(&self, is_3d: bool) -> Option<Player> {
        let next_state_f = if is_3d {
            Env::get_next_state_3d
        } else {
            Env::get_next_state
        };
        match next_state_f(self, self.player.position, self.player.direction) {
            (p, '.', d) => Some(Player::new(p, d)),
            (_, '#', _) => None,
            (_, c, _) => panic!("Unexpected character: {}", c),
        }
    }

    fn execute_instructions(&mut self, instructions: &[Instruction], is_3d: bool) {
        for instr in instructions.iter() {
            self.execute_instruction(instr, is_3d);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction, is_3d: bool) {
        match instruction {
            Instruction::Turn(dir) => self.player.turn(*dir),
            Instruction::Advance(dist) => {
                for _ in 0..*dist {
                    match self.compute_next_state(is_3d) {
                        Some(player) => self.player = player,
                        None => break,
                    }
                }
            }
        }
    }
}

impl Player {
    fn new(position: Point, direction: Point) -> Player {
        Player { position, direction }
    }

    fn from(grid: &Grid) -> Player {
        let x_start = grid
            .cells
            .iter()
            .filter(|(p, c)| **c == '.' && p.y == 0)
            .map(|(p, _)| p.x)
            .min()
            .unwrap();
        Player {
            position: Point::new(x_start, 0),
            direction: RIGHT,
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
            RIGHT => 0,
            DOWN => 1,
            LEFT => 2,
            UP => 3,
            _ => panic!("Unexpected direction: {:?}", self.direction),
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
        let mut env = Env::new(self.grid.clone());
        env.execute_instructions(&self.instructions, false);
        env.player.compute_score()
    }

    fn solve_b(&self) -> u64 {
        let mut env = Env::new(self.grid.clone());
        env.execute_instructions(&self.instructions, true);
        env.player.compute_score()
    }
}
