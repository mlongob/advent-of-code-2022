use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationToken {
    Old,
    UnsignedInt(u64),
}

impl OperationToken {
    pub fn apply(&self, old: u64) -> u64 {
        match self {
            OperationToken::Old => old,
            OperationToken::UnsignedInt(n) => *n,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operation {
    pub lhs: OperationToken,
    pub operator: Operator,
    pub rhs: OperationToken,
}

impl Operation {
    pub fn apply(&self, old: u64) -> u64 {
        let lhs = self.lhs.apply(old);
        let rhs = self.rhs.apply(old);
        match self.operator {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide => lhs / rhs,
            Operator::Subtract => lhs - rhs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Test {
    pub divisible_by: u64,
    pub true_monkey_id: usize,
    pub false_monkey_id: usize,
}

impl Test {
    pub fn apply(&self, n: u64) -> usize {
        if n % self.divisible_by == 0 {
            self.true_monkey_id
        } else {
            self.false_monkey_id
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test: Test,
}

impl Monkey {
    pub fn receive_item(&mut self, item: u64) {
        self.items.push(item)
    }

    pub fn throw_item(&mut self, reducer: impl Fn(u64) -> u64) -> Option<(usize, u64)> {
        let mut item = self.items.pop()?;
        item = self.operation.apply(item);
        item = reducer(item);
        let monkey_id = self.test.apply(item);
        Some((monkey_id, item))
    }
}

pub struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
    inspect_counts: Vec<u64>,
}

impl MonkeyBusiness {
    pub fn with_monkeys(monkeys: Vec<Monkey>) -> MonkeyBusiness {
        let mut inspect_counts = Vec::new();
        inspect_counts.resize(monkeys.len(), 0);
        MonkeyBusiness {
            monkeys,
            inspect_counts,
        }
    }

    pub fn run_round(&mut self, reducer: impl Fn(u64) -> u64) {
        for throwing_monkey_id in 0..self.monkeys.len() {
            while let Some((receiving_monkey_id, item)) =
                self.monkeys[throwing_monkey_id].throw_item(&reducer)
            {
                assert_ne!(throwing_monkey_id, receiving_monkey_id);
                self.inspect_counts[throwing_monkey_id] += 1;
                self.monkeys[receiving_monkey_id].receive_item(item);
            }
        }
    }

    pub fn monkey_business_score(&self) -> u64 {
        const NUM_MONKEYS: usize = 2;
        self.inspect_counts
            .iter()
            .sorted()
            .rev()
            .take(NUM_MONKEYS)
            .fold(1, |acc, c| c * acc)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut mb = MonkeyBusiness::with_monkeys(input.parse::<input_parser::Input>().ok()?.monkeys);
    for _ in 0..20 {
        mb.run_round(|n| n / 3);
    }
    Some(mb.monkey_business_score())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut mb = MonkeyBusiness::with_monkeys(input.parse::<input_parser::Input>().ok()?.monkeys);
    let base: u64 = mb.monkeys.iter().map(|m| m.test.divisible_by).product();
    for _ in 0..10000 {
        mb.run_round(|n| n % base);
    }
    Some(mb.monkey_business_score())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}

mod input_parser {
    use super::{Monkey, Operation, OperationToken, Operator, Test};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, newline, space0, space1},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::tuple,
        Finish, IResult,
    };
    use std::str::FromStr;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Input {
        pub monkeys: Vec<Monkey>,
    }

    fn number_usize(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse::<usize>)(input)
    }

    fn number_u64(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse::<u64>)(input)
    }

    fn items(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(tuple((tag(","), space0)), number_u64)(input)
    }

    fn operation_token(input: &str) -> IResult<&str, OperationToken> {
        alt((
            map(tag("old"), |_| OperationToken::Old),
            map(number_u64, OperationToken::UnsignedInt),
        ))(input)
    }

    fn operator(input: &str) -> IResult<&str, Operator> {
        alt((
            map(tag("*"), |_| Operator::Multiply),
            map(tag("+"), |_| Operator::Add),
            map(tag("/"), |_| Operator::Divide),
            map(tag("-"), |_| Operator::Subtract),
        ))(input)
    }

    fn operation(input: &str) -> IResult<&str, Operation> {
        map(
            tuple((
                tag("new"),
                space0,
                tag("="),
                space0,
                operation_token,
                space0,
                operator,
                space0,
                operation_token,
            )),
            |(_, _, _, _, lhs, _, operator, _, rhs)| Operation { lhs, operator, rhs },
        )(input)
    }

    fn test(input: &str) -> IResult<&str, Test> {
        map(
            tuple((
                tag("divisible by"),
                space1,
                number_u64,
                newline,
                space0,
                tag("If true:"),
                space0,
                tag("throw to monkey"),
                space1,
                number_usize,
                newline,
                space0,
                tag("If false:"),
                space0,
                tag("throw to monkey"),
                space1,
                number_usize,
            )),
            |(
                _,
                _,
                divisible_by,
                _,
                _,
                _,
                _,
                _,
                _,
                true_monkey_id,
                _,
                _,
                _,
                _,
                _,
                _,
                false_monkey_id,
            )| Test {
                divisible_by,
                true_monkey_id,
                false_monkey_id,
            },
        )(input)
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        map(
            tuple((
                space0,
                tag("Starting items:"),
                space0,
                items,
                newline,
                space0,
                tag("Operation:"),
                space0,
                operation,
                newline,
                space0,
                tag("Test:"),
                space0,
                test,
            )),
            |(_, _, _, items, _, _, _, _, operation, _, _, _, _, test)| Monkey {
                items,
                operation,
                test,
            },
        )(input)
    }

    fn input(input: &str) -> IResult<&str, Input> {
        map(
            separated_list1(
                tuple((newline, newline)),
                map(
                    tuple((
                        space0,
                        tag("Monkey"),
                        space1,
                        number_usize,
                        space0,
                        tag(":"),
                        newline,
                        monkey,
                    )),
                    |t| t.7,
                ),
            ),
            |monkeys| Input { monkeys },
        )(input)
    }

    impl FromStr for Input {
        type Err = nom::error::Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match input(s).finish() {
                Ok((_remaining, plan)) => Ok(plan),
                Err(nom::error::Error { input, code }) => Err(Self::Err {
                    input: input.to_string(),
                    code,
                }),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_items() {
            assert_eq!(
                items("66, 51, 71, 76, 58, 55, 58, 60"),
                Ok(("", vec![66, 51, 71, 76, 58, 55, 58, 60]))
            );
        }

        #[test]
        fn parse_operation() {
            assert_eq!(
                operation("new = old * 88"),
                Ok((
                    "",
                    Operation {
                        lhs: OperationToken::Old,
                        operator: Operator::Multiply,
                        rhs: OperationToken::UnsignedInt(88)
                    }
                ))
            );
        }

        #[test]
        fn parse_test() {
            assert_eq!(
                test(
                    "divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3"
                ),
                Ok((
                    "",
                    Test {
                        divisible_by: 13,
                        true_monkey_id: 1,
                        false_monkey_id: 3,
                    }
                ))
            );
        }

        #[test]
        fn parse_monkey() {
            assert_eq!(
                monkey(
                    "  Starting items: 79, 52, 55, 51
                Operation: new = old + 6
                Test: divisible by 3
                  If true: throw to monkey 6
                  If false: throw to monkey 4"
                ),
                Ok((
                    "",
                    Monkey {
                        items: vec![79, 52, 55, 51],
                        operation: Operation {
                            lhs: OperationToken::Old,
                            operator: Operator::Add,
                            rhs: OperationToken::UnsignedInt(6)
                        },
                        test: Test {
                            divisible_by: 3,
                            true_monkey_id: 6,
                            false_monkey_id: 4
                        }
                    }
                ))
            );
        }

        #[test]
        fn parse_input() {
            assert_eq!(
                input(&advent_of_code::read_file("examples", 11)),
                Ok((
                    "",
                    Input {
                        monkeys: vec![
                            Monkey {
                                items: vec![79, 98],
                                operation: Operation {
                                    lhs: OperationToken::Old,
                                    operator: Operator::Multiply,
                                    rhs: OperationToken::UnsignedInt(19)
                                },
                                test: Test {
                                    divisible_by: 23,
                                    true_monkey_id: 2,
                                    false_monkey_id: 3
                                }
                            },
                            Monkey {
                                items: vec![54, 65, 75, 74],
                                operation: Operation {
                                    lhs: OperationToken::Old,
                                    operator: Operator::Add,
                                    rhs: OperationToken::UnsignedInt(6)
                                },
                                test: Test {
                                    divisible_by: 19,
                                    true_monkey_id: 2,
                                    false_monkey_id: 0
                                }
                            },
                            Monkey {
                                items: vec![79, 60, 97],
                                operation: Operation {
                                    lhs: OperationToken::Old,
                                    operator: Operator::Multiply,
                                    rhs: OperationToken::Old
                                },
                                test: Test {
                                    divisible_by: 13,
                                    true_monkey_id: 1,
                                    false_monkey_id: 3
                                }
                            },
                            Monkey {
                                items: vec![74],
                                operation: Operation {
                                    lhs: OperationToken::Old,
                                    operator: Operator::Add,
                                    rhs: OperationToken::UnsignedInt(3)
                                },
                                test: Test {
                                    divisible_by: 17,
                                    true_monkey_id: 0,
                                    false_monkey_id: 1
                                }
                            },
                        ]
                    }
                ))
            );
        }
    }
}
