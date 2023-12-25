use std::cmp::min;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use itertools::Itertools;
use ndarray::Array2;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

struct FlowNetwork {
    graph: UnGraph<String, usize>,
}

impl FromStr for FlowNetwork {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut graph: UnGraph<String, usize> = UnGraph::new_undirected();
        let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
        let lines: Vec<&str> = input.split('\n').collect();

        lines.clone().iter().for_each(|line| {
            let parts: Vec<&str> = line.split(':').collect();

            let start = parts.first().unwrap();
            let start_idx: NodeIndex = if nodes.contains_key(start) {
                *nodes.get(start).unwrap()
            } else {
                let idx = graph.add_node(start.to_string());
                nodes.insert(start, idx);
                idx
            };

            parts.last().unwrap().trim().split(' ').for_each(|node| {
                let node_idx: NodeIndex = if nodes.contains_key(node) {
                    *nodes.get(node).unwrap()
                } else {
                    let idx = graph.add_node(node.to_string());
                    nodes.insert(node, idx);
                    idx
                };
                graph.add_edge(start_idx, node_idx, 0);
            });
        });

        return Ok(FlowNetwork { graph });
    }
}

impl FlowNetwork {
    fn augment(&self, from: &NodeIndex, to: &NodeIndex, delta: i32, flow: &mut Array2<i32>, visited: &mut Vec<bool>) -> i32 {
        visited[from.index()] = true;
        if from == to {
            return delta;
        }

        for next in self.graph.edges(*from).map(|it| it.target()).sorted_by(|a, b| a.index().cmp(&b.index())) {
            let next_flow: i32 = flow[[from.index(), next.index()]];
            if !visited[next.index()] && next_flow < 1 {
                let next_delta: i32 = self.augment(&next, to, min(delta, 1 - next_flow), flow, visited);
                if next_delta > 0 {
                    flow[[from.index(), next.index()]] += next_delta;
                    flow[[next.index(), from.index()]] -= next_delta;
                    return next_delta;
                }
            }
        }

        return 0;
    }

    fn max_flow(&self, from: &NodeIndex, to: &NodeIndex) -> i32 {
        let nodes_count: usize = self.graph.node_count();
        let mut flow: Array2<i32> = Array2::zeros([nodes_count, nodes_count]);

        loop {
            let mut visited: Vec<bool> = (0..nodes_count).map(|_| false).collect();
            if self.augment(from, to, i32::MAX, &mut flow, &mut visited) <= 0 {
                return flow.row(from.index()).sum();
            }
        }
    }
}

// based on https://brilliant.org/wiki/ford-fulkerson-algorithm/
pub fn solve_part1(input: &str) -> String {
    let flow_network: FlowNetwork = input.parse::<FlowNetwork>().unwrap();

    let nodes_count: usize = flow_network.graph.node_count();
    let start_idx: NodeIndex = NodeIndex::new(0);

    let result = (1..nodes_count).fold((1, 0), |acc, node_idx| {
        return if flow_network.max_flow(&start_idx, &NodeIndex::new(node_idx)) == 3 {
            (acc.0, acc.1 + 1)
        } else {
            (acc.0 + 1, acc.1)
        };
    });

    return (result.0 * result.1).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "54");
    }
}