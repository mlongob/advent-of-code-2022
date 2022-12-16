use anyhow::anyhow;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use z3::ast::Ast;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 500, y: 0 }
    }

    pub fn manhattan_distance(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct SensorReading {
    pub sensor: Position,
    pub beacon: Position,
}

impl SensorReading {
    pub fn strength(&self) -> i64 {
        self.sensor.manhattan_distance(&self.beacon)
    }
}

impl FromStr for SensorReading {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
        }
        let captures = RE.captures(s).ok_or(anyhow!("Could not match regex"))?;
        Ok(SensorReading {
            sensor: Position {
                x: captures
                    .name("sx")
                    .ok_or(anyhow!("Cannot extract Sensor[x]"))?
                    .as_str()
                    .parse::<i64>()?,
                y: captures
                    .name("sy")
                    .ok_or(anyhow!("Cannot extract Sensor[y]"))?
                    .as_str()
                    .parse::<i64>()?,
            },
            beacon: Position {
                x: captures
                    .name("bx")
                    .ok_or(anyhow!("Cannot extract Beacon[x]"))?
                    .as_str()
                    .parse::<i64>()?,
                y: captures
                    .name("by")
                    .ok_or(anyhow!("Cannot extract Beacon[y]"))?
                    .as_str()
                    .parse::<i64>()?,
            },
        })
    }
}

pub fn covered_per_row(input: &str, row_y: i64) -> Option<usize> {
    let mut beacons : HashSet<Position> = HashSet::new();
    let mut covered : HashSet<Position> = HashSet::new();
    input
        .lines()
        .filter_map(|l| l.parse::<SensorReading>().ok())
        .for_each(|SensorReading { sensor, beacon }| {
            let dist = sensor.manhattan_distance(&beacon);
            let mut marker = Position { x: sensor.x, y: row_y};
            while sensor.manhattan_distance(&marker) <= dist {
                covered.insert(marker.clone());
                marker.x += 1;
            }
            let mut marker = Position { x: sensor.x, y: row_y};
            while sensor.manhattan_distance(&marker) <= dist {
                covered.insert(marker.clone());
                marker.x -= 1;
            }
            if row_y == beacon.y {
                beacons.insert(beacon);
            }
        });
    Some(covered.difference(&beacons).count())
}

pub fn find_beacon_slow(input: &str, search_space: i64) -> Option<i64> {
    let mut candidates : HashSet<Position> = HashSet::new();
    let squares : Vec<(Position, i64)> = input
        .lines()
        .filter_map(|l| l.parse::<SensorReading>().ok())
        .map(|SensorReading { sensor, beacon }| {
            let dist = sensor.manhattan_distance(&beacon) + 1;
            for i in 0..=dist {
                let candidate = Position{x: sensor.x + dist - i, y: sensor.y + i};
                assert_eq!(candidate.manhattan_distance(&sensor), dist);
                if candidate.x >= 0 && candidate.x <= search_space && candidate.y >= 0 && candidate.y <= search_space {
                    candidates.insert(candidate);
                }
                let candidate = Position{x: sensor.x - dist + i, y: sensor.y - i};
                assert_eq!(candidate.manhattan_distance(&sensor), dist);
                if candidate.x >= 0 && candidate.x <= search_space && candidate.y >= 0 && candidate.y <= search_space {
                    candidates.insert(candidate);
                }
                let candidate = Position{x: sensor.x + i, y: sensor.y + dist - i};
                assert_eq!(candidate.manhattan_distance(&sensor), dist);
                if candidate.x >= 0 && candidate.x <= search_space && candidate.y >= 0 && candidate.y <= search_space {
                    candidates.insert(candidate);
                }
                let candidate = Position{x: sensor.x - i, y: sensor.y - dist + i};
                assert_eq!(candidate.manhattan_distance(&sensor), dist);
                if candidate.x >= 0 && candidate.x <= search_space && candidate.y >= 0 && candidate.y <= search_space {
                    candidates.insert(candidate);
                }
            }
            (sensor, dist - 1)
        }).collect();
    for candidate in candidates {
        if squares.iter().all(|(sensor, dist)| candidate.manhattan_distance(sensor) > *dist) {
            return Some(4000000*candidate.x + candidate.y);
        }
    }
    None
}

fn abs<'a>(val: z3::ast::Int<'a>) -> z3::ast::Int<'a> {
    let zero = z3::ast::Int::from_i64(val.get_ctx(), 0);
    val.gt(&zero).ite(&val, &(- &val))
}

fn within_range<'a>(val: &z3::ast::Int<'a>, low: i64, high: i64) -> z3::ast::Bool<'a> {
    let low = z3::ast::Int::from_i64(val.get_ctx(), low);
    let high = z3::ast::Int::from_i64(val.get_ctx(), high);
    z3::ast::Bool::and(val.get_ctx(), &[&val.ge(&low), &val.le(&high)])
}

pub fn find_beacon(input: &str, search_space: i64) -> Option<i64> {
    use z3::*;
    let ctx = Context::new(&Config::new());
    let goal_x = ast::Int::new_const(&ctx, "x");
    let goal_y = ast::Int::new_const(&ctx, "y");
    let solver = Solver::new(&ctx);
    solver.assert(&within_range(&goal_x, 0, search_space));
    solver.assert(&within_range(&goal_y, 0, search_space));
    for reading in input
        .lines()
        .filter_map(|l| l.parse::<SensorReading>().ok()) {
            let x = ast::Int::from_i64(&ctx, reading.sensor.x);
            let y = ast::Int::from_i64(&ctx, reading.sensor.y);
            let strength = ast::Int::from_i64(&ctx, reading.strength());
            solver.assert(&(abs(&goal_x - x) + abs(&goal_y - y)).gt(&strength));
    }
    if solver.check() != SatResult::Sat {
        return None
    }
    let model = solver.get_model()?;
    let xv = model.eval(&goal_x, true)?.as_i64()?;
    let yv = model.eval(&goal_y, true)?.as_i64()?;
    Some(4000000*xv + yv)
}

pub fn part_one(input: &str) -> Option<usize> {
    covered_per_row(input, 2000000)
}

pub fn part_two(input: &str) -> Option<i64> {
    find_beacon(input, 4000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(covered_per_row(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(find_beacon(&input, 20), Some(56000011));
    }
}
