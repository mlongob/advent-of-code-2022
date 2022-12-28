use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::iter;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub y: i32,
    pub x: i32,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn propose_move(&self, direction: &Direction) -> Position {
        match direction {
            Direction::North => Position {
                y: self.y - 1,
                x: self.x,
            },
            Direction::South => Position {
                y: self.y + 1,
                x: self.x,
            },
            Direction::West => Position {
                y: self.y,
                x: self.x - 1,
            },
            Direction::East => Position {
                y: self.y,
                x: self.x + 1,
            },
        }
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Position> {
        [
            Position {
                y: self.y - 1,
                x: self.x - 1,
            },
            Position {
                y: self.y - 1,
                x: self.x,
            },
            Position {
                y: self.y - 1,
                x: self.x + 1,
            },
            Position {
                y: self.y + 1,
                x: self.x - 1,
            },
            Position {
                y: self.y + 1,
                x: self.x,
            },
            Position {
                y: self.y + 1,
                x: self.x + 1,
            },
            Position {
                y: self.y,
                x: self.x - 1,
            },
            Position {
                y: self.y,
                x: self.x + 1,
            },
        ]
        .into_iter()
    }

    pub fn directional_neighbors(&self, direction: &Direction) -> impl Iterator<Item = Position> {
        match direction {
            Direction::North => [
                Position {
                    y: self.y - 1,
                    x: self.x - 1,
                },
                Position {
                    y: self.y - 1,
                    x: self.x,
                },
                Position {
                    y: self.y - 1,
                    x: self.x + 1,
                },
            ],
            Direction::South => [
                Position {
                    y: self.y + 1,
                    x: self.x - 1,
                },
                Position {
                    y: self.y + 1,
                    x: self.x,
                },
                Position {
                    y: self.y + 1,
                    x: self.x + 1,
                },
            ],
            Direction::West => [
                Position {
                    y: self.y - 1,
                    x: self.x - 1,
                },
                Position {
                    y: self.y,
                    x: self.x - 1,
                },
                Position {
                    y: self.y + 1,
                    x: self.x - 1,
                },
            ],
            Direction::East => [
                Position {
                    y: self.y - 1,
                    x: self.x + 1,
                },
                Position {
                    y: self.y,
                    x: self.x + 1,
                },
                Position {
                    y: self.y + 1,
                    x: self.x + 1,
                },
            ],
        }
        .into_iter()
    }
}

pub struct Grid {
    elves: BTreeSet<Position>,
    round: usize,
}

impl Grid {
    pub fn range(&self) -> RangeInclusive<Position> {
        let min_cube = self.elves.iter().fold(
            Position {
                x: i32::MAX,
                y: i32::MAX,
            },
            |a, b| Position {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
        );
        let max_cube = self.elves.iter().fold(
            Position {
                x: i32::MIN,
                y: i32::MIN,
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

    pub fn empty_tiles_in_rectangle(&self) -> u32 {
        let range = self.range();
        let area = (1 + range.end().y - range.start().y) * (1 + range.end().x - range.start().x);
        (area as u32) - (self.elves.len() as u32)
    }

    fn spend_round(&mut self, new_grid: BTreeSet<Position>) -> Option<()> {
        self.round += 1;
        if self.elves == new_grid {
            None
        } else {
            self.elves = new_grid;
            Some(())
        }
    }

    fn directions(&self) -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .into_iter()
        .cycle()
        .skip(self.round % 4)
        .take(4)
    }

    pub fn run_round(&mut self) -> Option<()> {
        let mut new_grid = BTreeSet::new();
        let mut proposed_moves: BTreeMap<Position, Vec<Position>> = BTreeMap::new();
        self.elves.iter().for_each(|elf| {
            if elf.neighbors().any(|n| self.elves.contains(&n)) {
                match self.directions().find(|d| {
                    elf.directional_neighbors(d)
                        .all(|p| !self.elves.contains(&p))
                }) {
                    None => {
                        new_grid.insert(elf.clone());
                    }
                    Some(d) => {
                        proposed_moves
                            .entry(elf.propose_move(&d))
                            .or_default()
                            .push(elf.clone());
                    }
                }
            } else {
                new_grid.insert(elf.clone());
            }
        });
        proposed_moves.into_iter().for_each(|(proposed, original)| {
            if original.len() == 1 {
                new_grid.insert(proposed);
            } else {
                original.into_iter().for_each(|p| {
                    new_grid.insert(p);
                });
            }
        });
        self.spend_round(new_grid)
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| {
                        let x = x as i32;
                        let y = y as i32;
                        Position { x, y }
                    })
            })
            .collect::<BTreeSet<_>>();
        Ok(Grid { elves, round: 0 })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = self.range();
        for y in range.start().y..=range.end().y {
            for x in range.start().x..=range.end().x {
                if self.elves.contains(&Position { y, x }) {
                    write!(f, "#")
                } else {
                    write!(f, ".")
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input.parse::<Grid>().ok()?;
    for _ in 0..10 {
        grid.run_round();
    }
    Some(grid.empty_tiles_in_rectangle())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = input.parse::<Grid>().ok()?;
    Some(iter::from_fn(|| grid.run_round()).count() + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
