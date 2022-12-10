use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    NoOp,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::{anychar, space1},
            combinator::{map, map_opt},
            multi::many1,
            sequence::separated_pair,
            Finish,
        };
        let mut parser = alt((
            map_opt(
                separated_pair(tag("addx"), space1::<&str, _>, many1(anychar)),
                |(_, num_str)| {
                    Some(Instruction::AddX(
                        num_str.iter().collect::<String>().parse::<i32>().ok()?,
                    ))
                },
            ),
            map(tag("noop"), |_| Instruction::NoOp),
        ));
        match parser(s).finish() {
            Ok((_remaining, plan)) => Ok(plan),
            Err(nom::error::Error { input, code }) => Err(Self::Err {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    cycles: Vec<i32>,
}

impl Program {
    pub fn with_instructions(instructions: &[Instruction]) -> Program {
        let cycles = instructions.iter().fold(
            vec![1],
            |mut cycles, instr| {
                let register = *cycles.last().unwrap();
                cycles.push(register);
                match instr {
                    Instruction::NoOp => {}
                    Instruction::AddX(addx) => {
                        cycles.push(register + addx);
                    }
                }
                cycles
            },
        );
        Program { cycles }
    }

    pub fn signal_strength(&self, interesting_cycles: &[usize]) -> i32 {
        interesting_cycles
            .iter()
            .filter_map(|c| Some(self.cycles.get(c - 1)? * (*c as i32)))
            .sum()
    }

    pub fn crt_plot(&self) -> String {
        const COLUMNS: usize = 40;
        let rows = self.cycles.len() / COLUMNS;
        (0..rows)
            .map(|row| {
                (0..COLUMNS)
                    .map(|column| {
                        let signal: i32 = self.cycles[row * 40 + column];
                        if ((column as i32) - signal).abs() <= 1 {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

type Input = Vec<Instruction>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| l.parse::<Instruction>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = parse_input(input);
    let program = Program::with_instructions(&instructions);
    let strength = program.signal_strength(&[20, 60, 100, 140, 180, 220]);
    Some(strength)
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = parse_input(input);
    let program = Program::with_instructions(&instructions);
    Some(program.crt_plot())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_string();
        assert_eq!(part_two(&input), Some(expected));
    }
}
