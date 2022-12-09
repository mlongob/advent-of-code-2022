use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn step(&mut self, direction: &Direction) {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
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
struct Move {
    direction: Direction,
    steps: u32,
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

fn rope_physics(head: &Position, tail: Position) -> Position
{
    tail
}

type Input = Vec<Move>;

pub fn part_one(input: &str) -> Option<u32> {
    let input: Input = input
        .lines()
        .filter_map(|l| l.parse::<Move>().ok())
        .collect();
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    let mut visited_positions: HashSet<Position> = HashSet::new();

    for Move { direction, steps } in input {
        for _ in 0..steps {
            head_position.step(&direction);
            tail_position = rope_physics(&head_position, tail_position);
            visited_positions.insert(tail_position.clone());
        }
    }
    Some(visited_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
