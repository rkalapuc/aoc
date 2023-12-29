use std::cmp::min;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use itertools::{FoldWhile, Itertools};
use ndarray::Array2;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug)]
struct Pattern {
    data: Array2<bool>
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<bool>> = input.split('\n')
            .map(|line| line.chars().map(|it| it == '#').collect())
            .collect();

        return Ok(Pattern {
            data: Array2::from_shape_vec(
                (data.len(), data.first().unwrap().len()),
                data.into_iter().flatten().collect(),
            ).unwrap()
        });
    }
}

impl Pattern {
    fn compare_rows(&self, idx1: usize, idx2: usize, max_diff: u32) -> u32 {
        return self.data.row(idx1).iter().enumerate().fold_while(0, |acc, it| {
            if acc > max_diff {
                return FoldWhile::Done(acc);
            }
            return if *it.1 != self.data[[idx2, it.0]] { FoldWhile::Continue(acc + 1) } else { FoldWhile::Continue(acc) };
        }).into_inner();
    }

    fn compare_columns(&self, idx1: usize, idx2: usize, max_diff: u32) -> u32 {
        return self.data.column(idx1).iter().enumerate().fold_while(0, |acc, it| {
            if acc > max_diff {
                return FoldWhile::Done(acc);
            }
            return if *it.1 != self.data[[it.0, idx2]] { FoldWhile::Continue(acc + 1) } else { FoldWhile::Continue(acc) };
        }).into_inner();
    }

    fn has_rows_reflection(&self, idx: usize, size: usize, max_diff: u32) -> bool {
        let result = (0..size).fold_while(0, |acc, it| {
            if acc > max_diff {
                return FoldWhile::Done(acc);
            }
            let result = self.compare_rows(idx - it - 1, idx + it, max_diff);
            return if result > 0 { FoldWhile::Continue(acc + 1) } else { FoldWhile::Continue(acc) };
        }).into_inner();
        return result <= max_diff;
    }

    fn has_columns_reflection(&self, idx: usize, size: usize, max_diff: u32) -> bool {
        let result = (0..size).fold_while(0, |acc, it| {
            if acc > max_diff {
                return FoldWhile::Done(acc);
            }
            return if self.compare_columns(idx - it - 1, idx + it, max_diff) > 0 { FoldWhile::Continue(acc + 1) } else { FoldWhile::Continue(acc) };
        }).into_inner();
        return result <= max_diff;
    }

    fn get_reflection_score(&self) -> u32 {
        let shape = self.data.shape();

        for idx in 1..shape[0] {
            let size = min(idx, shape[0] - idx);
            if self.has_rows_reflection(idx, size, 0) {
                return idx as u32 * 100;
            }
        }

        for idx in 1..shape[1] {
            let size = min(idx, shape[1] - idx);
            if self.has_columns_reflection(idx, size, 0) {
                return idx as u32;
            }
        }

        panic!("No reflection found: {:?}", self);
    }

    fn get_smudge_score(&self) -> u32 {
        let shape = self.data.shape();

        for idx in 0..shape[1] - 1 {
            let max_diff = (0..shape[1]).into_iter().fold_while(0, |acc, idy| {
                if acc > 1 {
                    return FoldWhile::Done(acc);
                }

                let start_idx = idx as i32 - idy as i32;
                let end_idx = idx + idy + 1;

                if start_idx >= 0 && start_idx < end_idx as i32 && end_idx < shape[1] {
                    return FoldWhile::Continue(acc + self.compare_columns(start_idx as usize, end_idx, 1));
                }

                return FoldWhile::Continue(acc);
            }).into_inner();

            if max_diff == 1 {
                return idx as u32 + 1;
            }
        }

        for idx in 0..shape[0] - 1 {
            let max_diff = (0..shape[0]).into_iter().fold_while(0, |acc, idy| {
                if acc > 1 {
                    return FoldWhile::Done(acc);
                }

                let start_idx = idx as i32 - idy as i32;
                let end_idx = idx + idy + 1;

                if start_idx >= 0 && start_idx < end_idx as i32 && end_idx < shape[0] {
                    return FoldWhile::Continue(acc + self.compare_rows(start_idx as usize, end_idx, 1));
                }

                return FoldWhile::Continue(acc);
            }).into_inner();

            if max_diff == 1 {
                return (idx as u32 + 1) * 100;
            }
        }

        panic!("No smudge found: {:?}", self);
    }
}

pub fn solve_part1(input: &str) -> String {
    let patterns: Vec<Pattern> = input.split("\n\n")
        .map(|it| it.parse::<Pattern>().unwrap())
        .collect();
    let result = patterns.iter()
        .fold(0, |acc, pattern| acc + pattern.get_reflection_score());
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let patterns: Vec<Pattern> = input.split("\n\n")
        .map(|it| it.parse::<Pattern>().unwrap())
        .collect();
    let result = patterns.iter()
        .fold(0, |acc, pattern| acc + pattern.get_smudge_score());
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "405");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "400");
    }
}