use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use ndarray::Array2;

use crate::Direction::{DOWN, LEFT, RIGHT, UP};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
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

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    GardenPlot,
    Rock,
    Start,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::GardenPlot => write!(f, "."),
            Tile::Rock => write!(f, "#"),
            Tile::Start => write!(f, "S"),
        }
    }
}

struct Garden {
    data: Array2<Tile>,
}

impl FromStr for Garden {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<Tile>> = input.split('\n')
            .map(|line| line.chars()
                .map(|it| match it {
                    '.' => Tile::GardenPlot,
                    '#' => Tile::Rock,
                    'S' => Tile::Start,
                    tile => panic!("Invalid tile: {}", tile)
                }).collect()
            ).collect();

        return Ok(Garden {
            data: Array2::from_shape_vec(
                (data.len(), data.first().unwrap().len()),
                data.into_iter().flatten().collect(),
            ).unwrap()
        });
    }
}

impl Garden {
    fn get_tile(&self, position: &Position) -> &Tile {
        let shape: &[usize] = self.data.shape();
        let x = (position.x % shape[0] as i32 + shape[0] as i32) % shape[0] as i32;
        let y = (position.y % shape[1] as i32 + shape[1] as i32) % shape[1] as i32;
        return &self.data[[x as usize, y as usize]];
    }

    fn find_start(&self) -> Position {
        return self.data.indexed_iter().find_map(|((x, y), tile)| {
            if *tile == Tile::Start { Some(Position { x: x as i32, y: y as i32 }) } else { None }
        }).unwrap();
    }

    fn travel(&self, prev_plots: &HashSet<Position>) -> HashSet<Position> {
        let directions: [Direction; 4] = [DOWN, UP, LEFT, RIGHT];

        let mut next_plots: HashSet<Position> = HashSet::new();
        for plot in prev_plots {
            next_plots.extend(
                directions.iter().filter_map(|it| {
                    let next_position: Position = it.next(&plot);
                    return if *self.get_tile(&next_position) != Tile::Rock { Some(next_position) } else { None };
                })
            )
        }

        return next_plots;
    }
}

pub fn solve_part1(input: &str, steps_count: u32) -> String {
    let garden: Garden = input.parse::<Garden>().unwrap();

    let plots: HashSet<Position> = (0..steps_count)
        .fold(HashSet::from([garden.find_start()]), |prev, _| garden.travel(&prev));

    let result: usize = plots.len();
    return result.to_string();
}

// took a while to spot a cycle, but looks like number of reachable plots is increased on some delta each "GRID SIZE" steps
// so solution can be split into 2 parts
// 1) find delta
// 2) calculate number of plots for 26501365 steps
pub fn solve_part2(input: &str, steps_count: usize) -> String {
    let garden: Garden = input.parse::<Garden>().unwrap();

    let shape: &[usize] = garden.data.shape();
    let center : usize = shape[0] / 2;
    let cycles_count: usize = 2;

    let (mut plots_counts, plots) = (0..center + shape[0] * cycles_count).fold(
        (Vec::new(), HashSet::from([garden.find_start()])),
        |(mut counts, prev), step| {
            if step == center || (step > center && (step - center) % shape[0] == 0) {
                counts.push(prev.len());
            }
            return (counts, garden.travel(&prev));
        },
    );

    plots_counts.push(plots.len());
    plots_counts.reverse();

    let delta: Vec<usize> = plots_counts.windows(2).map(|it| it[0] - it[1]).collect();
    let (result, _) = (0..(steps_count - center) / shape[0]).fold(
        (*plots_counts.last().unwrap(), *delta.last().unwrap()),
        |(total, step_delta), _| (total + step_delta, step_delta + delta[0] - delta[1]),
    );

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";


    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT1, 6);
        assert_eq!(result, "16");
    }
}