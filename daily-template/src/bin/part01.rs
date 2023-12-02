use std::fs;
use clap::Parser;
use {{crate_name}}::solve_part1;

fn main() {
    let args: Cli = Cli::parse();
    let input: String = fs::read_to_string(args.data_dir.join("input1.txt")).unwrap();
    println!("{}", solve_part1(&input));
}