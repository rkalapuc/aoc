use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

fn to_range(input: &str) -> (u32, u32) {
    return input.split("-")
        .map(|it| it.parse::<u32>().unwrap())
        .next_tuple()
        .unwrap();
}

pub fn solve_part1(input: &str) -> String {
    let result: u32 = input.split("\n").fold(0, |acc, chunk| {
        let parts: Vec<&str> = chunk.split(",").collect();

        let left: (u32, u32) = to_range(parts.first().unwrap());
        let right: (u32, u32) = to_range(parts.last().unwrap());

        let range: (u32, u32) = (
            if left.0 > right.0 { right.0 } else { left.0 },
            if left.1 > right.1 { left.1 } else { right.1 }
        );

        return acc + if left == range || right == range { 1u32 } else { 0u32 };
    });
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let result: u32 = input.split("\n").fold(0, |acc, chunk| {
        let parts: Vec<&str> = chunk.split(",").collect();

        let left: (u32, u32) = to_range(parts.first().unwrap());
        let right: (u32, u32) = to_range(parts.last().unwrap());

        let range: (u32, u32) = (
            if left.0 > right.0 { left.0 } else { right.0 },
            if left.1 > right.1 { right.1 } else { left.1 }
        );

        return acc + if range.0 <= range.1 { 1 } else { 0 };
    });
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "4");
    }
}