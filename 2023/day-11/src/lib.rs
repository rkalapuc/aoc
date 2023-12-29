use std::cmp::{max, min};
use std::path::PathBuf;

use clap::Parser;
use ndarray::Array2;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

struct Galaxy {
    data: Array2<bool>,
}

impl Galaxy {
    fn create(input: &str) -> Galaxy {
        let data: Vec<Vec<bool>> = input.split('\n')
            .map(|line| line.chars().map(|it| it == '#').collect())
            .collect();

        return Galaxy {
            data: Array2::from_shape_vec(
                (data.len(), data.first().unwrap().len()),
                data.into_iter().flatten().collect(),
            ).unwrap()
        };
    }

    fn find_empty_rows(&self) -> Vec<usize> {
        return self.data.rows().into_iter()
            .enumerate()
            .filter_map(|it| {
                if it.1.iter().all(|it| !*it) {
                    return Some(it.0);
                }
                return None;
            }).collect();
    }

    fn find_empty_columns(&self) -> Vec<usize> {
        return self.data.columns().into_iter()
            .enumerate()
            .filter_map(|it| {
                if it.1.iter().all(|it| !*it) {
                    return Some(it.0);
                }
                return None;
            }).collect();
    }

    fn find_planets(&self) -> Vec<[usize; 2]> {
        return self.data.indexed_iter().filter_map(|it| {
            if *it.1 {
                return Some([it.0.0 as usize, it.0.1 as usize]);
            }
            return None;
        }).collect();
    }

    fn find_paths_sum(&self, planets: &Vec<[usize; 2]>, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>, scale: usize) -> usize {
        return planets.clone().into_iter().enumerate().map(|it| {
            return planets.iter().take(it.0).map(|p| {
                let mut distances: usize = 0;
                distances = distances + (min(it.1[0], p[0])..max(it.1[0], p[0])).map(|row_idx| {
                    if empty_rows.contains(&row_idx) {
                        return scale;
                    }
                    return 1;
                }).fold(0, |acc, x| acc + x);
                distances = distances + (min(it.1[1], p[1])..max(it.1[1], p[1])).map(|col_idx| {
                    if empty_cols.contains(&col_idx) {
                        return scale;
                    }
                    return 1;
                }).fold(0, |acc, x| acc + x);
                return distances;
            }).fold(0, |acc, x| acc + x);
        }).fold(0, |acc, x| acc + x);
    }
}

pub fn solve_part1(input: &str) -> String {
    let galaxy: Galaxy = Galaxy::create(input);

    let planets: Vec<[usize;2]> = galaxy.find_planets();
    let empty_rows: Vec<usize> = galaxy.find_empty_rows();
    let empty_cols: Vec<usize> = galaxy.find_empty_columns();

    return galaxy.find_paths_sum(&planets, &empty_rows, &empty_cols, 2).to_string();
}

pub fn solve_part2(input: &str) -> String {
    let galaxy: Galaxy = Galaxy::create(input);

    let planets: Vec<[usize;2]> = galaxy.find_planets();
    let empty_rows: Vec<usize> = galaxy.find_empty_rows();
    let empty_cols: Vec<usize> = galaxy.find_empty_columns();

    return galaxy.find_paths_sum(&planets, &empty_rows, &empty_cols, 1000000).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";


    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "374");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "82000210");
    }
}