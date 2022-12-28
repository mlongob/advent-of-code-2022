use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt;
use std::str::FromStr;
use std::ops::Add;
use std::ops::AddAssign;
use std::iter::Sum;

use itertools::Itertools;
use itertools::EitherOrBoth::{Both, Left, Right};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SnafuDigit {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two
}

impl TryFrom<char> for SnafuDigit {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '=' => Ok(Self::DoubleMinus),
            '-' => Ok(Self::Minus),
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            _ => panic!("{value} is not a valid digit"),
        }
    }
}

impl Display for SnafuDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DoubleMinus => write!(f, "="),
            Self::Minus => write!(f, "-"),
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
        }
    }
}

impl Add for SnafuDigit {
    type Output = (SnafuDigit, SnafuDigit);
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // 0 + a >> (0, a)
            (Self::Zero, d) | (d, Self::Zero) => (Self::Zero, d),

            // 1 + = >> (0, -)
            (Self::One, Self::DoubleMinus) | (Self::DoubleMinus, Self::One) => (Self::Zero, Self::Minus),

            // 1 + - >> (0, 0)
            (Self::One, Self::Minus) | (Self::Minus, Self::One) => (Self::Zero, Self::Zero),

            // 1 + 1 >> (0, 2)
            (Self::One, Self::One) => (Self::Zero, Self::Two),

            // 1 + 2 >> (1, =)
            (Self::One, Self::Two) | (Self::Two, Self::One) => (Self::One, Self::DoubleMinus),

            // 2 + = >> (0, 0)
            (Self::Two, Self::DoubleMinus) | (Self::DoubleMinus, Self::Two) => (Self::Zero, Self::Zero),

            // 2 + - >> (0, 1)
            (Self::Two, Self::Minus) | (Self::Minus, Self::Two) => (Self::Zero, Self::One),

            // 2 + 2 >> (1, -)
            (Self::Two, Self::Two) => (Self::One, Self::Minus),

            // - + - >> (0, =)
            (Self::Minus, Self::Minus) => (Self::Zero, Self::DoubleMinus),

            // - + = >> (-, 2)
            (Self::Minus, Self::DoubleMinus) | (Self::DoubleMinus, Self::Minus) => (Self::Minus, Self::Two),

            // = + = >> (-, 1)
            (Self::DoubleMinus, Self::DoubleMinus) => (Self::Minus, Self::One),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Snafu {
    digits: VecDeque<SnafuDigit>
}

impl Snafu {
    pub fn zero() -> Snafu {
        Snafu { digits: VecDeque::from([SnafuDigit::Zero]) }
    }

    pub fn one() -> Snafu {
        Snafu { digits: VecDeque::from([SnafuDigit::One]) }
    }

    pub fn new() -> Snafu {
        Self::zero()
    }
}

impl Default for Snafu {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for digit in self.digits.iter() {
            write!(f, "{digit}")?;
        }
        Ok(())
    }
}

impl FromStr for Snafu {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s.chars().flat_map(|c|SnafuDigit::try_from(c).ok()).collect::<VecDeque<_>>();
        Ok(Snafu { digits })
    }
}

impl Add for Snafu {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (mut acc, carry) = self.digits.into_iter().rev().zip_longest(rhs.digits.into_iter().rev()).map(|x| match x {
            Both(a, b) => (a, b),
            Left(a) => (a, SnafuDigit::Zero),
            Right(b) => (SnafuDigit::Zero, b),
        }).fold((Snafu{digits: VecDeque::new()}, SnafuDigit::Zero), |(mut acc, carry), (lhs_digit, rhs_digit)| {
            let (c1, r1) = lhs_digit + carry;
            let (c2, result) = rhs_digit + r1;
            // Two carry's do not add up to more carry's
            let (_, carry) = c1 + c2;
            acc.digits.push_front(result);
            (acc, carry)
        });
        if carry != SnafuDigit::Zero {
            acc.digits.push_front(carry);
        }
        acc
    }
}

impl AddAssign for Snafu {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, i| acc + i)
    }
}

pub fn part_one(input: &str) -> Option<Snafu> {
    Some(input.lines().filter_map(|l| l.parse::<Snafu>().ok()).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".parse().unwrap()));
    }
}
