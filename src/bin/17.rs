use anyhow::anyhow;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shift {
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pattern {
    shifts: Vec<Shift>
}

impl FromStr for Pattern {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shifts = s.chars().map(|c| match c {
            '<' => Ok(Shift::Left),
            '>' => Ok(Shift::Right),
            _ => Err(anyhow!("'{c}' is not a valid Shift"))
        }).collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Pattern { shifts })
    }
}

impl Pattern {
    pub fn iter(&self) -> impl Iterator<Item = &Shift> {
        self.shifts.iter().cycle()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = input.parse::<Pattern>().ok()?;
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
