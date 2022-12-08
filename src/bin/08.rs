use ::take_until::TakeUntilExt;
use std::str::FromStr;

pub type TreeHeight = u32;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreeGrid {
    xy_grid: Vec<Vec<TreeHeight>>,
    yx_grid: Vec<Vec<TreeHeight>>,
}

impl TreeGrid {
    pub fn new() -> TreeGrid {
        TreeGrid {
            xy_grid: Vec::new(),
            yx_grid: Vec::new(),
        }
    }

    pub fn add_tree(&mut self, position: &Position, height: TreeHeight) {
        if (position.x) >= self.xy_grid.len() {
            self.xy_grid.resize(position.x + 1, Vec::new());
        }
        if (position.y) >= self.yx_grid.len() {
            self.yx_grid.resize(position.y + 1, Vec::new());
        }
        if (position.x) >= self.yx_grid[position.y].len() {
            self.yx_grid[position.y].resize(position.x + 1, 0);
        }
        if (position.y) >= self.xy_grid[position.x].len() {
            self.xy_grid[position.x].resize(position.y + 1, 0);
        }
        self.xy_grid[position.x][position.y] = height;
        self.yx_grid[position.y][position.x] = height;
    }

    pub fn get_tree(&self, position: &Position) -> &TreeHeight {
        &self.yx_grid[position.y][position.x]
    }

    pub fn iter(&self) -> impl Iterator<Item = (Position, &TreeHeight)> {
        self.yx_grid.iter().enumerate().flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .map(move |(x, height)| (Position { x, y }, height))
        })
    }

    fn bottom_view(&self, position: &Position) -> impl Iterator<Item = &TreeHeight> {
        self.xy_grid[position.x][position.y + 1..].iter()
    }

    fn top_view(&self, position: &Position) -> impl Iterator<Item = &TreeHeight> {
        self.xy_grid[position.x][0..position.y].iter().rev()
    }

    fn right_view(&self, position: &Position) -> impl Iterator<Item = &TreeHeight> {
        self.yx_grid[position.y][position.x + 1..].iter()
    }

    fn left_view(&self, position: &Position) -> impl Iterator<Item = &TreeHeight> {
        self.yx_grid[position.y][0..position.x].iter().rev()
    }

    fn views<'a>(
        &'a self,
        position: &Position,
    ) -> Vec<Box<dyn Iterator<Item = &'a TreeHeight> + 'a>> {
        vec![
            Box::new(self.top_view(position)),
            Box::new(self.right_view(position)),
            Box::new(self.bottom_view(position)),
            Box::new(self.left_view(position)),
        ]
    }

    pub fn visible_from_outside(&self, position: &Position) -> bool {
        let height = self.get_tree(position);
        self.views(position)
            .into_iter()
            .map(|mut iter| iter.all(|other| height > other))
            .any(|taller| taller)
    }

    pub fn scenic_score(&self, position: &Position) -> u32 {
        let height = self.get_tree(position);
        let score = self
            .views(position)
            .into_iter()
            .map(|iter| iter.take_until(|other| *other >= height).count())
            .reduce(|a, b| a * b)
            .unwrap_or(0);
        score as u32
    }
}

impl FromStr for TreeGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tree_grid =
            s.lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().filter_map(|c| c.to_digit(10)).enumerate().map(
                        move |(x, height)| {
                            let pos = Position { x, y };
                            (pos, height)
                        },
                    )
                })
                .fold(TreeGrid::new(), |mut tree_grid, (position, height)| {
                    tree_grid.add_tree(&position, height);
                    tree_grid
                });
        Ok(tree_grid)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree_grid: TreeGrid = input.parse().ok()?;
    let count_visible = tree_grid
        .iter()
        .filter(|(position, _)| tree_grid.visible_from_outside(position))
        .count();
    Some(count_visible as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree_grid: TreeGrid = input.parse().ok()?;
    let max_score = tree_grid
        .iter()
        .map(|(position, _)| tree_grid.scenic_score(&position))
        .max();
    max_score
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_score() {
        let input = advent_of_code::read_file("examples", 8);
        let tree_grid: TreeGrid = input.parse().unwrap();
        let position = &Position { x: 2, y: 1 };
        assert_eq!(tree_grid.scenic_score(&position), 4);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
