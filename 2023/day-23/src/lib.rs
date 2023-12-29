use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

use clap::Parser;
use linked_hash_set::LinkedHashSet;
use ndarray::Array2;
use petgraph::Outgoing;
use petgraph::prelude::DiGraphMap;

use crate::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::TrailItem::{Forest, Path, Slop};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UP => write!(f, "^"),
            RIGHT => write!(f, ">"),
            DOWN => write!(f, "v"),
            LEFT => write!(f, "<")
        }
    }
}

impl Direction {
    fn next(&self, pos: &Position) -> Position {
        return match self {
            UP => Position { x: pos.x - 1, y: pos.y },
            RIGHT => Position { x: pos.x, y: pos.y + 1 },
            DOWN => Position { x: pos.x + 1, y: pos.y },
            LEFT => Position { x: pos.x, y: pos.y - 1 }
        };
    }
}

#[derive(Eq, PartialEq)]
enum TrailItem {
    Path,
    Forest,
    Slop(Direction),
}

impl Debug for TrailItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Path => write!(f, "."),
            Forest => write!(f, "#"),
            Slop(direction) => write!(f, "{:?}", direction),
        }
    }
}

impl TrailItem {
    fn next(&self, pos: &Position) -> Vec<Position> {
        return match self {
            Path => [UP, LEFT, DOWN, RIGHT].iter().map(|it| it.next(pos)).collect(),
            Forest => Vec::new(),
            Slop(direction) => vec![direction.next(pos)]
        };
    }
}

#[derive(Debug)]
struct Trails {
    data: Array2<TrailItem>,
    shape: (i32, i32),
}

impl Trails {
    fn create(input: &str, skip_slopes: bool) -> Trails {
        let data: Vec<Vec<TrailItem>> = input.split('\n')
            .map(|line| line.chars()
                .map(|it| match it {
                    '.' => Path,
                    '#' => Forest,
                    '>' => if skip_slopes { Path } else { Slop(RIGHT) },
                    '<' => if skip_slopes { Path } else { Slop(LEFT) },
                    '^' => if skip_slopes { Path } else { Slop(UP) },
                    'v' => if skip_slopes { Path } else { Slop(DOWN) },
                    invalid => panic!("Invalid item: {}", invalid)
                }).collect()
            ).collect();

        let shape: (i32, i32) = (data.len() as i32, data.first().unwrap().len() as i32);

        return Trails {
            data: Array2::from_shape_vec(
                (data.len(), data.first().unwrap().len()),
                data.into_iter().flatten().collect(),
            ).unwrap(),
            shape,
        };
    }

    fn is_path(&self, pos: &Position) -> bool {
        return pos.x >= 0 && pos.x < self.shape.0 && pos.y >= 0 && pos.y < self.shape.1 && self.data[[pos.x as usize, pos.y as usize]] != Forest;
    }

    fn next(&self, pos: &Position) -> Vec<Position> {
        return self.data[[pos.x as usize, pos.y as usize]].next(pos).iter().filter_map(|pos| {
            return if self.is_path(pos) { Some(*pos) } else { None };
        }).collect::<Vec<Position>>();
    }

    fn to_graph(&self) -> DiGraphMap<Position, usize> {
        let start: Position = Position { x: 0, y: 1 };
        let finish: Position = Position { x: self.shape.0 - 1, y: self.shape.1 - 2 };

        let mut split_points: LinkedHashSet<Position> = LinkedHashSet::new();
        split_points.insert(start);
        split_points.insert(finish);

        split_points.extend(
            self.data.indexed_iter().filter_map(|((x, y), item)| {
                let pos: Position = Position { x: x as i32, y: y as i32 };
                return if *item != Forest && self.next(&pos).len() >= 3 { Some(pos) } else { None };
            })
        );

        let mut graph: DiGraphMap<Position, usize> = DiGraphMap::new();

        for point in split_points.iter() {
            let mut queue: VecDeque<(Position, usize)> = VecDeque::from([(*point, 0)]);
            let mut visited: HashSet<Position> = HashSet::from([*point]);

            loop {
                if queue.is_empty() {
                    break;
                }

                let (pos, distance) = queue.pop_back().unwrap();
                let item = &self.data[[pos.x as usize, pos.y as usize]];

                if distance > 0 && split_points.contains(&pos) {
                    graph.add_edge(*point, pos, distance);
                    continue;
                }

                for next_pos in item.next(&pos).iter().filter(|it| self.is_path(it)) {
                    if !visited.contains(next_pos) {
                        queue.push_back((*next_pos, distance + 1));
                        visited.insert(*next_pos);
                    }
                }
            }
        }

        return graph;
    }

    fn find_longest(&self) -> usize {
        let graph: DiGraphMap<Position, usize> = self.to_graph();
        let start: Position = Position { x: 0, y: 1 };

        return self.find_longest_dfs(&graph, &start, &mut HashSet::new(), 0);
    }

    fn find_longest_dfs(&self, graph: &DiGraphMap<Position, usize>, from: &Position, visited: &mut HashSet<Position>, distance: usize) -> usize {
        if from.x == self.shape.0 - 1 && from.y == self.shape.1 - 2 {
            return distance;
        }

        let mut longest: usize = 0;
        graph.edges_directed(*from, Outgoing).for_each(|(_, next, next_distance)| {
            if visited.insert(next) {
                longest = max(longest, self.find_longest_dfs(graph, &next, visited, distance + next_distance));
                visited.remove(&next);
            }
        });

        return longest;
    }
}

pub fn solve_part1(input: &str) -> String {
    let trails: Trails = Trails::create(input, false);
    let result: usize = trails.find_longest();
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let trails: Trails = Trails::create(input, true);
    let result: usize = trails.find_longest();
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";


    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "94");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "154");
    }
}