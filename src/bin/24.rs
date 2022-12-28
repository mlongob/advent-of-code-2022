use num::integer::lcm;
use petgraph::algo::astar;
use petgraph::prelude::*;
use petgraph::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::North),
            '<' => Ok(Self::West),
            'v' => Ok(Self::South),
            '>' => Ok(Self::East),
            _ => panic!("{value} is not a valid direction"),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn wrap(self, width: usize, height: usize) -> Position {
        Position {
            x: self.x.rem_euclid(width as i32),
            y: self.y.rem_euclid(height as i32),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blizzard {
    pub initial_position: Position,
    pub direction: Direction,
}

#[derive(Debug, Clone, Default)]
pub struct ValleyGraph {
    graph: Graph<(usize, Position), usize, Directed>,
    start: NodeIndex,
    end_position: Position,
}

impl ValleyGraph {
    fn shortest_path_impl(&self, start: NodeIndex, goal: &Position) -> Option<Vec<NodeIndex>> {
        astar(
            &self.graph,
            start,
            |idx| match self.graph.node_weight(idx) {
                None => false,
                Some((_, pos)) => pos == goal,
            },
            |e| *e.weight(),
            |_| 0,
        )
        .map(|t| t.1)
    }

    pub fn three_leg_path(&self) -> Option<Vec<Position>> {
        let start_position = self.graph.node_weight(self.start)?.1.clone();
        let first_leg = self.shortest_path_impl(self.start, &self.end_position)?;
        let pivot_1 = *first_leg.last()?;
        let second_leg = self.shortest_path_impl(pivot_1, &start_position)?;
        let pivot_2 = *second_leg.last()?;
        let third_leg = self.shortest_path_impl(pivot_2, &self.end_position)?;
        Some(
            first_leg
                .into_iter()
                .chain(second_leg.into_iter().skip(1))
                .chain(third_leg.into_iter().skip(1))
                .filter_map(|idx| self.graph.node_weight(idx))
                .map(|(_, p)| p.clone())
                .collect::<Vec<_>>(),
        )
    }

    pub fn single_leg_path(&self) -> Option<Vec<Position>> {
        self.shortest_path_impl(self.start, &self.end_position)
            .map(|v| {
                v.into_iter()
                    .filter_map(|idx| self.graph.node_weight(idx))
                    .map(|(_, p)| p.clone())
                    .collect::<Vec<_>>()
            })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Valley {
    width: usize,
    height: usize,
    start: Position,
    end: Position,
    blizzards: HashSet<Blizzard>,
}

impl Valley {
    pub fn in_bounds(&self, pos: &Position) -> bool {
        *pos == self.start
            || *pos == self.end
            || (pos.x >= 0
                && pos.x < (self.width as i32)
                && pos.y >= 0
                && pos.y < (self.height as i32))
    }

    pub fn neighbors<'a>(&'a self, pos: &'a Position) -> impl Iterator<Item = Position> + 'a {
        [
            // Same position
            Position { x: pos.x, y: pos.y },
            // South position
            Position {
                x: pos.x,
                y: pos.y + 1,
            },
            // North position
            Position {
                x: pos.x,
                y: pos.y - 1,
            },
            // East position
            Position {
                x: pos.x + 1,
                y: pos.y,
            },
            // West position
            Position {
                x: pos.x - 1,
                y: pos.y,
            },
        ]
        .into_iter()
        .filter(|p| self.in_bounds(p))
    }

    pub fn blizzard_position(&self, blizzard: &Blizzard, t: usize) -> Position {
        let t = t as i32;
        match blizzard.direction {
            Direction::North => Position {
                x: blizzard.initial_position.x,
                y: blizzard.initial_position.y - t,
            },
            Direction::West => Position {
                x: blizzard.initial_position.x - t,
                y: blizzard.initial_position.y,
            },
            Direction::South => Position {
                x: blizzard.initial_position.x,
                y: blizzard.initial_position.y + t,
            },
            Direction::East => Position {
                x: blizzard.initial_position.x + t,
                y: blizzard.initial_position.y,
            },
        }
        .wrap(self.width, self.height)
    }

    pub fn period(&self) -> usize {
        lcm(self.height, self.width)
    }

    pub fn graph(&self) -> ValleyGraph {
        let mut graph: Graph<(usize, Position), usize, Directed> = Graph::new();
        let mut node_map: HashMap<(usize, Position), NodeIndex> = HashMap::new();
        let period = self.period();

        // For all periods before we go back to the same blizzard configuration
        for t in 0..period {
            // Mark all blizzard positions
            let blizzards = self
                .blizzards
                .iter()
                .map(|b| self.blizzard_position(b, t))
                .collect::<HashSet<_>>();
            for x in 0..self.width {
                for y in 0..self.height {
                    let pos = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    if !blizzards.contains(&pos) {
                        // Insert a node in the graph for every valid position
                        node_map.insert((t, pos.clone()), graph.add_node((t, pos)));
                    }
                }
            }
            // Insert start node
            node_map.insert(
                (t, self.start.clone()),
                graph.add_node((t, self.start.clone())),
            );

            // Insert end node
            node_map.insert((t, self.end.clone()), graph.add_node((t, self.end.clone())));
        }

        // Add edges
        node_map.iter().for_each(|((t, pos), source)| {
            let next_t = (t + 1).rem_euclid(period);
            // Find neighbors in the next t and add them as edges
            self.neighbors(pos)
                .filter_map(|n_pos| {
                    let target = node_map.get(&(next_t, n_pos))?;
                    Some((*source, *target))
                })
                .for_each(|(source, target)| {
                    graph.add_edge(source, target, 1);
                });
        });

        // Start at the start position on t = 0
        let start = *node_map
            .get(&(0, self.start.clone()))
            .expect("Start must exist");

        ValleyGraph {
            graph,
            start,
            end_position: self.end.clone(),
        }
    }
}

impl FromStr for Valley {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_line = s
            .lines()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Empty input"))?;
        let last_line = s
            .lines()
            .last()
            .ok_or_else(|| anyhow::anyhow!("Empty input"))?;
        let width = first_line.len() - 2;
        let height = s.lines().count() - 2;
        let blizzards = s
            .lines()
            .skip(1)
            .take(height)
            .enumerate()
            .flat_map(|(y_u, l)| {
                l.chars()
                    .skip(1)
                    .take(width)
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .filter_map(move |(x_u, c)| {
                        let initial_position = Position {
                            x: x_u as i32,
                            y: y_u as i32,
                        };
                        let direction = Direction::try_from(c).ok()?;
                        Some(Blizzard {
                            initial_position,
                            direction,
                        })
                    })
            })
            .collect::<HashSet<_>>();
        let start = first_line
            .chars()
            .skip(1)
            .take(width)
            .enumerate()
            .find_map(|(x_u, c)| {
                if c == '.' {
                    Some(Position {
                        x: x_u as i32,
                        y: -1,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No start position"))?;
        let end = last_line
            .chars()
            .skip(1)
            .take(width)
            .enumerate()
            .find_map(|(x_u, c)| {
                if c == '.' {
                    Some(Position {
                        x: x_u as i32,
                        y: height as i32,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No start position"))?;
        Ok(Valley {
            width,
            height,
            start,
            end,
            blizzards,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let valley = input.parse::<Valley>().ok()?;
    let valley_graph = valley.graph();
    let path = valley_graph.single_leg_path()?;
    Some(path.len() - 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let valley = input.parse::<Valley>().ok()?;
    let valley_graph = valley.graph();
    let path = valley_graph.three_leg_path()?;
    Some(path.len() - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
