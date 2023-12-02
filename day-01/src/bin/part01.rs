use day_01::solve_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input1.txt").unwrap();
    println!("{}", solve_part1(&file));
}