use std::fs;
use day_02::solve_part1;
use day_02::CubesLimits;

fn main() {
    let file = fs::read_to_string("./input1.txt").unwrap();
    println!("{}", solve_part1(&file, &CubesLimits { red: 12, green: 13, blue: 14 }));
}