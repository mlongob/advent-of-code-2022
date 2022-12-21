use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expression {
    Num(u64),
    Sum(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MonkeyMath {
    expressions: HashMap<String, Expression>,
    result_cache: RefCell<HashMap<String, u64>>,
}

impl MonkeyMath {
    pub fn new() -> MonkeyMath {
        MonkeyMath {
            expressions: HashMap::new(),
            result_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_expressions(expressions: HashMap<String, Expression>) -> MonkeyMath {
        MonkeyMath {
            expressions,
            result_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn eval(&self, monkey: &String) -> Option<u64> {
        if let Some(result) = self.result_cache.borrow().get(monkey) {
            return Some(*result)
        }

        let expr = self.expressions.get(monkey)?;
        let result = match expr {
            Expression::Num(n) => *n,
            Expression::Sum(a, b) => self.eval(a)? + self.eval(b)?,
            Expression::Sub(a, b) => self.eval(a)? - self.eval(b)?,
            Expression::Mul(a, b) => self.eval(a)? * self.eval(b)?,
            Expression::Div(a, b) => self.eval(a)? / self.eval(b)?,
        };
        self.result_cache
            .borrow_mut()
            .insert(monkey.clone(), result);
        Some(result)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkey_math = input.parse::<MonkeyMath>().ok()?;
    monkey_math.eval(&"root".to_string())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}

mod input_parser {
    use super::{Expression, MonkeyMath};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, digit1, space0, newline},
        combinator::{map, map_res},
        multi::{separated_list0},
        sequence::{tuple},
        Finish, IResult,
    };
    use std::str::FromStr;

    fn monkey_id(input: &str) -> IResult<&str, String> {
        map(alpha1, |s: &str| s.to_string())(input)
    }

    fn expr_num(input: &str) -> IResult<&str, Expression> {
        map(map_res(digit1, |s: &str| s.parse::<u64>()), Expression::Num)(input)
    }

    fn expr_sum(input: &str) -> IResult<&str, Expression> {
        map(tuple((
            monkey_id,
            space0,
            tag("+"),
            space0,
            monkey_id)),
            |(a, _, _, _, b)| Expression::Sum(a, b))(input)
    }

    fn expr_sub(input: &str) -> IResult<&str, Expression> {
        map(tuple((
            monkey_id,
            space0,
            tag("-"),
            space0,
            monkey_id)),
            |(a, _, _, _, b)| Expression::Sub(a, b))(input)
    }

    fn expr_mul(input: &str) -> IResult<&str, Expression> {
        map(tuple((
            monkey_id,
            space0,
            tag("*"),
            space0,
            monkey_id)),
            |(a, _, _, _, b)| Expression::Mul(a, b))(input)
    }

    fn expr_div(input: &str) -> IResult<&str, Expression> {
        map(tuple((
            monkey_id,
            space0,
            tag("/"),
            space0,
            monkey_id)),
            |(a, _, _, _, b)| Expression::Div(a, b))(input)
    }

    fn expr(input: &str) -> IResult<&str, Expression> {
        alt((expr_num, expr_sum, expr_sub, expr_mul, expr_div))(input)
    }

    fn monkey_assignment(input: &str) -> IResult<&str, (String, Expression)> {
        map(
            tuple((monkey_id, space0, tag(":"), space0, expr)),
        |(monkey, _, _, _, ex)| (monkey, ex))(input)
    }

    fn monkey_math(input: &str) -> IResult<&str, MonkeyMath> {
        map(separated_list0(newline, monkey_assignment), |vs| {
            MonkeyMath::with_expressions(vs.into_iter().collect())
        })(input)
    }

    impl FromStr for MonkeyMath {
        type Err = nom::error::Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match monkey_math(s).finish() {
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
        fn parse_monkey_math() {
            let input = &advent_of_code::read_file("examples", 21);
            assert_eq!(
                monkey_math(input),
                Ok((
                    "",
                    MonkeyMath::with_expressions(std::collections::HashMap::from([
                        // root: pppw + sjmn
                        (String::from("root"), Expression::Sum(String::from("pppw"), String::from("sjmn"))),
                        // dbpl: 5
                        (String::from("dbpl"), Expression::Num(5)),
                        // cczh: sllz + lgvd
                        (String::from("cczh"), Expression::Sum(String::from("sllz"), String::from("lgvd"))),
                        // zczc: 2
                        (String::from("zczc"), Expression::Num(2)),
                        // ptdq: humn - dvpt
                        (String::from("ptdq"), Expression::Sub(String::from("humn"), String::from("dvpt"))),
                        // dvpt: 3
                        (String::from("dvpt"), Expression::Num(3)),
                        // lfqf: 4
                        (String::from("lfqf"), Expression::Num(4)),
                        // humn: 5
                        (String::from("humn"), Expression::Num(5)),
                        // ljgn: 2
                        (String::from("ljgn"), Expression::Num(2)),
                        // sjmn: drzm * dbpl
                        (String::from("sjmn"), Expression::Mul(String::from("drzm"), String::from("dbpl"))),
                        // sllz: 4
                        (String::from("sllz"), Expression::Num(4)),
                        // pppw: cczh / lfqf
                        (String::from("pppw"), Expression::Div(String::from("cczh"), String::from("lfqf"))),
                        // lgvd: ljgn * ptdq
                        (String::from("lgvd"), Expression::Mul(String::from("ljgn"), String::from("ptdq"))),
                        // drzm: hmdt - zczc
                        (String::from("drzm"), Expression::Sub(String::from("hmdt"), String::from("zczc"))),
                        // hmdt: 32
                        (String::from("hmdt"), Expression::Num(32)),
                    ]))
                ))
            );
        }
    }
}
