use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketData {
    Number(u32),
    List(Vec<PacketData>),
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(x), Self::Number(y)) => Some(x.cmp(y)),
            (Self::List(x), Self::List(y)) => {
                match x
                    .iter()
                    .zip(y.iter())
                    .map(|(a, b)| a.partial_cmp(b))
                    .find(|c| c != &Some(Ordering::Equal))
                {
                    Some(ordering) => ordering,
                    None => Some(x.len().cmp(&y.len())),
                }
            }
            (x @ Self::Number(_), y @ Self::List(_)) => Self::List(vec![x.clone()]).partial_cmp(y),
            (x @ Self::List(_), y @ Self::Number(_)) => x.partial_cmp(&Self::List(vec![y.clone()])),
        }
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistressSignal {
    pub signal: Vec<(PacketData, PacketData)>,
}

impl DistressSignal {
    pub fn check(&self) -> u32 {
        self.signal
            .iter()
            .enumerate()
            .map(|(index, (first, second))| {
                if first < second {
                    (index + 1) as u32
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn decoder_key(&self) -> Option<u32> {
        let signal_one: PacketData =
            PacketData::List(vec![PacketData::List(vec![PacketData::Number(2)])]);
        let signal_two: PacketData =
            PacketData::List(vec![PacketData::List(vec![PacketData::Number(6)])]);
        let signals = self
            .signal
            .iter()
            .flat_map(|(a, b)| [a, b])
            .sorted()
            .collect::<Vec<_>>();
        let s1 = match signals.binary_search(&&signal_one) {
            Ok(s) | Err(s) => (s + 1) as u32,
        };
        let s2 = match signals.binary_search(&&signal_two) {
            Ok(s) | Err(s) => (s + 2) as u32,
        };
        Some(s1 * s2)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let distress_signal: DistressSignal = input.parse().ok()?;
    Some(distress_signal.check())
}

pub fn part_two(input: &str) -> Option<u32> {
    let distress_signal: DistressSignal = input.parse().ok()?;
    distress_signal.decoder_key()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}

mod input_parser {
    use super::{DistressSignal, PacketData};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, newline},
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::{delimited, separated_pair, tuple},
        Finish, IResult,
    };
    use std::str::FromStr;

    fn number(input: &str) -> IResult<&str, PacketData> {
        map(
            map_res(digit1, |s: &str| s.parse::<u32>()),
            PacketData::Number,
        )(input)
    }

    fn list(input: &str) -> IResult<&str, PacketData> {
        map(
            delimited(tag("["), separated_list0(tag(","), packet_data), tag("]")),
            PacketData::List,
        )(input)
    }

    fn packet_data(input: &str) -> IResult<&str, PacketData> {
        alt((list, number))(input)
    }

    impl FromStr for PacketData {
        type Err = nom::error::Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match packet_data(s).finish() {
                Ok((_remaining, plan)) => Ok(plan),
                Err(nom::error::Error { input, code }) => Err(Self::Err {
                    input: input.to_string(),
                    code,
                }),
            }
        }
    }

    fn packet_pairs(input: &str) -> IResult<&str, (PacketData, PacketData)> {
        separated_pair(packet_data, newline, packet_data)(input)
    }

    fn distress_signal(input: &str) -> IResult<&str, DistressSignal> {
        map(
            separated_list0(tuple((newline, newline)), packet_pairs),
            |signal| DistressSignal { signal },
        )(input)
    }

    impl FromStr for DistressSignal {
        type Err = nom::error::Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match distress_signal(s).finish() {
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
        fn parse_number() {
            assert_eq!(number("66"), Ok(("", PacketData::Number(66))));
        }

        #[test]
        fn parse_list() {
            assert_eq!(
                list("[1,2,3]"),
                Ok((
                    "",
                    PacketData::List(vec![
                        PacketData::Number(1),
                        PacketData::Number(2),
                        PacketData::Number(3)
                    ])
                ))
            );
        }

        #[test]
        fn parse_packet_data() {
            assert_eq!(
                list("[[4,4],4,4,4]"),
                Ok((
                    "",
                    PacketData::List(vec![
                        PacketData::List(vec![PacketData::Number(4), PacketData::Number(4),]),
                        PacketData::Number(4),
                        PacketData::Number(4),
                        PacketData::Number(4),
                    ])
                ))
            );
        }
    }
}
