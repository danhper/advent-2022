use std::{collections::HashSet, path::Path};

use crate::utils::{captures, get_cap, read_lines, Day};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Materials {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

impl Materials {
    fn new(ore: u64) -> Materials {
        Materials {
            ore,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn add(&mut self, other: &Materials) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }

    fn modify_clone(&self, f: fn(&mut Materials)) -> Materials {
        let mut materials = *self;
        f(&mut materials);
        materials
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    materials: Materials,
    robots: Materials,
    next: Option<Materials>,
}

impl State {
    fn new() -> State {
        State {
            materials: Materials::new(0),
            robots: Materials::new(1),
            next: None,
        }
    }

    fn advance(&mut self) {
        self.materials.add(&self.robots);
        if let Some(robots) = self.next {
            self.robots = robots;
            self.next = None;
        }
    }

    fn get_next_states(&self, blueprint: &Blueprint) -> Vec<State> {
        let mut states = vec![];

        if self.materials.ore >= blueprint.geode.0 && self.materials.obsidian >= blueprint.geode.1 {
            let mut state = self.clone();
            state.materials.ore -= blueprint.geode.0;
            state.materials.obsidian -= blueprint.geode.1;
            state.next = Some(self.robots.modify_clone(|robots| robots.geode += 1));
            states.push(state);
            return states;
        }

        if self.materials.ore < blueprint.ore {
            states.push(self.clone());
        }

        if self.materials.ore >= blueprint.obsidian.0
            && self.materials.clay >= blueprint.obsidian.1
            && self.robots.obsidian < blueprint.geode.1
        {
            let mut state = self.clone();
            state.materials.ore -= blueprint.obsidian.0;
            state.materials.clay -= blueprint.obsidian.1;
            state.next = Some(self.robots.modify_clone(|robots| robots.obsidian += 1));
            states.push(state);
        }

        if self.materials.ore >= blueprint.ore && self.robots.ore < blueprint.max_ore_robots() {
            let mut state = self.clone();
            state.materials.ore -= blueprint.ore;
            state.next = Some(self.robots.modify_clone(|robots| robots.ore += 1));
            states.push(state);
        }

        if self.materials.ore >= blueprint.clay && self.robots.clay < blueprint.obsidian.1 {
            let mut state = self.clone();
            state.materials.ore -= blueprint.clay;
            state.next = Some(self.robots.modify_clone(|robots| robots.clay += 1));
            states.push(state);
        }

        states
    }
}

#[derive(Debug)]
struct Blueprint {
    ore: u64,
    clay: u64,
    obsidian: (u64, u64),
    geode: (u64, u64),
}

impl Blueprint {
    fn compute_best_score(&self, n: usize) -> u64 {
        let mut states = HashSet::from([State::new()]);

        for _ in 0..n {
            let mut new_states = HashSet::new();
            for mut state in states.into_iter() {
                state.advance();
                new_states.extend(state.get_next_states(self).into_iter());
            }
            states = new_states;
        }

        states
            .iter()
            .map(|state| state.materials.geode)
            .max()
            .unwrap()
    }

    fn max_ore_robots(&self) -> u64 {
        self.clay.max(self.obsidian.0).max(self.geode.0)
    }
}

pub struct Day19 {
    blueprints: Vec<Blueprint>,
}

impl Day19 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let blueprints = read_lines(filepath)
            .iter()
            .map(|line| parse_line(line))
            .collect();
        Box::new(Day19 { blueprints })
    }
}

fn parse_line(line: &str) -> Blueprint {
    let obsidian = captures(r"obsidian robot costs (\d+) ore and (\d+) clay", line);
    let geode = captures(r"geode robot costs (\d+) ore and (\d+) obsidian", line);
    Blueprint {
        ore: get_cap(&captures(r"ore robot costs (\d+)", line), 1),
        clay: get_cap(&captures(r"clay robot costs (\d+)", line), 1),
        obsidian: (get_cap(&obsidian, 1), get_cap(&obsidian, 2)),
        geode: (get_cap(&geode, 1), get_cap(&geode, 2)),
    }
}

impl Day for Day19 {
    fn solve_a(&self) -> u64 {
        self.blueprints
            .iter()
            .enumerate()
            .map(|(i, b)| (i as u64 + 1) * b.compute_best_score(24))
            .sum()
    }

    fn solve_b(&self) -> u64 {
        self.blueprints
            .iter()
            .take(3)
            .map(|b| b.compute_best_score(32))
            .product()
    }
}
