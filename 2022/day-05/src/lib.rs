extern crate core;

use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Vec<char>>,
    instructions: Vec<(u32, u32, u32)>,
}

impl Ship {
    fn create(input: &str) -> Ship {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let mut instructions: Vec<(u32, u32, u32)> = Vec::new();

        input.split("\n").for_each(|line| {
            if line.starts_with("move") {
                let (count, from, to) = line.strip_prefix("move ").unwrap()
                    .split(" ")
                    .enumerate()
                    .filter_map(|it| if (it.0 + 1) % 2 == 1 { Some(it.1.parse::<u32>().unwrap()) } else { None })
                    .collect_tuple()
                    .unwrap();
                instructions.push((count, from, to));
            } else if !line.trim().starts_with('1') && !line.trim().is_empty() {
                line.chars().chunks(4).into_iter()
                    .enumerate()
                    .for_each(|it| {
                        let chars: Vec<char> = it.1.collect();

                        if stacks.len() <= it.0 {
                            stacks.push(Vec::new());
                        }

                        match chars.iter().find(|ch| ch.is_alphabetic()) {
                            Some(ch) => stacks[it.0].push(*ch),
                            None => {}
                        }
                    })
            }
        });

        return Ship {
            stacks: stacks.iter().map(|it| it.iter().rev().map(|it| *it).collect()).collect(),
            instructions,
        };
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut ship: Ship = Ship::create(input);

    ship.instructions.iter_mut().for_each(|it| {
        let (count, from_idx, to_idx) = *it;
        let from: &mut Vec<char> = ship.stacks.get_mut((from_idx - 1) as usize).unwrap();
        let crates: Vec<char> = (0..count).map(|_| from.pop().expect("No more crates left")).collect();

        let to: &mut Vec<char> = ship.stacks.get_mut((to_idx - 1) as usize).unwrap();
        crates.iter().for_each(|it| to.push(*it));
    });

    let result: Vec<char> = ship.stacks.iter()
        .filter_map(|it| it.last())
        .map(|it| *it)
        .collect();

    return String::from_iter(result);
}

pub fn solve_part2(input: &str) -> String {
    let mut ship: Ship = Ship::create(input);

    ship.instructions.iter_mut().for_each(|it| {
        let (count, from_idx, to_idx) = *it;
        let from: &mut Vec<char> = ship.stacks.get_mut((from_idx - 1) as usize).unwrap();
        let crates: Vec<char> = (0..count).map(|_| from.pop().expect("No more crates left")).rev().collect();

        let to: &mut Vec<char> = ship.stacks.get_mut((to_idx - 1) as usize).unwrap();
        crates.iter().rev().for_each(|it| to.push(*it));
    });

    let result: Vec<char> = ship.stacks.iter()
        .filter_map(|it| it.last())
        .map(|it| *it)
        .collect();

    return String::from_iter(result);
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "MCD");
    }
}