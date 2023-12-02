use std::fs;
use day_02::solve_part2;

fn main() {
    let file = fs::read_to_string("./input2.txt").unwrap();
    println!("{}", solve_part2(&file));
}