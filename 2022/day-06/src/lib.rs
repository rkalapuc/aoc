use std::io::Read;
use std::path::PathBuf;
use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

pub fn solve_part1(input: &str) -> String {
    for (idx, window) in input.bytes().into_iter().tuples::<(_,_,_,_)>().enumerate() {

    }

    return String::new();
}

pub fn solve_part2(input: &str) -> String {
    return String::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1A: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT1B: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT1C: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT1D: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT1E: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    const INPUT2: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT1A);
        assert_eq!(result, "7");

        let result = solve_part1(INPUT1B);
        assert_eq!(result, "5");

        let result = solve_part1(INPUT1C);
        assert_eq!(result, "6");

        let result = solve_part1(INPUT1D);
        assert_eq!(result, "10");

        let result = solve_part1(INPUT1E);
        assert_eq!(result, "11");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT2);
        assert_eq!(result, "");
    }
}