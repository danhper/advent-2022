use std::{collections::HashSet, path::Path};

use crate::{
    grid::Point,
    utils::{self, get_cap, Day},
};
use regex::Regex;

pub struct Day15 {
    items: Vec<(Point, Point)>,
    is_test: bool,
}

impl Day15 {
    pub fn new(filepath: &Path, is_test: bool) -> Box<dyn Day> {
        let items = utils::read_lines(filepath)
            .iter()
            .map(|line| parse_line(line))
            .collect();
        Box::new(Self { items, is_test })
    }

    fn beacons_in_range(&self, start_x: i64, end_x: i64, height: i64) -> i64 {
        self.items
            .iter()
            .map(|(_, beacon)| beacon)
            .filter(|beacon| beacon.y == height && beacon.x >= start_x && beacon.x <= end_x)
            .collect::<HashSet<_>>()
            .len() as i64
    }

    fn compute_ranges(&self, height: i64, max: Option<i64>) -> Vec<(i64, i64)> {
        let ranges = self
            .items
            .iter()
            .filter_map(|(sensor, beacon)| compute_range_at(sensor, beacon, height));
        if let Some(max) = max {
            ranges
                .map(|(start, end)| (start.max(0), end.min(max)))
                .collect()
        } else {
            ranges.collect()
        }
    }
}

fn parse_line(line: &str) -> (Point, Point) {
    let r =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let caps = r.captures(line).unwrap();
    let sensor = Point::new(get_cap(&caps, 1), get_cap(&caps, 2));
    let beacon = Point::new(get_cap(&caps, 3), get_cap(&caps, 4));
    (sensor, beacon)
}

fn compute_range_at(sensor: &Point, beacon: &Point, height: i64) -> Option<(i64, i64)> {
    let distance = sensor.manhattan_distance(beacon) as i64;
    let vertical_distance = (sensor.y - height).abs();
    let diff = distance - vertical_distance;
    if diff < 0 {
        return None;
    }
    Some((sensor.x - diff, sensor.x + diff))
}

fn dedup_ranges(ranges: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut ranges = ranges.to_owned();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut deduped = Vec::new();
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;
    for (start, end) in ranges.iter().skip(1) {
        if *start <= current_end {
            current_end = current_end.max(*end);
        } else {
            deduped.push((current_start, current_end));
            current_start = *start;
            current_end = *end;
        }
    }
    deduped.push((current_start, current_end));
    deduped
}

impl Day for Day15 {
    fn solve_a(&self) -> u64 {
        let height = if self.is_test { 10 } else { 2000000 };
        let ranges = self.compute_ranges(height, None);
        let dedupped_ranges = dedup_ranges(&ranges);
        dedupped_ranges
            .iter()
            .map(|(start, end)| *end - *start + 1 - self.beacons_in_range(*start, *end, height))
            .sum::<i64>() as u64
    }

    fn solve_b(&self) -> u64 {
        let max = if self.is_test { 20 } else { 4_000_000 };
        let beacons = self
            .items
            .iter()
            .map(|(_, beacon)| beacon)
            .collect::<HashSet<_>>();
        for height in 0..=max {
            let ranges = self.compute_ranges(height, Some(max));
            let dedupped_ranges = dedup_ranges(&ranges);
            let mut previous_range = &dedupped_ranges[0];
            for range in dedupped_ranges.iter().skip(1) {
                for x in previous_range.1 + 1..range.0 {
                    let candidate = Point::new(x, height);
                    if !beacons.contains(&candidate) {
                        return (candidate.x * 4_000_000 + candidate.y) as u64;
                    }
                }
                previous_range = range;
            }
        }
        panic!("No solution found");
    }
}
