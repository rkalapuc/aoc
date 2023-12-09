use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

struct History {
    data: Vec<Vec<i32>>,
}

impl History {
    fn create(input: &str) -> History {
        return History {
            data: input.split('\n').map(|line| {
                return line.split(' ').map(|it| it.trim().parse::<i32>().unwrap()).collect();
            }).collect()
        };
    }
}

pub fn solve_part1(input: &str) -> String {
    let history: History = History::create(input);

    let result: i32 = history.data.iter().map(|line| {
        let mut path: Vec<i32> = Vec::new();
        let mut step: Vec<i32> = line.clone();

        loop {
            path.push(*step.last().unwrap());
            step = step.windows(2).map(|it| it.last().unwrap() - it.first().unwrap()).collect();
            if step.iter().all(|it| it == &0) {
                break;
            }
        }

        return path.iter().fold(0, |acc, it| acc + *it);
    }).sum();

    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let history: History = History::create(input);

    let result: i32 = history.data.iter().map(|line| {
        let mut path: Vec<i32> = Vec::new();
        let mut step: Vec<i32> = line.clone();

        loop {
            path.push(*step.first().unwrap());
            step = step.windows(2).map(|it| it.last().unwrap() - it.first().unwrap()).collect();
            if step.iter().all(|it| it == &0) {
                break;
            }
        }

        return path.iter().rev().fold(0, |acc, it| *it - acc);
    }).sum();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "114");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "2");
    }
}