use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Cube {
    pub fn new() -> Cube {
        Cube { x: 0, y: 0, z: 0 }
    }

    pub fn adjacent(&self) -> impl Iterator<Item = Cube> {
        [
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        ]
        .into_iter()
    }
}

impl FromStr for Cube {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        let x = *vals
            .first()
            .ok_or_else(|| anyhow::anyhow!("Cannot parse x"))?;
        let y = *vals
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("Cannot parse y"))?;
        let z = *vals
            .get(2)
            .ok_or_else(|| anyhow::anyhow!("Cannot parse z"))?;
        Ok(Cube { x, y, z })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    cubes: HashSet<Cube>,
    range: std::ops::RangeInclusive<Cube>,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::new();
        grid.cubes = s
            .lines()
            .map(|l| l.parse::<Cube>())
            .collect::<anyhow::Result<HashSet<_>>>()?;
        grid.range = grid.surrounding_range();
        Ok(grid)
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cubes: HashSet::new(),
            range: std::ops::RangeInclusive::new(Cube::new(), Cube::new()),
        }
    }

    pub fn exposed_area(&self) -> i32 {
        let sides = self
            .cubes
            .iter()
            .flat_map(Cube::adjacent)
            .filter(|ac| ac.x >= self.range.start().x && ac.x <= self.range.end().x)
            .filter(|ac| ac.y >= self.range.start().y && ac.y <= self.range.end().y)
            .filter(|ac| ac.z >= self.range.start().z && ac.z <= self.range.end().z)
            .filter(|ac| !self.cubes.contains(ac))
            .count();
        sides as i32
    }

    pub fn surrounding_range(&self) -> std::ops::RangeInclusive<Cube> {
        let min_cube = self.cubes.iter().fold(
            Cube {
                x: i32::MAX - 1,
                y: i32::MAX - 1,
                z: i32::MAX - 1,
            },
            |a, b| Cube {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
                z: a.z.min(b.z),
            },
        );
        let max_cube = self.cubes.iter().fold(
            Cube {
                x: i32::MIN + 1,
                y: i32::MIN + 1,
                z: i32::MIN + 1,
            },
            |a, b| Cube {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
                z: a.z.max(b.z),
            },
        );
        Cube {
            x: min_cube.x - 1,
            y: min_cube.y - 1,
            z: min_cube.z - 1,
        }..=Cube {
            x: max_cube.x + 1,
            y: max_cube.y + 1,
            z: max_cube.z + 1,
        }
    }

    pub fn flood_exterior(&mut self) {
        let mut stack = vec![self.surrounding_range().start().clone()];
        while let Some(cube) = stack.pop() {
            cube.adjacent()
                .filter(|ac| ac.x >= self.range.start().x && ac.x <= self.range.end().x)
                .filter(|ac| ac.y >= self.range.start().y && ac.y <= self.range.end().y)
                .filter(|ac| ac.z >= self.range.start().z && ac.z <= self.range.end().z)
                .filter(|ac| !self.cubes.contains(ac))
                .for_each(|neighbor| stack.push(neighbor));
            self.cubes.insert(cube);
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = input.parse::<Grid>().ok()?;
    Some(grid.exposed_area())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut grid = input.parse::<Grid>().ok()?;
    let area_before_flood = grid.exposed_area();
    grid.flood_exterior();
    let area_after_flood = grid.exposed_area();
    Some(area_before_flood - area_after_flood)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
