use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub y: u32,
    pub x: u32,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn score(&self) -> u32 {
        1000 * (self.y + 1) + 4 * (self.x + 1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    Void,
    Open,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(Self::Void),
            '.' => Ok(Self::Open),
            '#' => Ok(Self::Wall),
            _ => panic!("{value} is not a valid tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Void => write!(f, " "),
            Tile::Open => write!(f, "."),
            Tile::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command {
    TurnLeft,
    TurnRight,
    GoForward(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_left(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WrapStyle {
    Flat,
    Cube,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    grid: BTreeMap<Position, Tile>,
    position: Position,
    direction: Direction,
    range: RangeInclusive<Position>,
}

impl Board {
    pub fn score(&self) -> u32 {
        self.position.score() + self.direction.score()
    }

    fn front_flat(&self, position: &Position, direction: &Direction) -> (Position, Direction) {
        let position = match direction {
            Direction::Up => {
                let x = position.x;
                let y = if position.y == 0 {
                    self.range.end().y
                } else {
                    position.y - 1
                };
                Position { x, y }
            }
            Direction::Right => {
                let x = if position.x == self.range.end().x {
                    0
                } else {
                    position.x + 1
                };
                let y = position.y;
                Position { x, y }
            }
            Direction::Down => {
                let x = position.x;
                let y = if position.y == self.range.end().y {
                    0
                } else {
                    position.y + 1
                };
                Position { x, y }
            }
            Direction::Left => {
                let x = if position.x == 0 {
                    self.range.end().x
                } else {
                    position.x - 1
                };
                let y = position.y;
                Position { x, y }
            }
        };
        (position, direction.clone())
    }

    fn front_cube_example(
        &self,
        position: &Position,
        direction: &Direction,
    ) -> (Position, Direction) {
        match direction {
            Direction::Up => match position {
                Position { y: 0, x: 8..=11 } => (
                    Position {
                        y: 4,
                        x: 11 - position.x,
                    },
                    Direction::Down,
                ),
                Position { y: 4, x: 0..=3 } => (
                    Position {
                        y: 0,
                        x: 11 - position.x,
                    },
                    Direction::Down,
                ),
                Position { y: 4, x: 4..=7 } => (
                    Position {
                        y: position.x - 4,
                        x: 8,
                    },
                    Direction::Right,
                ),
                Position { y: 8, x: 12..=15 } => (
                    Position {
                        y: 19 - position.x,
                        x: 11,
                    },
                    Direction::Left,
                ),
                _ => (
                    Position {
                        x: position.x,
                        y: position.y - 1,
                    },
                    Direction::Up,
                ),
            },
            Direction::Right => match position {
                Position { y: 0..=3, x: 11 } => (
                    Position {
                        y: 11 - position.y,
                        x: 15,
                    },
                    Direction::Left,
                ),
                Position { y: 4..=7, x: 11 } => (
                    Position {
                        y: 8,
                        x: 19 - position.y,
                    },
                    Direction::Down,
                ),
                Position { y: 8..=11, x: 15 } => (
                    Position {
                        y: 11 - position.y,
                        x: 11,
                    },
                    Direction::Left,
                ),
                _ => (
                    Position {
                        x: position.x + 1,
                        y: position.y,
                    },
                    Direction::Right,
                ),
            },
            Direction::Down => match position {
                Position { y: 7, x: 0..=3 } => (
                    Position {
                        y: 11,
                        x: 11 - position.x,
                    },
                    Direction::Up,
                ),
                Position { y: 7, x: 4..=7 } => (
                    Position {
                        y: 15 - position.x,
                        x: 8,
                    },
                    Direction::Right,
                ),
                Position { y: 11, x: 8..=11 } => (
                    Position {
                        y: 7,
                        x: 11 - position.x,
                    },
                    Direction::Up,
                ),
                Position { y: 11, x: 12..=15 } => (
                    Position {
                        y: 19 - position.x,
                        x: 0,
                    },
                    Direction::Right,
                ),
                _ => (
                    Position {
                        x: position.x,
                        y: position.y + 1,
                    },
                    Direction::Down,
                ),
            },
            Direction::Left => match position {
                Position { y: 0..=3, x: 8 } => (
                    Position {
                        y: 4,
                        x: position.y + 4,
                    },
                    Direction::Down,
                ),
                Position { y: 4..=7, x: 0 } => (
                    Position {
                        y: 11,
                        x: 19 - position.y,
                    },
                    Direction::Up,
                ),
                Position { y: 8..=11, x: 8 } => (
                    Position {
                        y: 7,
                        x: 15 - position.y,
                    },
                    Direction::Up,
                ),
                _ => (
                    Position {
                        x: position.x - 1,
                        y: position.y,
                    },
                    Direction::Left,
                ),
            },
        }
    }

    fn front_cube_input(
        &self,
        position: &Position,
        direction: &Direction,
    ) -> (Position, Direction) {
        match direction {
            Direction::Up => match position {
                Position { y: 0, x: 50..=99 } => (
                    Position {
                        y: 100 + position.x,
                        x: 0,
                    },
                    Direction::Right,
                ),
                Position { y: 0, x: 100..=149 } => (
                    Position {
                        y: 199,
                        x: position.x - 100,
                    },
                    Direction::Up,
                ),
                Position { y: 100, x: 0..=49 } => (
                    Position {
                        y: 50 + position.x,
                        x: 50,
                    },
                    Direction::Right,
                ),
                _ => (
                    Position {
                        x: position.x,
                        y: position.y - 1,
                    },
                    Direction::Up,
                ),
            },
            Direction::Right => match position {
                Position { y: 0..=49, x: 149 } => (
                    Position {
                        y: 149 - position.y,
                        x: 99,
                    },
                    Direction::Left,
                ),
                Position { y: 50..=99, x: 99 } => (
                    Position {
                        y: 49,
                        x: 50 + position.y,
                    },
                    Direction::Up,
                ),
                Position {
                    y: 100..=149,
                    x: 99,
                } => (
                    Position {
                        y: 149 - position.y,
                        x: 149,
                    },
                    Direction::Left,
                ),
                Position {
                    y: 150..=199,
                    x: 49,
                } => (
                    Position {
                        y: 149,
                        x: position.y - 100,
                    },
                    Direction::Up,
                ),
                _ => (
                    Position {
                        x: position.x + 1,
                        y: position.y,
                    },
                    Direction::Right,
                ),
            },
            Direction::Down => match position {
                Position {
                    y: 49,
                    x: 100..=149,
                } => (
                    Position {
                        y: position.x - 50,
                        x: 99,
                    },
                    Direction::Left,
                ),
                Position { y: 149, x: 50..=99 } => (
                    Position {
                        y: 100 + position.x,
                        x: 49,
                    },
                    Direction::Left,
                ),
                Position { y: 199, x: 0..=49 } => (
                    Position {
                        y: 0,
                        x: position.x + 100,
                    },
                    Direction::Down,
                ),
                _ => (
                    Position {
                        x: position.x,
                        y: position.y + 1,
                    },
                    Direction::Down,
                ),
            },
            Direction::Left => match position {
                Position { y: 0..=49, x: 50 } => (
                    Position {
                        y: 149 - position.y,
                        x: 0,
                    },
                    Direction::Right,
                ),
                Position { y: 50..=99, x: 50 } => (
                    Position {
                        y: 100,
                        x: position.y - 50,
                    },
                    Direction::Down,
                ),
                Position { y: 100..=149, x: 0 } => (
                    Position {
                        y: 149 - position.y,
                        x: 50,
                    },
                    Direction::Right,
                ),
                Position { y: 150..=199, x: 0 } => (
                    Position {
                        y: 0,
                        x: position.y - 100,
                    },
                    Direction::Down,
                ),
                _ => (
                    Position {
                        x: position.x - 1,
                        y: position.y,
                    },
                    Direction::Left,
                ),
            },
        }
    }

    fn front(&self, wrap_style: &WrapStyle) -> (Position, Direction) {
        let mut current = (self.position.clone(), self.direction.clone());
        loop {
            current = match wrap_style {
                WrapStyle::Flat => self.front_flat(&current.0, &current.1),
                WrapStyle::Cube => {
                    if self.range.end().y < 20 {
                        self.front_cube_example(&current.0, &current.1)
                    } else {
                        self.front_cube_input(&current.0, &current.1)
                    }
                }
            };
            let tile = self.grid.get(&current.0).unwrap_or(&Tile::Void);
            if *tile != Tile::Void {
                break;
            }
        }
        current
    }

    fn step(&mut self, wrap_style: &WrapStyle) {
        let front = self.front(wrap_style);
        let front_tile = self.grid.get(&front.0).unwrap_or(&Tile::Void);
        if *front_tile == Tile::Open {
            self.position = front.0;
            self.direction = front.1;
        }
    }

    pub fn apply(&mut self, wrap_style: &WrapStyle, command: &Command) {
        match command {
            Command::TurnLeft => {
                self.direction.turn_left();
            }
            Command::TurnRight => {
                self.direction.turn_right();
            }
            Command::GoForward(steps) => {
                for _ in 0..*steps {
                    self.step(wrap_style);
                }
            }
        }
    }

    fn compute_range(grid: &BTreeMap<Position, Tile>) -> RangeInclusive<Position> {
        let min_cube = grid.keys().fold(
            Position {
                x: u32::MAX,
                y: u32::MAX,
            },
            |a, b| Position {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
        );
        let max_cube = grid.keys().fold(
            Position {
                x: u32::MIN,
                y: u32::MIN,
            },
            |a, b| Position {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
        );
        Position {
            x: min_cube.x,
            y: min_cube.y,
        }..=Position {
            x: max_cube.x,
            y: max_cube.y,
        }
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let x = x as u32;
                    let y = y as u32;
                    let tile = Tile::try_from(c).ok()?;
                    Some((Position { x, y }, tile))
                })
            })
            .collect::<BTreeMap<_, _>>();
        let position = grid
            .iter()
            .find(|(_, v)| **v == Tile::Open)
            .map(|(p, _)| p)
            .ok_or_else(|| anyhow::anyhow!("Grid is empty"))?
            .clone();
        let range = Board::compute_range(&grid);
        Ok(Board {
            range,
            grid,
            position,
            direction: Direction::Right,
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Position { y: max_y, x: max_x } = self.range.end();
        for y in 0..=*max_y {
            for x in 0..=*max_x {
                let pos = Position { x, y };
                if pos == self.position {
                    write!(f, "{}", self.direction)
                } else {
                    match self.grid.get(&pos) {
                        Some(t) => write!(f, "{t}"),
                        None => write!(f, "{}", Tile::Void),
                    }
                }?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    board: Board,
    commands: Vec<Command>,
}

impl FromStr for Input {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (board_str, commands_str) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("Cannot split"))?;
        let board = board_str.parse::<Board>()?;
        let commands = input_parser::parse_commands(commands_str)?;
        Ok(Input { board, commands })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.parse::<Input>().ok()?;
    //println!("{}", input.board);
    for command in input.commands {
        //println!("Applying {:?}:", command);
        input.board.apply(&WrapStyle::Flat, &command);
        //println!("{}", input.board);
    }
    Some(input.board.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.parse::<Input>().ok()?;
    //println!("{}", input.board);
    for command in input.commands {
        //println!("Applying {:?}:", command);
        input.board.apply(&WrapStyle::Cube, &command);
        //println!("{}", input.board);
    }
    Some(input.board.score())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(5031));
    }
}

mod input_parser {
    use super::Command;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res},
        multi::many0,
        Finish, IResult,
    };

    fn command_go_forward(input: &str) -> IResult<&str, Command> {
        map(
            map_res(digit1, |s: &str| s.parse::<usize>()),
            Command::GoForward,
        )(input)
    }

    fn command_turn_left(input: &str) -> IResult<&str, Command> {
        map(tag("L"), |_| Command::TurnLeft)(input)
    }

    fn command_turn_right(input: &str) -> IResult<&str, Command> {
        map(tag("R"), |_| Command::TurnRight)(input)
    }

    fn command(input: &str) -> IResult<&str, Command> {
        alt((command_go_forward, command_turn_left, command_turn_right))(input)
    }

    fn commands(input: &str) -> IResult<&str, Vec<Command>> {
        many0(command)(input)
    }

    pub fn parse_commands(s: &str) -> anyhow::Result<Vec<Command>> {
        match commands(s).finish() {
            Ok((_remaining, plan)) => Ok(plan),
            Err(nom::error::Error { input, code: _ }) => Err(anyhow::anyhow!(input.to_string())),
        }
    }
}
