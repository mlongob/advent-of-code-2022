use anyhow::anyhow;
use itertools::Itertools;
use lazy_static::lazy_static;
use petgraph::algo::floyd_warshall;
use petgraph::prelude::*;
use petgraph::Graph;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Valve {
    pub name: String,
    pub rate: u32,
}

#[derive(Debug, Clone)]
pub struct ValveSystem {
    graph: Graph<Valve, u32, Directed>,
    start: NodeIndex,
}

impl FromStr for ValveSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const START_VALVE: &str = "AA";
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<tunnels>.+)$").expect("Regex did not compile");
        }
        let mut graph = Graph::new();
        let mut start = NodeIndex::new(0);
        let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();
        let mut raw_edges: Vec<(&str, &str)> = Vec::new();
        for l in s.lines() {
            let captures = RE
                .captures(l)
                .ok_or_else(|| anyhow!("Could not match regex"))?;
            let name = captures
                .name("name")
                .ok_or_else(|| anyhow!("Cannot extract valve name"))?
                .as_str();
            let rate = captures
                .name("rate")
                .ok_or_else(|| anyhow!("Cannot extract valve rate"))?
                .as_str()
                .parse::<u32>()?;
            let tunnels = captures
                .name("tunnels")
                .ok_or_else(|| anyhow!("Cannot extract valve tunnels"))?
                .as_str()
                .split(", ");
            for tunnel in tunnels {
                raw_edges.push((name, tunnel));
            }
            let node = graph.add_node(Valve {
                name: name.to_string(),
                rate: rate,
            });
            node_map.insert(name, node);
            if name == START_VALVE {
                start = node;
            }
        }
        for (from_raw, to_raw) in raw_edges {
            let from = *node_map
                .get(from_raw)
                .ok_or_else(|| anyhow!("Valve {from_raw} not found"))?;
            let to = *node_map
                .get(to_raw)
                .ok_or_else(|| anyhow!("Valve {to_raw} not found"))?;
            graph.add_edge(from, to, 1);
        }
        Ok(ValveSystem { start, graph })
    }
}

impl ValveSystem {
    pub fn optimize(&mut self) {
        let mut new_graph = Graph::new();
        let mut old_to_new: HashMap<NodeIndex, NodeIndex> = HashMap::new();
        let mut new_start = NodeIndex::new(0);

        // Only keep start node and nodes with positive rates
        self.graph.node_indices().for_each(|old_index| {
            let valve = self
                .graph
                .node_weight(old_index)
                .expect("Node should exist");
            if self.start == old_index || valve.rate > 0 {
                let new_index = new_graph.add_node(valve.clone());
                old_to_new.insert(old_index, new_index);
                if old_index == self.start {
                    new_start = new_index;
                }
            }
        });

        // Only keep best path edges between new nodes
        floyd_warshall(&self.graph, |_| 1)
            .expect("Cannot optimize: Invalid graph.")
            .iter()
            .filter(|((old_from, old_to), _)| old_from != old_to)
            .filter_map(|((old_from, old_to), w)| {
                let new_from = old_to_new.get(old_from)?;
                let new_to = old_to_new.get(old_to)?;
                Some(((new_from, new_to), w))
            })
            .for_each(|((from, to), weight)| {
                new_graph.add_edge(*from, *to, *weight);
            });

        self.graph = new_graph;
        self.start = new_start;
    }

    pub fn max_pressure_impl(
        &self,
        minutes: u32,
        node: NodeIndex,
        visited: HashSet<NodeIndex>,
        additional_run: Option<u32>,
        memo: &RefCell<HashMap<(u32, NodeIndex, Vec<NodeIndex>, Option<u32>), u32>>,
    ) -> u32 {
        let memo_key = (
            minutes,
            node,
            visited.iter().sorted().map(|x| *x).collect_vec(),
            additional_run,
        );
        if let Some(result) = memo.borrow().get(&memo_key) {
            return *result
        }

        let additional = match additional_run {
            // At any point in the search we should stop if the elephant can get more work done with the current visited set
            // the elephant would start from the beginning and be allocated the full time
            Some(minutes) => {
                self.max_pressure_impl(minutes, self.start, visited.clone(), None, memo)
            }
            None => 0,
        };

        let result = additional.max(
            // Return max pressure relief from visiting any adjacent edge
            self.graph
                .edges(node)
                .filter(|edge| !visited.contains(&edge.target()) && *edge.weight() < minutes)
                .map(|edge| {
                    // weight_minutes to get to it + 1 minute to open the valve
                    let minutes_spent = *edge.weight() + 1;
                    let minutes_remaining = minutes - minutes_spent;
                    let target_rate = self
                        .graph
                        .node_weight(edge.target())
                        .expect("Node must exist")
                        .rate;
                    let mut visited = visited.clone();
                    visited.insert(edge.target());
                    target_rate * minutes_remaining
                        + self.max_pressure_impl(
                            minutes_remaining,
                            edge.target(),
                            visited,
                            additional_run,
                            memo,
                        )
                })
                .max()
                .unwrap_or(0),
        );
        memo.borrow_mut().insert(memo_key, result);
        result
    }

    pub fn max_pressure(&self, you_minutes: u32, elephant_minutes: Option<u32>) -> u32 {
        let memo: RefCell<HashMap<(u32, NodeIndex, Vec<NodeIndex>, Option<u32>), u32>> =
            RefCell::new(HashMap::new());
        let mut visited = HashSet::new();
        // If start has no rate, don't stop there
        if self
            .graph
            .node_weight(self.start)
            .expect("Node should exist")
            .rate
            == 0
        {
            visited.insert(self.start);
        }
        self.max_pressure_impl(you_minutes, self.start, visited, elephant_minutes, &memo)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut valve_system = input.parse::<ValveSystem>().ok()?;
    //println!("{:?}", petgraph::dot::Dot::new(&valve_system.graph));
    valve_system.optimize();
    //println!("{:?}", petgraph::dot::Dot::new(&optimized.graph));
    Some(valve_system.max_pressure(30, None))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut valve_system = input.parse::<ValveSystem>().ok()?;
    valve_system.optimize();
    Some(valve_system.max_pressure(26, Some(26)))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
