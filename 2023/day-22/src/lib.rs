use std::cmp::{max, min};
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use itertools::Itertools;
use ndarray::Array2;
use petgraph::Direction;
use petgraph::prelude::DiGraphMap;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coords: Vec<usize> = input.split(",").map(|it| it.parse::<usize>().unwrap()).collect();
        return Ok(Point { x: coords[0], y: coords[1], z: coords[2] });
    }
}

#[derive(Debug)]
struct Snapshot {
    bricks: Vec<(Point, Point)>,
    bounds: (usize, usize),
}

impl FromStr for Snapshot {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut bricks: Vec<(Point, Point)> = input.split('\n')
            .map(|line| {
                let parts: Vec<&str> = line.split("~").collect();
                return (parts[0].parse::<Point>().unwrap(), parts[1].parse::<Point>().unwrap());
            }).collect();

        bricks.sort_by(|a, b| min(a.0.z, a.1.z).cmp(&min(b.0.z, b.1.z)));

        let bounds: (usize, usize) = bricks.iter().fold((0, 0), |acc, brick| {
            return (
                max(acc.0, max(brick.0.x, brick.1.x)),
                max(acc.1, max(brick.0.y, brick.1.y))
            );
        });

        return Ok(Snapshot { bricks, bounds: (bounds.0 + 1, bounds.1 + 1) });
    }
}

impl Snapshot {
    fn to_graph(&self) -> DiGraphMap<usize, usize> {
        let mut graph: DiGraphMap<usize, usize> = DiGraphMap::new();

        let mut z_offsets: Array2<usize> = Array2::zeros(self.bounds);
        let mut z_indexes: Array2<usize> = Array2::zeros(self.bounds);

        for (idx, brick) in self.bricks.iter().enumerate() {
            let mut z_offset: usize = 0;

            // calculate dependent bricks
            (brick.0.x..=brick.1.x).into_iter().cartesian_product((brick.0.y..=brick.1.y).into_iter()).for_each(|(x, y)| {
                if z_offsets[[x, y]] > z_offset {
                    z_offset = z_offsets[[x, y]];

                    // clean previous edges
                    graph.edges_directed(idx, Direction::Outgoing).map(|it| it.1)
                        .collect::<Vec<usize>>()
                        .into_iter()
                        .for_each(|it| { graph.remove_edge(idx, it); });

                    // add new one
                    graph.add_edge(idx, z_indexes[[x, y]], 0);
                } else if z_offsets[[x, y]] == z_offset && z_indexes[[x, y]] > 0 {
                    graph.add_edge(idx, z_indexes[[x, y]], 0);
                }
            });

            // update offsets and indexes
            (brick.0.x..=brick.1.x).into_iter().cartesian_product((brick.0.y..=brick.1.y).into_iter()).for_each(|(x, y)| {
                z_offsets[[x, y]] = z_offset + (brick.1.z - brick.0.z + 1);
                z_indexes[[x, y]] = idx;
            })
        }

        return graph;
    }
}

pub fn solve_part1(input: &str) -> String {
    let snapshot: Snapshot = input.parse::<Snapshot>().unwrap();
    let graph = snapshot.to_graph();

    let result = (0..snapshot.bricks.len()).fold(0, |acc, idx| {
        let can_be_disintegrated: bool = graph.neighbors_directed(idx, Direction::Incoming).all(|candidate_idx| {
            return graph.neighbors_directed(candidate_idx, Direction::Outgoing).count() > 1;
        });
        return if can_be_disintegrated { acc + 1 } else { acc };
    });

    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let snapshot: Snapshot = input.parse::<Snapshot>().unwrap();
    let graph = snapshot.to_graph();

    let result = (0..snapshot.bricks.len()).fold(0, |acc, idx| {
        let mut fallen: HashSet<usize> = HashSet::from([idx]);
        let mut queue: Vec<usize> = Vec::from([idx]);
        loop {
            let mut next_bricks: Vec<usize> = Vec::new();

            for brick_idx in queue.iter() {
                graph.neighbors_directed(*brick_idx, Direction::Incoming).for_each(|candidate_idx| {
                    if graph.neighbors_directed(candidate_idx, Direction::Outgoing).all(|it| fallen.contains(&it)) {
                        next_bricks.push(candidate_idx);
                        fallen.insert(candidate_idx);
                    }
                })
            }

            if next_bricks.is_empty() {
                break;
            }

            queue = next_bricks;
        }

        return acc + fallen.len() - 1;
    });

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "5");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "7");
    }
}