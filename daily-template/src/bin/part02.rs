use std::fs;
use clap::Parser;
use {{crate_name}}::{Cli, solve_part2};

fn main() {
    let args: Cli = Cli::parse();
    let input: String = fs::read_to_string(args.data_dir.join("input.txt")).unwrap();
    println!("{}", solve_part2(&input));
}