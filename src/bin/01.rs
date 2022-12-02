use itertools::Itertools;
use std::cmp::Reverse;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Elf {
    snacks: Vec<u32>,
}

impl Elf {
    fn count_calories(&self) -> u32 {
        self.snacks.iter().sum()
    }
}

impl FromStr for Elf {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_snacks = s
            .lines()
            .map(|l| l.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Elf {
            snacks: parsed_snacks,
        })
    }
}

type Input = Vec<Elf>;

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    most_calories_carried(&parsed_input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    top3_calories_carried(&parsed_input)
}

fn parse_input(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|e| e.parse::<Elf>().unwrap())
        .collect()
}

fn most_calories_carried(input: &Input) -> Option<u32> {
    input.iter().map(Elf::count_calories).max()
}

fn top3_calories_carried(input: &Input) -> Option<u32> {
    Some(
        input
            .iter()
            .map(Elf::count_calories)
            .sorted_by_key(|c| Reverse(*c))
            .take(3)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_count_calories() {
        let elf = Elf {
            snacks: vec![7000, 8000, 9000],
        };
        assert_eq!(elf.count_calories(), 24000);
    }
    #[test]
    fn test_most_calories_carried() {
        let input = vec![
            Elf {
                snacks: vec![1000, 2000, 3000],
            },
            Elf { snacks: vec![4000] },
            Elf {
                snacks: vec![5000, 6000],
            },
            Elf {
                snacks: vec![7000, 8000, 9000],
            },
            Elf {
                snacks: vec![10000],
            },
        ];
        assert_eq!(most_calories_carried(&input), Some(24000));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
