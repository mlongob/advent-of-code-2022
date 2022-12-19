use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Resource {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "ore" => Self::Ore,
            "clay" => Self::Clay,
            "obsidian" => Self::Obsidian,
            "geode" => Self::Geode,
            _ => anyhow::bail!("{s} is not a valid Resource"),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blueprint {
    robot_costs: HashMap<Resource, HashMap<Resource, usize>>,
}

impl Blueprint {
    pub fn max_geodes_in_minutes(&self, minutes: usize) -> usize {
        0
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let blueprints = input
        .lines()
        .filter_map(|l| l.parse::<Blueprint>().ok())
        .collect::<Vec<_>>();
    let cumulative_score = blueprints
        .iter()
        .map(|b| b.max_geodes_in_minutes(26))
        .enumerate()
        .map(|(n, s)| n * s).sum();
    Some(cumulative_score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let blueprints = input
        .lines()
        .filter_map(|l| l.parse::<Blueprint>().ok())
        .collect::<Vec<_>>();
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}

mod input_parser {
    use super::{Blueprint, Resource};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, space1},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
        Finish, IResult,
    };
    use std::collections::HashMap;
    use std::str::FromStr;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn resource(input: &str) -> IResult<&str, Resource> {
        map_res(
            alt((tag("ore"), tag("clay"), tag("obsidian"), tag("geode"))),
            |s: &str| s.parse::<Resource>(),
        )(input)
    }

    fn cost(input: &str) -> IResult<&str, (Resource, usize)> {
        map(separated_pair(number, space1, resource), |(n, r)| (r, n))(input)
    }

    fn costs(input: &str) -> IResult<&str, HashMap<Resource, usize>> {
        map(separated_list1(tag(" and "), cost), |vs| {
            vs.into_iter().collect()
        })(input)
    }

    fn resource_costs(input: &str) -> IResult<&str, (Resource, HashMap<Resource, usize>)> {
        map(
            tuple((
                tag("Each"),
                space1,
                resource,
                space1,
                tag("robot"),
                space1,
                tag("costs"),
                space1,
                costs,
                tag("."),
            )),
            |(_, _, res, _, _, _, _, _, cs, _)| (res, cs),
        )(input)
    }

    fn blueprint(input: &str) -> IResult<&str, Blueprint> {
        map(
            tuple((
                tag("Blueprint"),
                space1,
                number,
                tag(":"),
                space1,
                separated_list1(space1, resource_costs),
            )),
            |t| Blueprint {
                robot_costs: t.5.into_iter().collect(),
            },
        )(input)
    }

    impl FromStr for Blueprint {
        type Err = nom::error::Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match blueprint(s).finish() {
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
        fn parse_blueprint() {
            assert_eq!(
                blueprint("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."),
                Ok((
                    "",
                    Blueprint {
                        robot_costs: HashMap::from([
                            (Resource::Ore, HashMap::from([
                                (Resource::Ore, 4),
                            ])),
                            (Resource::Clay, HashMap::from([
                                (Resource::Ore, 2),
                            ])),
                            (Resource::Obsidian, HashMap::from([
                                (Resource::Ore, 3),
                                (Resource::Clay, 14),
                            ])),
                            (Resource::Geode, HashMap::from([
                                (Resource::Ore, 2),
                                (Resource::Obsidian, 7),
                            ])),
                        ])
                    }
                ))
            );
        }
    }
}
