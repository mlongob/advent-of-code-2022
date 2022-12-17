use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shift {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pattern {
    shifts: Vec<Shift>,
}

impl FromStr for Pattern {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shifts = s
            .chars()
            .filter_map(|c| match c {
                '<' => Some(Shift::Left),
                '>' => Some(Shift::Right),
                _ => None,
            })
            .collect();
        Ok(Pattern { shifts })
    }
}

impl Pattern {
    pub fn iter(&self) -> impl Iterator<Item = &Shift> {
        self.shifts.iter().cycle()
    }
}

impl IntoIterator for Pattern {
    type Item = Shift;
    type IntoIter = std::iter::Cycle<std::vec::IntoIter<Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        self.shifts.into_iter().cycle()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct Position {
    pub y: i64,
    pub x: i64,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn shift(&mut self, direction: &Shift) {
        match direction {
            Shift::Left => self.x -= 1,
            Shift::Right => self.x += 1,
        };
    }

    pub fn fall(&mut self) {
        self.y -= 1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct Shape {
    rocks: Vec<Position>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShapeType {
    HorizontalLine,
    Plus,
    ReverseL,
    VerticalLine,
    Square,
}

impl Shape {
    pub fn new(shape_type: ShapeType, height: i64) -> Shape {
        const START_COL: i64 = 2;

        let rocks = match shape_type {
            ShapeType::HorizontalLine => (0..4)
                .map(|i| Position {
                    x: START_COL + i,
                    y: height,
                })
                .collect(),
            ShapeType::Plus => {
                vec![
                    Position {
                        x: START_COL + 1,
                        y: height,
                    },
                    Position {
                        x: START_COL,
                        y: height + 1,
                    },
                    Position {
                        x: START_COL + 1,
                        y: height + 1,
                    },
                    Position {
                        x: START_COL + 2,
                        y: height + 1,
                    },
                    Position {
                        x: START_COL + 1,
                        y: height + 2,
                    },
                ]
            }
            ShapeType::ReverseL => {
                vec![
                    Position {
                        x: START_COL,
                        y: height,
                    },
                    Position {
                        x: START_COL + 1,
                        y: height,
                    },
                    Position {
                        x: START_COL + 2,
                        y: height,
                    },
                    Position {
                        x: START_COL + 2,
                        y: height + 1,
                    },
                    Position {
                        x: START_COL + 2,
                        y: height + 2,
                    },
                ]
            }
            ShapeType::VerticalLine => (0..4)
                .map(|i| Position {
                    x: START_COL,
                    y: height + i,
                })
                .collect(),
            ShapeType::Square => {
                vec![
                    Position {
                        x: START_COL,
                        y: height,
                    },
                    Position {
                        x: START_COL + 1,
                        y: height,
                    },
                    Position {
                        x: START_COL,
                        y: height + 1,
                    },
                    Position {
                        x: START_COL + 1,
                        y: height + 1,
                    },
                ]
            }
        };
        Shape { rocks }
    }

    pub fn shift(&mut self, direction: &Shift) {
        self.rocks.iter_mut().for_each(|p| p.shift(direction));
    }

    pub fn fall(&mut self) {
        self.rocks.iter_mut().for_each(Position::fall);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Position> {
        self.rocks.iter()
    }

    pub fn range(&self) -> std::ops::RangeInclusive<Position> {
        let min_p = self.rocks.iter().fold(
            Position {
                x: i64::MAX,
                y: i64::MAX,
            },
            |acc, p| Position {
                x: acc.x.min(p.x),
                y: acc.y.min(p.y),
            },
        );
        let max_p = self.rocks.iter().fold(
            Position {
                x: i64::MIN,
                y: i64::MIN,
            },
            |acc, p| Position {
                x: acc.x.max(p.x),
                y: acc.y.max(p.y),
            },
        );
        min_p..=max_p
    }
}

impl IntoIterator for Shape {
    type Item = Position;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.rocks.into_iter()
    }
}

pub struct TetrisChamber {
    rocks: BTreeSet<Position>,
    falling_shape: Option<Shape>,
    shape_iter: Box<dyn Iterator<Item = ShapeType>>,
    shift_iter: Box<dyn Iterator<Item = Shift>>,
}

impl fmt::Display for TetrisChamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let falling_rocks = match &self.falling_shape {
            None => BTreeSet::new(),
            Some(shape) => shape.iter().collect::<BTreeSet<_>>(),
        };
        let falling_rocks_height = falling_rocks.last().map(|p| p.y + 1).unwrap_or(1);
        let rows = self.height().max(falling_rocks_height);
        for y in (0..rows).rev() {
            write!(f, "|")?;
            for x in 0..Self::WIDTH {
                if self.rocks.contains(&Position { x, y }) {
                    write!(f, "#")?;
                } else if falling_rocks.contains(&Position { x, y }) {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+{}+", "-".repeat(Self::WIDTH as usize))
    }
}

impl TetrisChamber {
    const WIDTH: i64 = 7;

    pub fn new(pattern: Pattern) -> TetrisChamber {
        let rocks = BTreeSet::new();
        let shape_pattern = [
            ShapeType::HorizontalLine,
            ShapeType::Plus,
            ShapeType::ReverseL,
            ShapeType::VerticalLine,
            ShapeType::Square,
        ];
        let shape_iter = Box::new(shape_pattern.into_iter().cycle());
        let shift_iter = Box::new(pattern.into_iter());
        let falling_shape = None;
        TetrisChamber {
            rocks,
            shape_iter,
            shift_iter,
            falling_shape,
        }
    }

    fn collides(&self, shape: &Shape) -> bool {
        let range = shape.range();
        if range.start().y < 0 || range.start().x < 0 || range.end().x >= Self::WIDTH {
            true
        } else {
            shape.iter().any(|p| self.rocks.contains(p))
        }
    }

    pub fn shape_fall(&mut self) {
        const FALL_HEIGHT: i64 = 3;
        let shape_height = self.height() + FALL_HEIGHT;
        let shape_type = self.shape_iter.next().expect("Infinite iterator");
        self.falling_shape = Some(Shape::new(shape_type, shape_height));
        //println!("The rock begins falling:");
        //println!("{self}");
        loop {
            // Shift
            let direction = self.shift_iter.next().expect("Infinite iterator");
            {
                let mut shifted = self.falling_shape.as_ref().unwrap().clone();
                shifted.shift(&direction);
                if !self.collides(&shifted) {
                    self.falling_shape = Some(shifted);
                    //println!("Jet of gas pushes rock {direction:?}:");
                } else {
                    //println!("Jet of gas pushes rock {direction:?}, but nothing happens:");
                }
                //println!("{self}");
            }
            // Fall
            {
                let mut fell = self.falling_shape.as_ref().unwrap().clone();
                fell.fall();
                if !self.collides(&fell) {
                    self.falling_shape = Some(fell);
                    //println!("Rock falls 1 unit:");
                    //println!("{self}");
                } else {
                    break;
                }
            }
        }
        //println!("Rock falls 1 unit, causing it to come to rest:");
        //println!("{self}");
        self.rocks
            .extend(self.falling_shape.take().unwrap().into_iter());
    }

    pub fn height(&self) -> i64 {
        self.rocks.last().map(|p| p.y + 1).unwrap_or(0)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut tetris_chamber = TetrisChamber::new(input.parse::<Pattern>().unwrap());
    for _ in 0..2022 {
        tetris_chamber.shape_fall();
    }
    Some(tetris_chamber.height())
}

pub fn find_cycle_to_run_n<F>(
    n: i64,
    max_cycle_len: usize,
    offset_len: usize,
    run: F,
) -> Option<i64>
where
    F: FnMut() -> i64,
{
    let mut run_iter = std::iter::repeat_with(run)
        .tuple_windows()
        .map(|(h1, h2)| h2 - h1);

    // Take away a fixed offsets of heighths and record the sum
    let offset_sum = run_iter.by_ref().take(offset_len).sum::<i64>();
    let deltas = run_iter.take(max_cycle_len).collect_vec();

    // Find the cycle length that satisfies the whole pattern
    let cycle_len = (1..max_cycle_len).find(|size| {
        let window = deltas[..*size].iter().cycle();
        deltas.iter().zip(window).all(|(a, b)| a == b)
    })?;

    // Sum heights for the cycle length
    let cycle_sum = deltas.iter().take(cycle_len).sum::<i64>();

    // Count number of cycles needed to get to n
    let cycle_count = (n - (offset_len as i64)) / (cycle_len as i64);

    // Count items needed as the remainder of the cycles
    let reminder_items = (n - (offset_len as i64)) % (cycle_len as i64);

    // Sum heights for the remainder items
    let reminder_sum = deltas.iter().take(reminder_items as usize).sum::<i64>();

    // Sum up everything
    Some(offset_sum + cycle_count * cycle_sum + reminder_sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut tetris_chamber = TetrisChamber::new(input.parse::<Pattern>().unwrap());

    find_cycle_to_run_n(1_000_000_000_000, 3000, 250, || {
        let h = tetris_chamber.height();
        tetris_chamber.shape_fall();
        h
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
