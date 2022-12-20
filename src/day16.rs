use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use regex::Regex;

use crate::utils::{get_cap, read_lines, Day};

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u64,
    tunnels: Vec<String>,
}

pub struct Day16 {
    valves: HashMap<String, Valve>,
    non_zero_valves: u64,
    sorted_valve_sizes: Vec<u64>,
}

#[derive(Debug, Clone)]
struct Position {
    current: String,
    previous: String,
}

#[derive(Debug, Clone)]
struct State {
    positions: Vec<Position>,
    open_valves: HashSet<String>,
    final_flow: u64,
}

impl State {
    fn get_position_tuple(&self) -> ((String, String), (String, String)) {
        let mut second = ("".to_owned(), "".to_owned());
        let pos = &self.positions;
        if self.positions.len() > 1 {
            second = (pos[1].current.clone(), pos[1].previous.clone());
        }
        ((pos[0].current.clone(), pos[0].previous.clone()), second)
    }
}

impl Day16 {
    pub fn new(filepath: &Path) -> Box<dyn Day> {
        let valves: HashMap<_, _> = read_lines(filepath).iter().map(|l| parse_line(l)).collect();
        let non_zero_valves = valves.values().filter(|v| v.flow_rate > 0).count() as u64;
        let mut sorted_valve_sizes = valves.values().map(|v| v.flow_rate).collect::<Vec<_>>();
        sorted_valve_sizes.sort_by(|a, b| b.cmp(a));
        Box::new(Day16 {
            valves,
            non_zero_valves,
            sorted_valve_sizes,
        })
    }

    fn solve(&self, players: usize, initial_time_left: u64) -> u64 {
        let initial_position = Position {
            current: "AA".to_owned(),
            previous: "".to_owned(),
        };
        let initial_state = State {
            positions: vec![initial_position; players],
            final_flow: 0,
            open_valves: HashSet::new(),
        };
        let mut states = vec![initial_state];

        for time_left in (1..initial_time_left).rev() {
            states = self.compute_next_states(&states, 0, time_left);
            if players == 2 {
                states = self.compute_next_states(&states, 1, time_left);
            }
            states = self.prune_states(&states, time_left);
        }

        self.get_best(&states)
    }

    fn get_best(&self, states: &[State]) -> u64 {
        states.iter().map(|s| s.final_flow).max().unwrap()
    }

    fn get_max_potential(&self, time_left: u64) -> u64 {
        (0..=time_left)
            .rev()
            .step_by(2)
            .zip(&self.sorted_valve_sizes)
            .map(|(t, size)| t * size)
            .sum()
    }

    fn prune_states(&self, states: &[State], time_left: u64) -> Vec<State> {
        let current_best = self.get_best(states);
        let max_potential = self.get_max_potential(time_left);
        let mut best_positions: HashMap<_, State> = HashMap::new();
        for state in states.iter() {
            if state.final_flow + max_potential < current_best {
                continue;
            }
            let position = state.get_position_tuple();
            let best_state = best_positions.get(&position);
            if let Some(best_state) = best_state {
                if state.final_flow < best_state.final_flow {
                    continue;
                }
            }
            best_positions.insert(position, state.clone());
        }
        best_positions.values().cloned().collect()
    }

    fn compute_next_states(&self, states: &[State], index: usize, time_left: u64) -> Vec<State> {
        let mut next_states = vec![];
        for state in states.iter() {
            // if everything is open, just keep the state around
            if state.open_valves.len() == self.non_zero_valves as usize {
                next_states.push(state.clone());
                continue;
            }

            let position = &state.positions[index];
            let current_valve = &self.valves[&position.current];
            // if it makes sense to open the current valve, do it
            if current_valve.flow_rate > 0 && !state.open_valves.contains(&current_valve.name) {
                let mut potential_state = state.clone();
                potential_state
                    .open_valves
                    .insert(current_valve.name.clone());
                potential_state.positions[index].previous = current_valve.name.clone();
                potential_state.final_flow += current_valve.flow_rate * time_left;
                next_states.push(potential_state);
            }

            // go through all the tunnels, except if we just came from there
            for tunnel in current_valve.tunnels.iter() {
                if tunnel == &position.previous {
                    continue;
                }
                let mut potential_state = state.clone();
                potential_state.positions[index].current = tunnel.clone();
                potential_state.positions[index].previous = current_valve.name.clone();
                next_states.push(potential_state);
            }
        }

        next_states
    }
}

fn parse_line(line: &str) -> (String, Valve) {
    let re =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)")
            .unwrap();
    let caps = re.captures(line).unwrap();
    let valve = Valve {
        name: get_cap(&caps, 1),
        flow_rate: get_cap(&caps, 2),
        tunnels: get_cap::<String>(&caps, 3)
            .split(", ")
            .map(|s| s.to_string())
            .collect(),
    };
    (valve.name.clone(), valve)
}

impl Day for Day16 {
    fn solve_a(&self) -> u64 {
        self.solve(1, 30)
    }

    fn solve_b(&self) -> u64 {
        self.solve(2, 26)
    }
}
