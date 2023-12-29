use std::cmp::max;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use itertools::Itertools;
use ndarray::Array2;
use queues::{IsQueue, Queue};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BeamPosition {
    x: i32,
    y: i32,
}

impl Default for BeamPosition {
    fn default() -> Self {
        return BeamPosition { x: 0, y: 0 };
    }
}

impl BeamPosition {
    fn is_inside(&self, shape: &[i32]) -> bool {
        return self.x >= 0 && self.x < shape[0] && self.y >= 0 && self.y < shape[1];
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BeamDirection {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl BeamDirection {
    fn next(&self, pos: &BeamPosition) -> BeamPosition {
        return match self {
            BeamDirection::UP => BeamPosition { x: pos.x - 1, y: pos.y },
            BeamDirection::RIGHT => BeamPosition { x: pos.x, y: pos.y + 1 },
            BeamDirection::DOWN => BeamPosition { x: pos.x + 1, y: pos.y },
            BeamDirection::LEFT => BeamPosition { x: pos.x, y: pos.y - 1 }
        };
    }
}

enum ContraptionItem {
    UpwardMirror,
    DownwardMirror,
    HorizontalSplitter,
    VerticalSplitter,
    EmptySpace,
}

impl Debug for ContraptionItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContraptionItem::UpwardMirror => write!(f, "/"),
            ContraptionItem::DownwardMirror => write!(f, "\\"),
            ContraptionItem::VerticalSplitter => write!(f, "|"),
            ContraptionItem::HorizontalSplitter => write!(f, "-"),
            ContraptionItem::EmptySpace => write!(f, ".")
        }
    }
}

impl ContraptionItem {
    fn reflect(&self, direction: &BeamDirection) -> Vec<BeamDirection> {
        return match self {
            ContraptionItem::UpwardMirror => {
                return match direction {
                    BeamDirection::UP => vec![BeamDirection::RIGHT],
                    BeamDirection::RIGHT => vec![BeamDirection::UP],
                    BeamDirection::DOWN => vec![BeamDirection::LEFT],
                    BeamDirection::LEFT => vec![BeamDirection::DOWN],
                };
            }
            ContraptionItem::DownwardMirror => {
                return match direction {
                    BeamDirection::UP => vec![BeamDirection::LEFT],
                    BeamDirection::RIGHT => vec![BeamDirection::DOWN],
                    BeamDirection::DOWN => vec![BeamDirection::RIGHT],
                    BeamDirection::LEFT => vec![BeamDirection::UP]
                };
            }
            ContraptionItem::HorizontalSplitter => {
                return match direction {
                    BeamDirection::UP | BeamDirection::DOWN => vec![BeamDirection::LEFT, BeamDirection::RIGHT],
                    BeamDirection::RIGHT | BeamDirection::LEFT => vec![direction.clone()]
                };
            }
            ContraptionItem::VerticalSplitter => {
                return match direction {
                    BeamDirection::UP | BeamDirection::DOWN => vec![direction.clone()],
                    BeamDirection::RIGHT | BeamDirection::LEFT => vec![BeamDirection::UP, BeamDirection::DOWN]
                };
            }
            ContraptionItem::EmptySpace => vec![direction.clone()]
        };
    }
}

#[derive(Debug)]
struct Contraption {
    data: Array2<ContraptionItem>,
}

impl FromStr for Contraption {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<ContraptionItem>> = input.split('\n')
            .map(|line| line.chars()
                .map(|it| match it {
                    '/' => ContraptionItem::UpwardMirror,
                    '\\' => ContraptionItem::DownwardMirror,
                    '|' => ContraptionItem::VerticalSplitter,
                    '-' => ContraptionItem::HorizontalSplitter,
                    '.' => ContraptionItem::EmptySpace,
                    it => panic!("Invalid platform item: {}", it)
                }).collect()
            ).collect();

        return Ok(Contraption {
            data: Array2::from_shape_vec(
                (data.len(), data.first().unwrap().len()),
                data.into_iter().flatten().collect(),
            ).unwrap()
        });
    }
}

impl Contraption {
    fn get_item(&self, position: &BeamPosition) -> &ContraptionItem {
        return &self.data[[position.x as usize, position.y as usize]];
    }

    fn energize(&self, start: BeamPosition, direction: BeamDirection) -> usize {
        let shape: [i32; 2] = [self.data.shape()[0] as i32, self.data.shape()[1] as i32];

        let mut queue: Queue<(BeamPosition, BeamDirection)> = Queue::new();
        queue.add((start, direction)).unwrap();

        let mut energized: HashSet<(BeamPosition, BeamDirection)> = HashSet::new();
        energized.insert((start, direction));

        while queue.size() > 0 {
            // println!("---------------------------------------------------------");
            let (position, direction) = queue.remove().unwrap();
            // println!("position: {:?} | direction: {:?}", position, direction);

            let item = self.get_item(&position);
            // println!("item: {:?}", item);

            let next_directions: Vec<BeamDirection> = item.reflect(&direction);
            // println!("next_directions: {:?}", next_directions);

            let next_positions: Vec<(BeamPosition, BeamDirection)> = next_directions.iter()
                .map(|it| (it.next(&position), it.clone()))
                .filter(|(pos, _)| pos.is_inside(&shape[0..2]))
                .filter(|it| !energized.contains(it))
                .collect();

            // println!("next_positions: {:?}", next_positions);

            for item in next_positions {
                energized.insert(item);
                queue.add(item).unwrap();
            }
        }

        // let mut energized_array: Array2<i32> = Array2::<i32>::zeros((shape[0] as usize, shape[1] as usize));
        // energized.iter().map(|it| (*it).0).unique().for_each(|it| {
        //     energized_array[[it.x as usize, it.y as usize]] = 1;
        // });
        // println!("{:?}", energized_array);

        return energized.iter().map(|it| (*it).0).unique().count();
    }
}

pub fn solve_part1(input: &str) -> String {
    let contraption: Contraption = input.parse::<Contraption>().unwrap();
    let result = contraption.energize(BeamPosition::default(), BeamDirection::RIGHT);
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let contraption: Contraption = input.parse::<Contraption>().unwrap();

    let shape: &[usize] = contraption.data.shape();
    let left_right_max = (0..shape[0]).fold(0, |acc, idx| {
        return max(
            acc,
            max(
                contraption.energize(BeamPosition { x: idx as i32, y: shape[1] as i32 - 1 }, BeamDirection::LEFT),
                contraption.energize(BeamPosition { x: idx as i32, y: 0 }, BeamDirection::RIGHT),
            ),
        );
    });

    let up_down_max = (0..shape[1]).fold(0, |acc, idx| {
        return max(
            acc,
            max(
                contraption.energize(BeamPosition { x: 0, y: idx as i32 }, BeamDirection::DOWN),
                contraption.energize(BeamPosition { x: shape[0] as i32 - 1, y: idx as i32 }, BeamDirection::UP),
            ),
        );
    });

    let result = max(left_right_max, up_down_max);
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "46");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "51");
    }
}