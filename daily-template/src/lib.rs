use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

pub fn solve_part1(input: &str) -> String {
    return String::new();
}

pub fn solve_part2(input: &str) -> String {
    return String::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "";

    const INPUT2: &str = "";

    #[test]
    fn test_part1() {
        todo!("haven't built test yet");
        let result = solve_part1(INPUT1);
        assert_eq!(result, "");
    }

    #[test]
    fn test_part2() {
        todo!("haven't built test yet");
        let result = solve_part2(INPUT2);
        assert_eq!(result, "");
    }
}