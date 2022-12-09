use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
        }
    }

    pub fn touches(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    pub fn follow(&mut self, head: &Position) {
        if !self.touches(head) {
            let offset = if head.x == self.x || head.y == self.y {
                1
            } else {
                0
            };
            if head.y > self.y + offset {
                self.step(&Direction::Up);
            }
            if head.y < self.y - offset {
                self.step(&Direction::Down);
            }
            if head.x > self.x + offset {
                self.step(&Direction::Right);
            }
            if head.x < self.x - offset {
                self.step(&Direction::Left);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("{s}: Unkown direction"))?,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub direction: Direction,
    pub steps: u32,
}

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_s, steps_s) = s.split_once(' ').ok_or("Did not find two arguments")?;
        let direction = dir_s.parse()?;
        let steps = steps_s.parse()?;
        Ok(Move { direction, steps })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    pub fn with_knots(num_knots: usize) -> Rope {
        assert!(
            num_knots >= 2,
            "Rope::with_knots called with less than 2 knots"
        );
        let mut knots = Vec::with_capacity(num_knots);
        knots.resize(num_knots, Position::new());
        Rope { knots }
    }

    pub fn do_move(&mut self, direction: &Direction) {
        let head = self.knots.first_mut().unwrap();
        head.step(direction);
        self.knots.iter_mut().reduce(|prev, knot| {
            knot.follow(prev);
            knot
        });
    }

    pub fn tail(&self) -> &Position {
        &self.knots.last().unwrap()
    }
}

type Input = Vec<Move>;

pub fn solve(input: &str, rope_len: usize) -> Option<u32> {
    let input: Input = input
        .lines()
        .filter_map(|l| l.parse::<Move>().ok())
        .collect();
    let mut rope = Rope::with_knots(rope_len);
    let mut visited_positions: HashSet<Position> = HashSet::new();

    for Move { direction, steps } in input {
        for _ in 0..steps {
            rope.do_move(&direction);
            visited_positions.insert(rope.tail().clone());
        }
    }
    Some(visited_positions.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
