use advent_of_code::helpers::Stack;
use std::borrow::BorrowMut;
use std::str::FromStr;

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
pub struct Plan {
    initial_system: CraneSystem,
    rearrangement_procedure: Vec<Command>,
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

impl FromStr for Plan {
    type Err = plan_parser::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        plan_parser::parse(s)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let plan = input.parse::<Plan>().ok()?;
    Some(plan.apply(&CraneType::CrateMover9000))
}

pub fn part_two(input: &str) -> Option<String> {
    let plan = input.parse::<Plan>().ok()?;
    Some(plan.apply(&CraneType::CrateMover9001))
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

mod plan_parser {
    use super::*;
    use nom::{bytes, character, combinator, sequence, Finish, IResult};

    fn empty_item(input: &str) -> IResult<&str, Option<Item>> {
        let parser = nom::multi::count(character::complete::char(' '), 3);
        combinator::map(parser, |_| None)(input)
    }

    fn item(input: &str) -> IResult<&str, Option<Item>> {
        let parser = sequence::delimited(
            character::complete::char('['),
            character::complete::anychar,
            character::complete::char(']'),
        );
        combinator::map(parser, |c| Some(Item(c)))(input)
    }

    fn optional_item(input: &str) -> IResult<&str, Option<Item>> {
        nom::branch::alt((item, empty_item))(input)
    }

    fn item_line(input: &str) -> IResult<&str, Vec<Option<Item>>> {
        nom::multi::separated_list1(character::complete::char(' '), optional_item)(input)
    }

    fn crane_system(input: &str) -> IResult<&str, CraneSystem> {
        let parser = nom::multi::separated_list1(character::complete::newline, item_line);
        combinator::map(parser, |lines| {
            let mut stacks: Vec<Stack<Item>> = Vec::new();
            for line in lines.iter().rev() {
                stacks.resize(line.len(), Stack::new());
                for i in 0..line.len() {
                    if let Some(item) = &line[i] {
                        stacks[i].push(item.clone())
                    }
                }
            }
            CraneSystem { stacks }
        })(input)
    }

    fn separator(input: &str) -> IResult<&str, ()> {
        let parser = sequence::delimited(
            character::complete::space1,
            nom::multi::separated_list1(character::complete::space1, character::complete::digit1),
            character::complete::space1,
        );
        combinator::map(parser, |_| ())(input)
    }

    fn number(input: &str) -> IResult<&str, usize> {
        combinator::map_res(character::complete::digit1, str::parse::<usize>)(input)
    }

    fn command(input: &str) -> IResult<&str, Command> {
        let parser = sequence::tuple((
            bytes::complete::tag("move"),
            character::complete::space1,
            number,
            character::complete::space1,
            bytes::complete::tag("from"),
            character::complete::space1,
            number,
            character::complete::space1,
            bytes::complete::tag("to"),
            character::complete::space1,
            number,
        ));
        combinator::map(parser, |(_, _, cnt, _, _, _, frm, _, _, _, t)| Command {
            quantity: cnt,
            from_id: frm - 1,
            to_id: t - 1,
        })(input)
    }

    fn plan(input: &str) -> IResult<&str, Plan> {
        let parser = sequence::tuple((
            crane_system,
            character::complete::newline,
            separator,
            character::complete::newline,
            character::complete::newline,
            nom::multi::separated_list1(character::complete::newline, command),
        ));
        combinator::map(
            parser,
            |(initial_system, _, _, _, _, rearrangement_procedure)| Plan {
                initial_system,
                rearrangement_procedure,
            },
        )(input)
    }

    pub type Error = nom::error::Error<String>;

    pub fn parse(input: &str) -> Result<Plan, Error> {
        match plan(input).finish() {
            Ok((_remaining, plan)) => Ok(plan),
            Err(nom::error::Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn parse_item_line() {
            assert_eq!(
                item_line("[N] [C]    "),
                Ok(("", vec![Some(Item('N')), Some(Item('C')), None]))
            );
            assert_eq!(
                item_line("[Z] [M] [P]"),
                Ok(("", vec![Some(Item('Z')), Some(Item('M')), Some(Item('P'))]))
            );
        }

        #[test]
        fn parse_crane_system() {
            assert_eq!(
                crane_system(
                    "    [D]    
[N] [C]    
[Z] [M] [P]"
                ),
                Ok(("", CraneSystem::build(&["NZ", "DCM", "P"])))
            );
        }
    }

    #[test]
    fn parse_separator() {
        assert_eq!(
            separator(" 1   2   3   4   5   6   7   8   9 "),
            Ok(("", ()))
        );
    }

    #[test]
    fn parse_command() {
        assert_eq!(
            command("move 3 from 1 to 3"),
            Ok((
                "",
                Command {
                    quantity: 3,
                    from_id: 0,
                    to_id: 2
                }
            ))
        );
    }
}
