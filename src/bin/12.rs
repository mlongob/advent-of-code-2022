use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct HeightMap {
    pub graph: Graph<(), (), Directed>,
    pub start: NodeIndex,
    pub goal: NodeIndex,
    pub low_points: HashSet<NodeIndex>,
}

impl FromStr for HeightMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<(i32, i32), (char, NodeIndex)> = HashMap::new();
        let mut graph: Graph<(), (), Directed> = Graph::new();
        let mut low_points: HashSet<NodeIndex> = HashSet::new();
        let mut start: NodeIndex = NodeIndex::new(0);
        let mut goal: NodeIndex = NodeIndex::new(0);
        for (col, l) in s.lines().enumerate() {
            for (row, c) in l.chars().enumerate() {
                let node = graph.add_node(());
                let height = match c {
                    'S' => {
                        start = node;
                        'a'
                    }
                    'E' => {
                        goal = node;
                        'z'
                    }
                    'a' => {
                        low_points.insert(node);
                        'a'
                    }
                    x => x,
                };
                map.insert((col as i32, row as i32), (height, node));
            }
        }
        for ((col, row), (c, node)) in &map {
            for (c2, r2) in [
                (col - 1, *row),
                (col + 1, *row),
                (*col, row - 1),
                (*col, row + 1),
            ]
            .into_iter()
            {
                if let Some((adj_c, adj_node)) = map.get(&(c2, r2)) {
                    if (*c as u32) + 1 >= (*adj_c as u32) {
                        graph.add_edge(*adj_node, *node, ());
                    }
                }
            }
        }
        Ok(HeightMap {
            graph,
            start,
            goal,
            low_points,
        })
    }
}

impl HeightMap {
    pub fn shortest_paths_to_goal(&self) -> HashMap<NodeIndex, i32> {
        dijkstra(&self.graph, self.goal, None, |_| 1)
    }

    pub fn shortest_start_goal_path(&self) -> Option<u32> {
        self.shortest_paths_to_goal()
            .get(&self.start)
            .map(|v| *v as u32)
    }

    pub fn shortest_hiking_trail(&self) -> Option<u32> {
        let paths = self.shortest_paths_to_goal();
        Some(
            *self
                .low_points
                .iter()
                .filter_map(|node| paths.get(node))
                .min()? as u32,
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let height_map: HeightMap = input.parse().ok()?;
    height_map.shortest_start_goal_path()
}

pub fn part_two(input: &str) -> Option<u32> {
    let height_map: HeightMap = input.parse().ok()?;
    height_map.shortest_hiking_trail()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
