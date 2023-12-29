use std::collections::HashSet;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

pub fn solve_part1(input: &str) -> String {
    let result: u32 = input.split("\n").fold(0, |acc, line| {
        let parts: (&str, &str) = line.split_at(line.len() / 2);
        let set: HashSet<u8> = HashSet::from_iter(parts.1.bytes().into_iter());
        let char: u8 = parts.0.bytes().find(|it| set.contains(it)).unwrap();
        let score = if char >= 97 { char - 96 } else { char - 38 };
        return acc + score as u32;
    });
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let result: u32 = input.split("\n").collect::<Vec::<&str>>().chunks(3).into_iter().fold(0, |acc, chunk| {
        let set1: HashSet<u8> = HashSet::from_iter(chunk[1].bytes().into_iter());
        let set2: HashSet<u8> = HashSet::from_iter(chunk[2].bytes().into_iter());
        let char: u8 = chunk[0].bytes().find(|it| set1.contains(it) && set2.contains(it)).unwrap();
        let score = if char >= 97 { char - 96 } else { char - 38 };
        return acc + score as u32;
    });
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "70");
    }
}