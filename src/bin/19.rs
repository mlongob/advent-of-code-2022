use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ops::BitAnd;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GameState {
    robots: BTreeMap<Resource, usize>,
    resources: BTreeMap<Resource, usize>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            robots: BTreeMap::from([(Resource::Ore, 1)]),
            resources: BTreeMap::new(),
        }
    }

    pub fn collect(&mut self) {
        // Harvest robot resources
        self.robots.iter().for_each(|(resource, robots)| {
            self.resources
                .entry(*resource)
                .and_modify(|cnt| *cnt += robots)
                .or_insert(*robots);
        })
    }

    pub fn can_afford(&self, costs: &HashMap<Resource, usize>) -> bool {
        costs
            .iter()
            .all(|(res, cost)| self.resources.get(res).unwrap_or(&0) >= cost)
    }

    pub fn build_robot(&mut self, blueprint: &Blueprint, resource: Resource) -> Option<()> {
        let robot_costs = blueprint.robot_costs.get(&resource)?;
        if self.can_afford(robot_costs) {
            robot_costs.iter().for_each(|(res, cost)| {
                self.resources.entry(*res).and_modify(|cnt| *cnt -= cost);
            });
            self.robots
                .entry(resource)
                .and_modify(|cnt| *cnt += 1)
                .or_insert(1);
            Some(())
        } else {
            None
        }
    }

    pub fn affordable_robots<'a>(
        &'a self,
        blueprint: &'a Blueprint,
    ) -> impl Iterator<Item = Resource> + 'a {
        blueprint
            .robot_costs
            .iter()
            .filter(|(_, costs)| self.can_afford(costs))
            .map(|(res, _)| *res)
    }

    pub fn needed_robots<'a>(
        &'a self,
        blueprint: &'a Blueprint,
    ) -> impl Iterator<Item = Resource> + 'a {
        let max_robots = blueprint
            .robot_costs
            .values()
            .fold(HashMap::new(), |mut acc, v| {
                v.iter().for_each(|(res, cnt)| {
                    acc.entry(*res)
                        .and_modify(|c: &mut usize| *c = (*c).max(*cnt))
                        .or_insert(*cnt);
                });
                acc
            });
        max_robots
            .into_iter()
            .filter(|(resource, max)| *self.robots.get(resource).unwrap_or(&0) < *max)
            .map(|(resource, _)| resource)
    }

    pub fn robots_to_buy(&self, blueprint: &Blueprint) -> BTreeSet<Resource> {
        let affordable_robots: BTreeSet<_> = self.affordable_robots(blueprint).collect();
        if affordable_robots.contains(&Resource::Geode) {
            BTreeSet::from([Resource::Geode])
        } else {
            let needed_robots: BTreeSet<_> = self.needed_robots(blueprint).collect();
            affordable_robots.bitand(&needed_robots)
        }
    }

    pub fn geodes(&self) -> usize {
        *self.resources.get(&Resource::Geode).unwrap_or(&0)
    }

    pub fn geodes_upper_limit(&self, minutes: usize) -> usize {
        // Geodes we have
        let current = self.geodes();

        // Geodes we will have with existing Robots
        let future = self.robots.get(&Resource::Geode).unwrap_or(&0) * minutes;

        // Geodes we will have if we build geode robots on every remaining turn (optimistic)
        let optimistic = (minutes - 1) * (minutes / 2);

        current + future + optimistic
    }
}

impl Blueprint {
    fn max_geode_helper(
        &self,
        minutes: usize,
        mut state: GameState,
        do_not_buy: BTreeSet<Resource>,
        running_max: &mut usize,
    ) -> usize {
        // Optimization #1:
        // If this branch can't possibly get more geodes than the running max, abandon the branch
        if state.geodes_upper_limit(minutes) < *running_max {
            return 0;
        }
        let max_geodes = if minutes == 1 {
            // Optimization #2:
            // If there's just 1 minute left, don't bother building
            state.collect();
            state.geodes()
        } else {
            // Optimization #3:
            // Only look at candidate robots for resources where there isn't enough robots to mine
            // the cost of any other robots in 1 turn. (implemented by robots_to_buy)
            let candidates = state.robots_to_buy(self);
            if candidates.contains(&Resource::Geode) {
                // Optimization #4:
                // If you can build a geode robot, just do it. No need to look at other branches
                state.collect();
                state.build_robot(self, Resource::Geode);
                return self.max_geode_helper(minutes - 1, state, BTreeSet::new(), running_max);
            }
            // Optimization #5:
            // If we decide not to build a robot when we have the option, that robot should not be built anywhere
            // else in that branch until another robot is built (do_not_buy set)
            let buy_a_robot = candidates
                .difference(&do_not_buy)
                .map(|resource| {
                    let mut state = state.clone();
                    state.collect();
                    state.build_robot(self, *resource);
                    self.max_geode_helper(minutes - 1, state, BTreeSet::new(), running_max)
                })
                .max()
                .unwrap_or(0);
            state.collect();
            let wait_it_out = self.max_geode_helper(minutes - 1, state, candidates, running_max);
            buy_a_robot.max(wait_it_out)
        };
        *running_max = (*running_max).max(max_geodes);
        max_geodes
    }

    pub fn max_geodes_in_minutes(&self, minutes: usize) -> usize {
        let mut running_max = 0;
        self.max_geode_helper(minutes, GameState::new(), BTreeSet::new(), &mut running_max)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let cumulative_score = input
        .lines()
        .filter_map(|l| l.parse::<Blueprint>().ok())
        .map(|b| b.max_geodes_in_minutes(24))
        .enumerate()
        .map(|(n, s)| (n + 1) * s)
        .sum();
    Some(cumulative_score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let max_product = input
        .lines()
        .filter_map(|l| l.parse::<Blueprint>().ok())
        .take(3)
        .map(|b| b.max_geodes_in_minutes(32))
        .product();
    Some(max_product)
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
        assert_eq!(part_two(&input), Some(3472));
    }

    #[test]
    fn part_two_blueprint_1() {
        let blueprint = advent_of_code::read_file("examples", 19)
            .lines()
            .next()
            .unwrap()
            .parse::<Blueprint>()
            .unwrap();
        assert_eq!(blueprint.max_geodes_in_minutes(32), 56);
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
