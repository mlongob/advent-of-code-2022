use advent_of_code::helpers::Stack;
use std::{borrow::BorrowMut, collections::VecDeque};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Item(char);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Command {
    pub quantity: usize,
    pub from_id: usize,
    pub to_id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CraneSystem {
    stacks: Vec<Stack<Item>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

impl CraneSystem {
    fn crate_mover_9000(&mut self, command: &Command) -> usize {
        (0..command.quantity)
            .filter_map(|_| {
                let from_stack = self.stacks.get_mut(command.from_id)?;
                let item = from_stack.borrow_mut().pop()?;
                let to_stack = self.stacks.get_mut(command.to_id)?;
                to_stack.push(item);
                Some(())
            })
            .count()
    }

    fn crate_mover_9001(&mut self, command: &Command) -> usize {
        (|| {
            let from_stack = self.stacks.get_mut(command.from_id)?;
            let items = from_stack.pop_n(command.quantity);
            let to_stack = self.stacks.get_mut(command.to_id)?;
            let count = items.len();
            to_stack.push_n(items);
            Some(count)
        })()
        .unwrap_or(0)
    }

    pub fn apply(&mut self, crane_type: &CraneType, command: &Command) -> usize {
        match crane_type {
            CraneType::CrateMover9000 => self.crate_mover_9000(command),
            CraneType::CrateMover9001 => self.crate_mover_9001(command),
        }
    }

    pub fn top_items(&self) -> String {
        self.stacks
            .iter()
            .filter_map(Stack::top_item)
            .map(|i| i.0)
            .collect()
    }

    pub fn build(stacks_str: &[&str]) -> CraneSystem {
        let stacks = stacks_str
            .iter()
            .map(|stack_str| {
                let mut stack = Stack::new();
                stack_str.chars().rev().map(Item).for_each(|i| {
                    stack.push(i);
                });
                stack
            })
            .collect();
        CraneSystem { stacks }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Plan {
    initial_system: CraneSystem,
    rearrangement_procedure: Vec<Command>,
}

mod PlanParser {
    use nom::{bytes, character, combinator, sequence, IResult, Parser};

    fn empty_item(input: &str) -> IResult<&str, Option<super::Item>> {
        let parser = nom::multi::count(character::complete::char(' '), 3);
        combinator::map(parser, |_| None)(input)
    }

    fn item(input: &str) -> IResult<&str, Option<super::Item>> {
        let parser = sequence::delimited(
            character::complete::char('['),
            character::complete::anychar,
            character::complete::char(']'),
        );
        combinator::map(parser, |c| Some(super::Item(c)))(input)
    }

    fn optional_item(input: &str) -> IResult<&str, Option<super::Item>> {
        nom::branch::alt((item, empty_item))(input)
    }

    pub fn item_line(input: &str) -> IResult<&str, Vec<Option<super::Item>>> {
        nom::multi::separated_list1(character::complete::char(' '), optional_item)(input)
    }

    fn plan(input: &str) -> super::Plan {
        let initial_system = unimplemented!();
        let rearrangement_procedure = unimplemented!();
        super::Plan {
            initial_system,
            rearrangement_procedure,
        }
    }
}

impl Plan {
    pub fn apply(self, crane_type: &CraneType) -> String {
        let mut system = self.initial_system;
        self.rearrangement_procedure.iter().for_each(|command| {
            system.apply(crane_type, command);
        });
        system.top_items()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks, moves) = input.split_once("\n\n")?;
    let stacks: Vec<_> = stacks
        .lines()
        .map(|s| s.replace("[", " ").replace("]", " "))
        .rev()
        .collect();
    let stack_nums = stacks[0].split_whitespace().count();
    let stacks = &stacks[1..];

    let mut data: Vec<VecDeque<char>> = Vec::new();

    for _i in 0..stack_nums {
        data.push(VecDeque::new());
    }

    for stack in stacks {
        let stack: Vec<_> = stack.chars().collect();

        for i in 0..stack_nums {
            let c: char = stack[4 * i + 1];
            if c.is_alphanumeric() {
                data[i].push_front(c);
            }
        }
    }

    let moves: Vec<_> = moves
        .lines()
        .map(|l| {
            l.replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
        })
        .collect();

    for m in moves {
        let (num_s, rest) = m.split_once(" ")?;
        let (from_s, to_s) = rest.split_once(" ")?;
        let num = num_s.parse::<usize>().ok()?;
        let from = from_s.parse::<usize>().ok()?;
        let to = to_s.parse::<usize>().ok()?;
        for _ in 0..num {
            let from_queue = &mut data[from - 1];
            let item = from_queue.pop_front()?;
            let to_queue = &mut data[to - 1];
            to_queue.push_front(item);
        }
    }

    let result: String = data.iter().filter_map(|q| q.front()).collect();
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, moves) = input.split_once("\n\n")?;
    let stacks: Vec<_> = stacks
        .lines()
        .map(|s| s.replace("[", " ").replace("]", " "))
        .rev()
        .collect();
    let stack_nums = stacks[0].split_whitespace().count();
    let stacks = &stacks[1..];

    let mut data: Vec<VecDeque<char>> = Vec::new();

    for _i in 0..stack_nums {
        data.push(VecDeque::new());
    }

    for stack in stacks {
        let stack: Vec<_> = stack.chars().collect();

        for i in 0..stack_nums {
            let c: char = stack[4 * i + 1];
            if c.is_alphanumeric() {
                data[i].push_front(c);
            }
        }
    }

    let moves: Vec<_> = moves
        .lines()
        .map(|l| {
            l.replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
        })
        .collect();

    for m in moves {
        let (num_s, rest) = m.split_once(" ")?;
        let (from_s, to_s) = rest.split_once(" ")?;
        let num = num_s.parse::<usize>().ok()?;
        let from = from_s.parse::<usize>().ok()?;
        let to = to_s.parse::<usize>().ok()?;
        let from_queue = &mut data[from - 1];
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..num {
            let item = from_queue.pop_front()?;
            temp.push(item);
        }
        let to_queue = &mut data[to - 1];
        for item in temp.into_iter().rev() {
            to_queue.push_front(item);
        }
    }

    let result: String = data.iter().filter_map(|q| q.front()).collect();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_item_line() {
        assert_eq!(PlanParser::item_line("[N] [C]    "), Ok(("", vec![Some(Item('N'))])));
    }

    #[test]
    fn crane_system_build() {
        let crane_system = CraneSystem::build(&["NZ", "DCM", "P"]);
        dbg!(&crane_system);
        assert_eq!(crane_system.top_items(), "NDP".to_string());
    }

    #[test]
    fn crane_system_move_one_9000() {
        let mut crane_system = CraneSystem::build(&["NZ", "DCM", "P"]);
        crane_system.apply(
            &CraneType::CrateMover9000,
            &Command {
                quantity: 1,
                from_id: 1,
                to_id: 0,
            },
        );
        dbg!(&crane_system);
        assert_eq!(crane_system.top_items(), "DCP".to_string());
    }

    #[test]
    fn crane_system_move_one_9001() {
        let mut crane_system = CraneSystem::build(&["NZ", "DCM", "P"]);
        crane_system.apply(
            &CraneType::CrateMover9001,
            &Command {
                quantity: 1,
                from_id: 1,
                to_id: 0,
            },
        );
        dbg!(&crane_system);
        assert_eq!(crane_system.top_items(), "DCP".to_string());
    }

    #[test]
    fn crane_system_move_multi_9000() {
        let mut crane_system = CraneSystem::build(&["DNZ", "CM", "P"]);
        crane_system.apply(
            &CraneType::CrateMover9000,
            &Command {
                quantity: 3,
                from_id: 0,
                to_id: 2,
            },
        );
        dbg!(&crane_system);
        assert_eq!(crane_system.top_items(), "CZ".to_string());
    }

    #[test]
    fn crane_system_move_multi_9001() {
        let mut crane_system = CraneSystem::build(&["DNZ", "CM", "P"]);
        crane_system.apply(
            &CraneType::CrateMover9001,
            &Command {
                quantity: 3,
                from_id: 0,
                to_id: 2,
            },
        );
        dbg!(&crane_system);
        assert_eq!(crane_system.top_items(), "CD".to_string());
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
