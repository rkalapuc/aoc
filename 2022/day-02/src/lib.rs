use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

enum RPS {
    Rock,
    Paper,
    Scissors
}

enum Result {
    Win,
    Lose,
    Draw
}

impl RPS {
    fn create(char: &str) -> Option<RPS> {
        return match char {
            "A" => Some(RPS::Rock),
            "X" => Some(RPS::Rock),
            "B" => Some(RPS::Paper),
            "Y" => Some(RPS::Paper),
            "C" => Some(RPS::Scissors),
            "Z" => Some(RPS::Scissors),
            _ => None
        }
    }

    fn score(&self, other: &RPS) -> u8 {
        return match self {
            RPS::Rock => {
                1 + match other {
                    RPS::Rock => 3,
                    RPS::Paper => 0,
                    RPS::Scissors => 6,
                }
            }
            RPS::Paper => {
                2 + match other {
                    RPS::Rock => 6,
                    RPS::Paper => 3,
                    RPS::Scissors => 0
                }
            }
            RPS::Scissors => {
                3 + match other {
                    RPS::Rock => 0,
                    RPS::Paper => 6,
                    RPS::Scissors => 3
                }
            }
        }
    }

    fn score_for_result(&self, result: &Result) -> u8 {
        return match self {
            RPS::Rock => {
                match result {
                    Result::Win => 8,
                    Result::Lose => 3,
                    Result::Draw => 4
                }
            }
            RPS::Paper => {
                match result {
                    Result::Win => 9,
                    Result::Lose => 1,
                    Result::Draw => 5
                }
            }
            RPS::Scissors => {
                match result {
                    Result::Win => 7,
                    Result::Lose => 2,
                    Result::Draw => 6
                }
            }
        }
    }
}

impl Result {
    fn create(char: &str) -> Option<Result> {
        return match char {
            "X" => Some(Result::Lose),
            "Y" => Some(Result::Draw),
            "Z" => Some(Result::Win),
            _ => None
        }
    }
}

pub fn solve_part1(input: &str) -> String {
    let result: u32 = input.split("\n").fold(0, |acc, line| {
        let game: Vec<&str> = line.split(" ").collect();

        let left: RPS = RPS::create(game.first().unwrap()).expect("Invalid input");
        let right: RPS = RPS::create(game.last().unwrap()).expect("Invalid input");

        return acc + right.score(&left) as u32;
    });
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let result: u32 = input.split("\n").fold(0, |acc, line| {
        let game: Vec<&str> = line.split(" ").collect();

        let left: RPS = RPS::create(game.first().unwrap()).expect("Invalid input");
        let right: Result = Result::create(game.last().unwrap()).expect("Invalid input");

        return acc + left.score_for_result(&right) as u32;
    });
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";


    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "12");
    }
}