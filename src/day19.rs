use std::{collections::HashSet, path::Path};

use crate::utils::{captures, get_cap, read_lines, Day};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,

    ore_robot_pending: bool,
    clay_robot_pending: bool,
    obsidian_robot_pending: bool,
    geode_robot_pending: bool,

    ore_robots: u64,
    clay_robots: u64,
    obsidian_robots: u64,
    geode_robots: u64,
}

impl State {
    fn new() -> State {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            ore_robot_pending: false,
            clay_robot_pending: false,
            obsidian_robot_pending: false,
            geode_robot_pending: false,
        }
    }

    fn advance(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        if self.geode_robot_pending {
            self.geode_robots += 1;
            self.geode_robot_pending = false;
        }
        if self.clay_robot_pending {
            self.clay_robots += 1;
            self.clay_robot_pending = false;
        }
        if self.obsidian_robot_pending {
            self.obsidian_robots += 1;
            self.obsidian_robot_pending = false;
        }
        if self.ore_robot_pending {
            self.ore_robots += 1;
            self.ore_robot_pending = false;
        }
    }

    fn get_next_states(&self, blueprint: &Blueprint) -> Vec<State> {
        let mut states = Vec::new();

        if self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1 {
            let mut state = self.clone();
            state.ore -= blueprint.geode.0;
            state.obsidian -= blueprint.geode.1;
            state.geode_robot_pending = true;
            states.push(state);
            return states;
        }

        if self.ore < blueprint.ore
            || self.ore < blueprint.clay
            || self.ore < blueprint.obsidian.0
            || self.clay < blueprint.obsidian.1
        {
            states.push(self.clone());
        }

        if self.ore >= blueprint.obsidian.0
            && self.clay >= blueprint.obsidian.1
            && self.obsidian_robots < blueprint.geode.1
        {
            let mut state = self.clone();
            state.ore -= blueprint.obsidian.0;
            state.clay -= blueprint.obsidian.1;
            state.obsidian_robot_pending = true;
            states.push(state);
        }

        if self.ore >= blueprint.ore && self.ore_robots < blueprint.max_ore_robots() {
            let mut state = self.clone();
            state.ore -= blueprint.ore;
            state.ore_robot_pending = true;
            states.push(state);
        }

        if self.ore >= blueprint.clay && self.clay_robots < blueprint.obsidian.1 {
            let mut state = self.clone();
            state.ore -= blueprint.clay;
            state.clay_robot_pending = true;
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

        states.iter().map(|state| state.geode).max().unwrap()
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
