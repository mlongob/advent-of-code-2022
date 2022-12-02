use itertools::Itertools;
use std::str::FromStr;

type ParseError = String;

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scrissor,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scrissor => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scrissor),
            _ => Err(format!("Shape parse error: \"{s}\" not a valid shape")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round(Shape, Shape);

impl Round {
    fn outcome(&self) -> Outcome {
        match self {
            Round(a, b) if a == b => Outcome::Draw,
            Round(Shape::Rock, Shape::Scrissor)
            | Round(Shape::Paper, Shape::Rock)
            | Round(Shape::Scrissor, Shape::Paper) => Outcome::Loss,
            _ => Outcome::Win,
        }
    }

    fn score(&self) -> u32 {
        let outcome_score = self.outcome().score();
        let shape_score = self.1.score();
        shape_score + outcome_score
    }
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (movea, moveb) = s
            .split_whitespace()
            .map(|m| m.parse::<Shape>())
            .collect_tuple()
            .ok_or("Round Parse Error: expected 2 Shapes per round")?;
        Ok(Round(movea?, moveb?))
    }
}

#[derive(Debug)]
struct StrategyGuide {
    rounds: Vec<Round>,
}

impl StrategyGuide {
    fn score(&self) -> u32 {
        self.rounds.iter().map(Round::score).sum()
    }
}

impl FromStr for StrategyGuide {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_rounds = s
            .lines()
            .map(|l| l.parse::<Round>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            rounds: parsed_rounds,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let strategy_guide = input.parse::<StrategyGuide>().unwrap();
    Some(strategy_guide.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_outcome() {
        assert_eq!(Round(Shape::Rock, Shape::Scrissor).outcome(), Outcome::Loss);
        assert_eq!(Round(Shape::Rock, Shape::Rock).outcome(), Outcome::Draw);
        assert_eq!(Round(Shape::Rock, Shape::Paper).outcome(), Outcome::Win);
    }

    #[test]
    fn test_round_score() {
        assert_eq!(Round(Shape::Rock, Shape::Paper).score(), 8);
        assert_eq!(Round(Shape::Paper, Shape::Rock).score(), 1);
        assert_eq!(Round(Shape::Scrissor, Shape::Scrissor).score(), 6);
    }

    #[test]
    fn test_strategy_guide_score() {
        let guide = StrategyGuide {
            rounds: vec![
                Round(Shape::Rock, Shape::Paper),
                Round(Shape::Paper, Shape::Rock),
                Round(Shape::Scrissor, Shape::Scrissor),
            ],
        };
        assert_eq!(guide.score(), 15);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
