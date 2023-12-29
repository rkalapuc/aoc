use regex::Regex;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

pub fn solve_part1(input: &str) -> String {
    let re: Regex = Regex::new(r"(?i)(\d{1})").unwrap();

    let result: i64 = input.split("\n")
        .map(|line| {
            let numbers: Vec<i64> = re.find_iter(line)
                .map(|it| it.as_str().parse().ok().unwrap())
                .collect();
            return match numbers.len() {
                0 => 0,
                1 => numbers[0] * 10 + numbers[0],
                _n => numbers[0] * 10 + numbers.last().unwrap(),
            };
        }).sum();

    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let digits: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let rev_digits: Vec<String> = digits.iter().map(|&s| s.chars().rev().collect::<String>()).collect();

    let mut pattern = r"(?i)(\d{1}|".to_owned();
    pattern.push_str(&digits.join("|"));
    pattern.push_str(")");

    let mut rev_pattern = r"(?i)(\d{1}|".to_owned();
    rev_pattern.push_str(&rev_digits.join("|"));
    rev_pattern.push_str(")");

    let re: Regex = Regex::new(pattern.as_str()).unwrap();
    let rev_re: Regex = Regex::new(rev_pattern.as_str()).unwrap();

    let result: i64 = input.split("\n")
        .map(|line| {
            let mut numbers: Vec<i64> = vec![0,2];

            let first = re.find(line);
            if !first.is_none() {
                let group = first.unwrap().as_str();
                numbers[0] = match group.parse::<i64>() {
                    Ok(num) => num,
                    Err(_) => digits.iter().position(|&digit| digit == group).unwrap() as i64
                };
            } else {
                numbers[0] = 0;
            }

            let rev_line: String = line.chars().rev().collect();
            let last = rev_re.find(rev_line.as_str());
            if !last.is_none() {
                let group = last.unwrap().as_str();
                numbers[1] = match group.parse::<i64>() {
                    Ok(num) => num,
                    Err(_) => rev_digits.iter().position(|digit| digit.as_str() == group).unwrap() as i64
                };
            } else {
                numbers[1] = 0;
            }

            return numbers[0] * 10 + numbers[1];
        }).sum();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
7bbxlhgdbrh9sph44sbboneoneightxcn";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT1);
        assert_eq!(result, "142");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT2);
        assert_eq!(result, "359");
    }
}