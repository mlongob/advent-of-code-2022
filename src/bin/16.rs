use anyhow::anyhow;
use lazy_static::lazy_static;
use petgraph::algo::floyd_warshall;
use petgraph::prelude::*;
use petgraph::Graph;
use petgraph::IntoWeightedEdge;
use regex::Regex;
use std::collections::BTreeSet;
use std::collections::HashMap;
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
                rate,
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

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MaxPressureInput {
    minutes: u32,
    node: NodeIndex,
    visited: BTreeSet<NodeIndex>,
    additional_run: Option<u32>,
}

impl ValveSystem {
    pub fn optimize(&mut self) {
        // We must rebuild start_value because indices get invalidated
        let start_valve = self
            .graph
            .node_weight(self.start)
            .expect("Node must exist")
            .clone();

        let fw_results =
            floyd_warshall(&self.graph, |e| *e.weight()).expect("Cannot optimize: Invalid graph");

        // Delete all edges and replace them with shortest-paths fully-connected edges from floyd_warshall
        self.graph.clear_edges();
        self.graph.extend_with_edges(
            fw_results
                .into_iter()
                .map(|((s, t), w)| (s, t, w).into_weighted_edge()),
        );

        // Only keep start node and nodes with positive rates
        self.graph.retain_nodes(|graph, idx| {
            let valve = graph.node_weight(idx).expect("Node must exist");
            *valve == start_valve || valve.rate > 0
        });

        // Rebuilds start value by finding the start node
        self.start = self
            .graph
            .node_indices()
            .find(|idx| self.graph.node_weight(*idx).expect("Node must exist") == &start_valve)
            .expect("Start node must exist");
    }

    fn max_pressure_impl(
        &self,
        input: MaxPressureInput,
        memo: &mut HashMap<MaxPressureInput, u32>,
    ) -> u32 {
        // Check if result has been cached
        if let Some(result) = memo.get(&input) {
            return *result;
        }

        let additional = match input.additional_run {
            // At any point in the search we should stop if the elephant can get more work done with the current visited set
            // the elephant would start from the beginning and be allocated the full time
            Some(minutes) => self.max_pressure_impl(
                MaxPressureInput {
                    minutes,
                    node: self.start,
                    additional_run: None,
                    visited: input.visited.clone(),
                },
                memo,
            ),
            None => 0,
        };

        let result = additional.max(
            // Return max pressure relief from visiting any adjacent edge
            self.graph
                .edges(input.node)
                .filter(|edge| {
                    !input.visited.contains(&edge.target()) && *edge.weight() < input.minutes
                })
                .map(|edge| {
                    // weight_minutes to get to it + 1 minute to open the valve
                    let minutes_spent = *edge.weight() + 1;
                    let minutes_remaining = input.minutes - minutes_spent;
                    let target_rate = self
                        .graph
                        .node_weight(edge.target())
                        .expect("Node must exist")
                        .rate;
                    let mut visited = input.visited.clone();
                    visited.insert(edge.target());
                    target_rate * minutes_remaining
                        + self.max_pressure_impl(
                            MaxPressureInput {
                                minutes: minutes_remaining,
                                node: edge.target(),
                                visited,
                                additional_run: input.additional_run,
                            },
                            memo,
                        )
                })
                .max()
                .unwrap_or(0),
        );
        // Cache result
        memo.insert(input, result);
        result
    }

    pub fn max_pressure(&self, you_minutes: u32, elephant_minutes: Option<u32>) -> u32 {
        let mut mp_input = MaxPressureInput {
            minutes: you_minutes,
            node: self.start,
            visited: BTreeSet::new(),
            additional_run: elephant_minutes,
        };
        let mut memo: HashMap<MaxPressureInput, u32> = HashMap::new();
        // If start has no rate, don't stop there
        if self
            .graph
            .node_weight(self.start)
            .expect("Node should exist")
            .rate
            == 0
        {
            mp_input.visited.insert(self.start);
        }
        self.max_pressure_impl(mp_input, &mut memo)
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
