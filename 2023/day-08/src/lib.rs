use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;

use common::math::lcm_vec;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

enum LR {
    L,
    R,
}

struct Map {
    instructions: Vec<LR>,
    network: HashMap<String, (String, String)>,
}

impl Map {
    fn create(input: &str) -> Map {
        let lines: Vec<&str> = input.splitn(2, "\n").collect();

        return Map {
            instructions: lines.first().unwrap().chars().map(|it| if it == 'L' { LR::L } else { LR::R }).collect(),
            network: lines.last().unwrap().trim().split('\n').map(|it| {
                let record: Vec<&str> = it.split(" = ").collect();
                let coords: Vec<&str> = record.last().unwrap().split(",").collect();
                (
                    record.first().unwrap().trim().to_owned(),
                    (
                        coords.first().unwrap().strip_prefix("(").unwrap().trim().to_owned(),
                        coords.last().unwrap().strip_suffix(")").unwrap().trim().to_owned()
                    )
                )
            }).collect(),
        };
    }

    fn next_instruction(&self, steps_count: &u64) -> &LR {
        return self.instructions.get((steps_count % self.instructions.len() as u64) as usize).unwrap();
    }
}


pub fn solve_part1(input: &str) -> String {
    let map: Map = Map::create(input);

    let mut node: &str = "AAA";
    let mut steps_count: u64 = 0;

    while node != "ZZZ" {
        match map.next_instruction(&steps_count) {
            LR::L => node = map.network.get(node).unwrap().0.as_str(),
            LR::R => node = map.network.get(node).unwrap().1.as_str(),
        }
        steps_count += 1;
    }

    return steps_count.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let map: Map = Map::create(input);

    let nodes: Vec<&str> = map.network.keys().filter_map(|it| if it.ends_with("A") { Some(it.as_str()) } else { None }).collect();
    let steps_count: Vec<u64> = nodes.iter().map(|it| {
        let mut node: &str = it;
        let mut steps_count: u64 = 0;

        while !node.ends_with("Z") {
            match map.next_instruction(&steps_count) {
                LR::L => node = map.network.get(node).unwrap().0.as_str(),
                LR::R => node = map.network.get(node).unwrap().1.as_str(),
            }
            steps_count += 1;
        }
        return steps_count;
    }).collect();

    let result: u64 = lcm_vec(&steps_count).unwrap();
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT1);
        assert_eq!(result, "2");

        let result = solve_part1(INPUT2);
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT3);
        assert_eq!(result, "6");
    }
}