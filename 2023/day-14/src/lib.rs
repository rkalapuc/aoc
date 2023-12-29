use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use itertools::{FoldWhile, Itertools};
use ndarray::{Array2, ArrayViewMut1};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum PlatformItem {
    RoundRock,
    CubeRock,
    EmptySpace,
}

#[derive(PartialEq, Eq)]
enum TiltDirection {
    North,
    West,
    South,
    East,
}

impl Debug for PlatformItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformItem::RoundRock => write!(f, "0"),
            PlatformItem::CubeRock => write!(f, "#"),
            PlatformItem::EmptySpace => write!(f, ".")
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct Platform {
    data: Array2<PlatformItem>,
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<PlatformItem>> = input.split('\n')
            .map(|line| line.chars()
                .map(|it| match it {
                    'O' => PlatformItem::RoundRock,
                    '#' => PlatformItem::CubeRock,
                    '.' => PlatformItem::EmptySpace,
                    it => panic!("Invalid platform item: {}", it)
                }).collect()
            ).collect();

        return Ok(Platform {
            data: Array2::from_shape_vec(
                (data.len(), data.first().unwrap().len()),
                data.into_iter().flatten().collect(),
            ).unwrap()
        });
    }
}

impl Platform {
    fn tilt(&self, direction: TiltDirection) -> Platform {
        let mut titled: Platform = self.clone();
        let shape = titled.data.shape();

        match direction {
            TiltDirection::North | TiltDirection::South => {
                for idx in 0..shape[1] {
                    if direction == TiltDirection::North {
                        Platform::tilt_forward(titled.data.column_mut(idx))
                    } else {
                        Platform::tilt_backward(titled.data.column_mut(idx))
                    }
                }
            }
            TiltDirection::West | TiltDirection::East => {
                for idx in 0..shape[0] {
                    if direction == TiltDirection::West {
                        Platform::tilt_forward(titled.data.row_mut(idx))
                    } else {
                        Platform::tilt_backward(titled.data.row_mut(idx))
                    }
                }
            }
        }

        return titled;
    }

    fn tilt_cycle(&self) -> Platform {
        return self.tilt(TiltDirection::North)
            .tilt(TiltDirection::West)
            .tilt(TiltDirection::South)
            .tilt(TiltDirection::East);
    }

    fn tilt_forward(mut view: ArrayViewMut1<PlatformItem>) {
        let mut next_pos: usize = 0;
        for idx in 0..view.len() {
            match view[idx] {
                PlatformItem::CubeRock => next_pos = idx + 1,
                PlatformItem::RoundRock => {
                    view.swap(next_pos, idx);
                    next_pos += 1;
                }
                _ => {}
            }
        }
    }

    fn tilt_backward(mut view: ArrayViewMut1<PlatformItem>) {
        let mut next_pos: i32 = view.len() as i32 - 1;
        for idx in (0..view.len()).rev() {
            match view[idx] {
                PlatformItem::CubeRock => next_pos = idx as i32 - 1,
                PlatformItem::RoundRock => {
                    view.swap(next_pos as usize, idx);
                    next_pos -= 1;
                }
                _ => {}
            }
        }
    }

    fn total_load(&self) -> u32 {
        self.data.columns().into_iter().fold(0, |acc, column| {
            return acc + column.indexed_iter().fold(0, |acc, (pos, it)| {
                return if *it == PlatformItem::RoundRock {
                    acc + column.len() as u32 - pos as u32
                } else {
                    acc
                };
            });
        })
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return hasher.finish();
    }
}

pub fn solve_part1(input: &str) -> String {
    let platform: Platform = input.parse::<Platform>().unwrap();
    let result = platform.tilt(TiltDirection::North).total_load();
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let mut cache: HashMap<u64, (usize, Platform)> = HashMap::new();

    let platform: Platform = input.parse::<Platform>().unwrap();
    cache.insert(platform.get_hash(), (0, platform.clone()));

    let mut titled: Platform = platform;
    let cycles_count: usize = 1000000000;

    let position = (1..cycles_count).fold_while(0, |acc, idx| {
        titled = titled.tilt_cycle();

        let hash: u64 = titled.get_hash();
        if cache.contains_key(&hash) {
            let existing = cache.get(&hash).unwrap();
            let position = (cycles_count - (*existing).0) % (idx - (*existing).0) + (*existing).0;
            return FoldWhile::Done(position);
        }

        cache.insert(hash, (idx, titled.clone()));
        return FoldWhile::Continue(acc);
    }).into_inner();

    let result: u32 = cache.iter()
        .find(|it| it.to_owned().1.to_owned().0 == position)
        .expect("Failed to find resulted platform").1.to_owned().1.total_load();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "136");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "64");
    }
}