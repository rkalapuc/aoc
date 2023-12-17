use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use itertools::Itertools;
use ndarray::Array2;
use petgraph::algo::dijkstra;
use petgraph::graphmap::DiGraphMap;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Default for Position {
    fn default() -> Self {
        return Position { x: 0, y: 0 };
    }
}

impl Position {
    fn is_inside(&self, shape: &[i32]) -> bool {
        return self.x >= 0 && self.x < shape[0] && self.y >= 0 && self.y < shape[1];
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::UP => write!(f, "^"),
            Direction::RIGHT => write!(f, ">"),
            Direction::DOWN => write!(f, "v"),
            Direction::LEFT => write!(f, "<"),
        }
    }
}

impl Direction {
    fn turns(&self) -> [Direction; 2] {
        match self {
            Direction::UP | Direction::DOWN => [Direction::LEFT, Direction::RIGHT],
            Direction::RIGHT | Direction::LEFT => [Direction::UP, Direction::DOWN],
        }
    }

    fn next(&self, pos: &Position) -> Position {
        return match self {
            Direction::UP => Position { x: pos.x - 1, y: pos.y },
            Direction::RIGHT => Position { x: pos.x, y: pos.y + 1 },
            Direction::DOWN => Position { x: pos.x + 1, y: pos.y },
            Direction::LEFT => Position { x: pos.x, y: pos.y - 1 }
        };
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
struct Node {
    position: Position,
    direction: Direction,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[({}, {}):{:?}]", self.position.x, self.position.y, self.direction)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[({}, {}):{:?}]", self.position.x, self.position.y, self.direction)
    }
}

impl Node {
    fn create(position: &Position, direction: &Direction) -> Node {
        return Node { position: position.clone(), direction: direction.clone() };
    }
}

#[derive(Debug)]
struct Map {
    data: Array2<u8>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<u8>> = input.split('\n')
            .map(|line| line.chars().map(|it| it.to_digit(10).unwrap() as u8).collect())
            .collect();
        return Ok(Map {
            data: Array2::from_shape_vec(
                (data.len(), data.first().unwrap().len()),
                data.into_iter().flatten().collect(),
            ).unwrap()
        });
    }
}

impl Map {
    fn start_positions(&self) -> [Node; 2] {
        return [
            Node { position: Position::default(), direction: Direction::UP },
            Node { position: Position::default(), direction: Direction::LEFT }
        ];
    }

    fn finish_positions(&self) -> [Node; 2] {
        let shape: &[usize] = self.data.shape();
        return [
            Node {
                position: Position { x: shape[0] as i32 - 1, y: shape[1] as i32 - 1 },
                direction: Direction::DOWN,
            },
            Node {
                position: Position { x: shape[0] as i32 - 1, y: shape[1] as i32 - 1 },
                direction: Direction::RIGHT,
            }
        ];
    }

    fn get_cost(&self, position: &Position) -> u8 {
        return self.data[[position.x as usize, position.y as usize]];
    }

    fn to_graph(&self, min_blocks: u32, max_blocks: u32) -> DiGraphMap<Node, u32> {
        let shape: [i32; 2] = [self.data.shape()[0] as i32, self.data.shape()[1] as i32];

        let mut graph: DiGraphMap<Node, u32> = DiGraphMap::new();
        self.data.indexed_iter().for_each(|(pos, _)| {
            for direction in [Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT] {
                let position: Position = Position { x: pos.0 as i32, y: pos.1 as i32 };

                let node: Node = Node::create(&position, &direction);
                graph.add_node(node);

                for turn in direction.turns() {
                    let mut next_position = position;
                    let _ = (0..max_blocks).fold(0, |acc, idx| {
                        next_position = turn.next(&next_position);
                        if next_position.is_inside(&shape[0..2]) {
                            let cost = acc + self.get_cost(&next_position) as u32;
                            if idx >= min_blocks {
                                graph.add_edge(node, Node::create(&next_position, &turn), cost);
                            }
                            return cost;
                        }
                        return acc;
                    });
                }
            }
        });

        return graph;
    }

    fn min_heat_loss(&self, min_blocks: u32, max_blocks: u32) -> u32 {
        let graph: DiGraphMap<Node, u32> = self.to_graph(min_blocks, max_blocks);
        return self.start_positions().iter()
            .map(|start| dijkstra(&graph, *start, None, |(_, _, w)| *w))
            .cartesian_product(self.finish_positions().iter())
            .map(|(short_path, finish)| *short_path.get(finish).unwrap())
            .min().unwrap();
    }
}

pub fn solve_part1(input: &str) -> String {
    let map: Map = input.parse::<Map>().unwrap();
    let result = map.min_heat_loss(0, 3);
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let map: Map = input.parse::<Map>().unwrap();
    let result = map.min_heat_loss(3, 10);
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const INPUT2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT1);
        assert_eq!(result, "102");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT1);
        assert_eq!(result, "94");
        let result = solve_part2(INPUT2);
        assert_eq!(result, "71");
    }
}