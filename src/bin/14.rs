use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::iter::from_fn;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn new() -> Position {
        Position { x: 500, y: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Object {
    Rock,
    Sand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    objects: HashMap<Position, Object>,
    limits: (Position, Position),
    floor: Option<i32>,
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::new();
        s.lines().for_each(|l| {
            l.split(" -> ")
                .filter_map(|p| p.split_once(','))
                .filter_map(|(x_s, y_s)| {
                    Some(Position {
                        x: x_s.parse::<i32>().ok()?,
                        y: y_s.parse::<i32>().ok()?,
                    })
                })
                .tuple_windows()
                .for_each(|(start, end)| {
                    for x in start.x.min(end.x)..=start.x.max(end.x) {
                        for y in start.y.min(end.y)..=start.y.max(end.y) {
                            grid.add_object(Position { x, y }, Object::Rock);
                        }
                    }
                })
        });
        Ok(grid)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.limits.0.y..=self.y_limit() {
            for x in self.limits.0.x..=self.limits.1.x {
                let position = Position { x, y };
                if position == Position::new() {
                    write!(f, "+")?;
                    continue;
                }
                if self.floor.is_some() && position.y == self.y_limit() {
                    write!(f, "#")?;
                    continue;
                }
                match self.objects.get(&position) {
                    Some(Object::Rock) => write!(f, "#")?,
                    Some(Object::Sand) => write!(f, "o")?,
                    _ => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            objects: HashMap::new(),
            limits: (Position::new(), Position::new()),
            floor: None,
        }
    }

    fn sand_fall(&self, position: &Position) -> Option<Position> {
        let res = [
            Position {
                x: position.x,
                y: position.y + 1,
            },
            Position {
                x: position.x - 1,
                y: position.y + 1,
            },
            Position {
                x: position.x + 1,
                y: position.y + 1,
            },
        ]
        .into_iter()
        .find(|p| !self.objects.contains_key(p))?;
        if self.floor.is_some() && res.y >= self.y_limit() {
            None
        } else {
            Some(res)
        }
    }

    pub fn add_object(&mut self, position: Position, object: Object) {
        self.limits.0.x = self.limits.0.x.min(position.x);
        self.limits.0.y = self.limits.0.y.min(position.y);
        self.limits.1.x = self.limits.1.x.max(position.x);
        self.limits.1.y = self.limits.1.y.max(position.y);
        self.objects.insert(position, object);
    }

    pub fn add_sand(&mut self) -> Option<()> {
        let mut grain = Position::new();
        while let Some(new_pos) = self.sand_fall(&grain) {
            if new_pos.y > self.y_limit() && self.floor.is_none() {
                return None;
            }
            grain = new_pos;
        }
        if grain == Position::new() {
            return None;
        }
        self.add_object(grain, Object::Sand);
        Some(())
    }

    pub fn set_floor(&mut self, floor_delta: usize) {
        self.floor = Some(self.limits.1.y + (floor_delta as i32));
    }

    fn y_limit(&self) -> i32 {
        self.floor.unwrap_or(self.limits.1.y)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Grid = input.parse().ok()?;
    let grains_of_sand = from_fn(|| grid.add_sand()).count();
    Some(grains_of_sand)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Grid = input.parse().ok()?;
    grid.set_floor(2);
    let grains_of_sand = from_fn(|| grid.add_sand()).count();
    Some(grains_of_sand + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
