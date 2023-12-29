use std::path::PathBuf;

use clap::Parser;
use linked_hash_map::LinkedHashMap;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

fn make_hash(input: &str) -> u32 {
    return input.bytes().fold(0, |acc, code| ((acc + code as u32) * 17) % 256);
}

fn parse_step(input: &str) -> (&str, &str) {
    let parts: Vec<&str> = input.split(['-', '=']).collect();
    return (parts.first().unwrap(), parts.last().unwrap());
}

pub fn solve_part1(input: &str) -> String {
    let result: u32 = input.split(',').fold(0, |acc, it| acc + make_hash(it));
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let mut boxes: Vec<LinkedHashMap<&str, u32>> = (0..256).into_iter().map(|_| LinkedHashMap::new()).collect();

    input.split(',').for_each(|step| {
        let (label, focal) = parse_step(step);
        let hash: u32 = make_hash(label);
        if !focal.is_empty() {
            let storage = boxes.get_mut(hash as usize).unwrap();
            if storage.contains_key(label) {
                storage[label] = focal.parse::<u32>().unwrap();
            } else {
                storage.insert(label, focal.parse::<u32>().unwrap());
            }
        } else if boxes[hash as usize].contains_key(label) {
            boxes.get_mut(hash as usize).unwrap().remove(label);
        }
    });

    let result: usize = boxes.iter().enumerate().fold(0, |acc, (idx, storage)| {
        return acc + storage.iter().enumerate().fold(0, |acc, (idy, label)| {
            return acc + (idx + 1) * (idy + 1) * (*label.1 as usize);
        });
    });

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "1320");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "145");
    }
}