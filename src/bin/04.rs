use std::cmp;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Range(u32, u32);

impl FromStr for Range {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l1_s, l2_s) = s.split_once('-').ok_or("Cannot parse Range")?;
        let l1 = l1_s.parse::<u32>()?;
        let l2 = l2_s.parse::<u32>()?;
        Ok(Self(l1, l2))
    }
}

#[derive(Debug)]
struct ElfPair(Range, Range);

impl FromStr for ElfPair {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l1_s, l2_s) = s.split_once(',').ok_or("Cannot parse ElfPair")?;
        let l1 = l1_s.parse::<Range>()?;
        let l2 = l2_s.parse::<Range>()?;
        Ok(Self(l1, l2))
    }
}

impl ElfPair {
    fn fully_contain(&self) -> bool {
        (self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .1)
            || (self.1 .0 <= self.0 .0 && self.1 .1 >= self.0 .1)
    }

    fn overlap(&self) -> bool {
        cmp::max(self.0 .0, self.1 .0) <= cmp::min(self.0 .1, self.1 .1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let val: u32 = input
        .lines()
        .filter_map(|l| l.parse::<ElfPair>().ok())
        .filter(ElfPair::fully_contain)
        .count() as u32;
    Some(val)
}

pub fn part_two(input: &str) -> Option<u32> {
    let val: u32 = input
        .lines()
        .filter_map(|l| l.parse::<ElfPair>().ok())
        .filter(ElfPair::overlap)
        .count() as u32;
    Some(val)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
