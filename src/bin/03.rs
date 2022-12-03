use std::str::FromStr;
use std::{collections::HashSet, ops::BitAnd};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Item(char);

impl Item {
    fn score(&self) -> u32 {
        let val = self.0 as u32;
        if self.0.is_uppercase() {
            val - ('A' as u32) + 27
        } else {
            val - ('a' as u32) + 1
        }
    }

    fn from_char(c: &char) -> Item {
        Item(*c)
    }
}

#[derive(Debug, Clone)]
struct Rucksack {
    items: HashSet<Item>,
}

impl FromIterator<Item> for Rucksack {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        Self {
            items: HashSet::from_iter(iter),
        }
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_iter(s.chars().map(|c| Item::from_char(&c))))
    }
}

impl Rucksack {
    fn common_sack(self, other: &Rucksack) -> Rucksack {
        Rucksack {
            items: self.items.bitand(&other.items),
        }
    }

    fn score(&self) -> u32 {
        self.items.iter().map(Item::score).sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let (one_str, two_str) = l.split_at(l.len() / 2);
                let one = one_str.parse::<Rucksack>().ok()?;
                let two = two_str.parse::<Rucksack>().ok()?;
                Some(one.common_sack(&two).score())
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    Some(
        lines
            .chunks(3)
            .filter_map(|chunks| {
                Some(
                    chunks
                        .iter()
                        .filter_map(|chunk| chunk.parse::<Rucksack>().ok())
                        .reduce(|accum, r| accum.common_sack(&r))?
                        .score(),
                )
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
