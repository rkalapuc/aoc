extern crate core;

use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

use clap::Parser;
use polygonical::point::Point;
use polygonical::polygon::Polygon;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

enum DigDirection {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Debug for DigDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DigDirection::UP => write!(f, "U"),
            DigDirection::RIGHT => write!(f, "R"),
            DigDirection::DOWN => write!(f, "D"),
            DigDirection::LEFT => write!(f, "L")
        }
    }
}

impl DigDirection {
    fn from_str(input: &str) -> DigDirection {
        return match input {
            "U" => DigDirection::UP,
            "R" => DigDirection::RIGHT,
            "D" => DigDirection::DOWN,
            "L" => DigDirection::LEFT,
            _ => panic!("Invalid direction: {}", input)
        };
    }

    fn from_num(input: &u8) -> DigDirection {
        return match input {
            0 => DigDirection::RIGHT,
            1 => DigDirection::DOWN,
            2 => DigDirection::LEFT,
            3 => DigDirection::UP,
            _ => panic!("Invalid direction: {}", input)
        };
    }

    fn dig(&self, length: u64) -> (i64, i64) {
        return match self {
            DigDirection::UP => (length as i64 * -1, 0),
            DigDirection::RIGHT => (0, length as i64),
            DigDirection::DOWN => (length as i64, 0),
            DigDirection::LEFT => (0, length as i64 * -1),
        };
    }
}

#[derive(Debug)]
struct DigInstruction {
    direction: DigDirection,
    length: u64,
}

impl DigInstruction {
    fn create(input: &str) -> DigInstruction {
        let parts: Vec<&str> = input.split(" ").take(2).collect();
        return DigInstruction {
            direction: DigDirection::from_str(parts.first().unwrap()),
            length: parts.last().unwrap().parse::<u64>().unwrap(),
        };
    }

    fn decode(input: &str) -> DigInstruction {
        let hex: &str = input.split(" ").last().unwrap().trim_matches(|it| it == '(' || it == ')' || it == '#');
        return DigInstruction {
            direction: DigDirection::from_num(&hex[5..].parse::<u8>().unwrap()),
            length: u64::from_str_radix(&hex[..5], 16).unwrap(),
        };
    }

    fn dig(&self, from: &Point) -> Point {
        let (offset_x, offset_y) = self.direction.dig(self.length);
        return Point { x: from.x + offset_x as f64, y: from.y + offset_y as f64 };
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut points: Vec<Point> = Vec::new();
    let (perimeter, _) = input.split('\n').fold((0u64, Point { x: 0f64, y: 0f64 }), |(perimeter, last_point), it| {
        points.push(last_point);
        let instruction: DigInstruction = DigInstruction::create(it);
        return (perimeter + instruction.length, instruction.dig(&last_point));
    });

    let polygon: Polygon = Polygon::new(points);
    let area: u64 = polygon.area() as u64;

    // pick theorem
    let points_inside: u64 = area - (perimeter / 2) + 1;

    let result: u64 = perimeter + points_inside;
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let mut points: Vec<Point> = Vec::new();
    let (perimeter, _) = input.split('\n').fold((0u64, Point { x: 0f64, y: 0f64 }), |(perimeter, last_point), it| {
        points.push(last_point);
        let instruction: DigInstruction = DigInstruction::decode(it);
        return (perimeter + instruction.length, instruction.dig(&last_point));
    });

    let polygon: Polygon = Polygon::new(points);
    let area: u64 = polygon.area() as u64;

    // pick theorem
    let points_inside: u64 = area - (perimeter / 2) + 1;

    let result: u64 = perimeter + points_inside;
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "62");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "952408144115");
    }
}