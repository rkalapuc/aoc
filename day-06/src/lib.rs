use std::path::PathBuf;
use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

fn calculate_options_dummy(time: u32, duration: u32) -> u32 {
    let mut options_count: u32 = 0;
    for idx in (1..time-1).rev() {
        let speed: u32 = time - idx;
        options_count += if (speed *  (time - speed)) > duration { 1 } else { 0 };
    }
    return options_count;
}

pub fn solve_part1(input: &str) -> String {
    let inputs: Vec<Vec<u32>> = input.split("\n").map(|line| {
        let numbers: Vec<u32> = line.split(":").last().unwrap().split(" ").filter_map(|it| {
            match it {
                "" => None,
                num => Some(num.trim().parse::<u32>().unwrap())
            }
        }).collect();
        return numbers;
    }).collect();

    let times: Vec<u32> = inputs.first().unwrap().to_owned();
    let durations: Vec<u32> = inputs.last().unwrap().to_owned();

    let result = times.iter().zip(durations.iter())
        .map(|tuple| calculate_options_dummy(*tuple.0, *tuple.1))
        .fold(1, |acc, it| acc * it);

    return result.to_string();
}

fn calculate_options_smart(time: u64, duration: u64) -> u64 {
    let discriminant_root: f64 = ((time.pow(2) - 4 * duration) as f64).sqrt();
    let min_x: f64 = (time as f64 - discriminant_root) / 2f64;
    let max_x: f64 = (time as f64 + discriminant_root) / 2f64;
    return (max_x.trunc() - min_x.trunc()) as u64;
}

pub fn solve_part2(input: &str) -> String {
    let inputs: Vec<u64> = input.split("\n").map(|line| {
        let number: String = line.split(":").last().unwrap().split(" ").filter_map(|it| {
            match it {
                "" => None,
                num => Some(num.trim())
            }
        }).intersperse("").collect();
        return number.parse::<u64>().unwrap();
    }).collect();

    let result: u64 = calculate_options_smart(*inputs.first().unwrap(), *inputs.last().unwrap());
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "288");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "71503");
    }
}