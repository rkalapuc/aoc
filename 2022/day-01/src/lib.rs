use std::cmp::Reverse;
use std::path::PathBuf;

use clap::Parser;
use sorted_vec::ReverseSortedVec;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

pub fn solve_part1(input: &str) -> String {
    let result = (input.to_owned() + "\n").split('\n').fold((0, 0), |acc, it| {
        if it.len() == 0 {
            return if acc.1 < acc.0 { (0, acc.0) } else { (0, acc.1) };
        }
        return (acc.0 + it.parse::<i32>().unwrap(), acc.1);
    }).1;
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let mut sorted_vec: ReverseSortedVec<i32> = ReverseSortedVec::new();
    let mut calories: i32 = 0;

    (input.to_owned() + "\n").split('\n').for_each(|it| {
        if it.len() == 0 {
            sorted_vec.push(Reverse(calories));
            calories = 0;
        } else {
            calories = calories + it.parse::<i32>().unwrap();
        }
    });

    let result: i32 = sorted_vec.iter()
        .take(3)
        .fold(0, |acc, it| acc + it.0);

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "45000");
    }
}