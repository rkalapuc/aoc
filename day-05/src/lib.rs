use std::cmp;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug)]
struct Mapping {
    dest_start: u64,
    src_start: u64,
    len: u64,
}

#[derive(Debug, Clone)]
struct Range(u64, u64);

impl Mapping {
    fn create(input: &str) -> Mapping {
        let range: Vec<u64> = input.trim().split(" ").map(|it| it.trim().parse::<u64>().unwrap()).collect();
        return Mapping {
            dest_start: range.first().unwrap().to_owned(),
            src_start: range.get(1).unwrap().to_owned(),
            len: range.last().unwrap().to_owned(),
        };
    }

    fn includes(&self, seed: &u64) -> bool {
        return self.src_start <= *seed && *seed < self.src_start + self.len;
    }
}

#[derive(Debug)]
struct Almanac<T> {
    seeds: Vec<T>,
    mapping: Vec<Vec<Mapping>>,
}

impl<T> Almanac<T> {
    fn default() -> Almanac<T> {
        return Almanac { seeds: Vec::new(), mapping: Vec::new() };
    }
}

impl Almanac<u64> {
    fn create(input: &str) -> Almanac<u64> {
        return input.split("\n").fold((Almanac::default() as Almanac<u64>, None::<usize>), |mut acc, line| {
            if line.starts_with("seeds:") {
                acc.0.seeds = line.strip_prefix("seeds:").unwrap().trim().split(" ").map(|it| it.trim().parse::<u64>().unwrap()).collect();
            } else if line.len() != 0 {
                if line.chars().next().unwrap().is_alphabetic() {
                    acc.1 = match acc.1 {
                        Some(n) => Some(n + 1),
                        _ => Some(0)
                    };
                    acc.0.mapping.push(Vec::new());
                } else if line.chars().next().unwrap().is_digit(10) {
                    acc.0.mapping.get_mut(acc.1.unwrap()).unwrap().push(Mapping::create(line));
                }
            }
            return acc;
        }).0;
    }

    fn map(&self) -> Vec<u64> {
        return self.mapping.iter().fold(self.seeds.clone(), |input, mapping| {
            return input.iter().map(|seed| {
                return match mapping.iter().find(|it| it.includes(seed)) {
                    Some(m) => seed + m.dest_start - m.src_start,
                    _ => seed.to_owned()
                };
            }).collect();
        });
    }
}

impl Almanac<Range> {
    fn create(input: &str) -> Almanac<Range> {
        return input.split("\n").fold((Almanac::default() as Almanac<Range>, None::<usize>), |mut acc, line| {
            if line.starts_with("seeds:") {
                let seeds: Vec<u64> = line.strip_prefix("seeds:").unwrap().trim().split(" ").map(|it| it.trim().parse::<u64>().unwrap()).collect();
                acc.0.seeds = seeds.chunks(2).map(|it| Range { 0: it[0], 1: it[1] }).collect();
            } else if line.len() != 0 {
                if line.chars().next().unwrap().is_alphabetic() {
                    acc.1 = match acc.1 {
                        Some(n) => Some(n + 1),
                        _ => Some(0)
                    };
                    acc.0.mapping.push(Vec::new());
                } else if line.chars().next().unwrap().is_digit(10) {
                    acc.0.mapping.get_mut(acc.1.unwrap()).unwrap().push(Mapping::create(line));
                }
            }
            return acc;
        }).0;
    }

    fn map(&self) -> Vec<Range> {
        return self.mapping.iter().fold(self.seeds.clone(), |input, mapping| {
            let mut output: Vec<Range> = Vec::new();

            input.iter().for_each(|it| {
                let mut next_range: Option<Range> = Some(it.to_owned());
                while next_range.is_some() {
                    let range = next_range.unwrap();
                    match mapping.iter().find(|it| it.includes(&range.0)) {
                        Some(m) => {
                            let covered: u64 = cmp::min(m.src_start + m.len - range.0, range.1);
                            let left: u64 = range.1 - covered;
                            output.push(Range { 0: range.0 + m.dest_start - m.src_start, 1: covered });
                            next_range = if left > 0 { Some(Range { 0: range.0 + covered, 1: left }) } else { None };
                        }
                        _ => {
                            output.push(range.clone());
                            next_range = None
                        }
                    }
                }
            });

            return output;
        });
    }
}

pub fn solve_part1(input: &str) -> String {
    let almanac: Almanac<u64> = <Almanac<u64>>::create(input);
    let output_seeds: Vec<u64> = almanac.map();
    let result = output_seeds.iter().min().unwrap();
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let almanac: Almanac<Range> = <Almanac<Range>>::create(input);
    let output_seeds: Vec<Range> = almanac.map();
    let result = output_seeds.iter().map(|it| it.0).min().unwrap();
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "35");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "46");
    }
}