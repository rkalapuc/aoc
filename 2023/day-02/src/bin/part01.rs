use std::fs;
use clap::Parser;
use aoc2023_day_02::{Cli, solve_part1};
use aoc2023_day_02::CubesLimits;

fn main() {
    let args: Cli = Cli::parse();
    let input: String = fs::read_to_string(args.data_dir.join("input.txt")).unwrap();
    println!("{}", solve_part1(&input, &CubesLimits { red: 12, green: 13, blue: 14 }));
}