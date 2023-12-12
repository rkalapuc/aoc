use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone, Copy)]
enum LineItem {
    Separator,
    Spring,
    Unknown,
}

#[derive(Debug)]
struct DamagedLine {
    line: Vec<LineItem>,
    sizes: Vec<u32>,
    cache: HashMap<(usize, i32, usize), u64>,
}

impl DamagedLine {
    fn create_single(input: &str) -> DamagedLine {
        let parts: Vec<&str> = input.split(' ').collect();
        return DamagedLine {
            line: parts.first().unwrap().chars().map(|it| match it {
                '?' => LineItem::Unknown,
                '.' => LineItem::Separator,
                '#' => LineItem::Spring,
                _ => panic!("Unexpected input")
            }).collect(),
            sizes: parts.last().unwrap().split(',').map(|it| it.parse::<u32>().unwrap()).collect(),
            cache: HashMap::new(),
        };
    }

    fn create_unfolded(input: &str, factor: usize) -> DamagedLine {
        let mut single: DamagedLine = DamagedLine::create_single(input);
        single.line.push(LineItem::Unknown);

        return DamagedLine {
            line: single.line.iter()
                .cycle()
                .take(single.line.len() * factor - 1)
                .map(|it| *it)
                .collect(),
            sizes: single.sizes.iter()
                .cycle()
                .take(single.sizes.len() * factor)
                .map(|it| *it)
                .collect(),
            cache: HashMap::new(),
        };
    }

    fn handle_separator(&mut self, line_pos: usize, size_pos: (i32, usize)) -> u64 {
        return match size_pos.0 <= 0 {
            true => self.calc_arrangements(line_pos + 1, (-1, size_pos.1)),
            false => 0
        };
    }

    fn handle_spring(&mut self, line_pos: usize, size_pos: (i32, usize)) -> u64 {
        return match size_pos.0 > 0 {
            true => self.calc_arrangements(line_pos + 1, (size_pos.0 - 1, size_pos.1)),
            false => match size_pos.0 < 0 && size_pos.1 < self.sizes.len() {
                true => self.calc_arrangements(line_pos + 1, (self.sizes[size_pos.1] as i32 - 1, size_pos.1 + 1)),
                false => 0
            }
        };
    }

    fn calc_arrangements(&mut self, line_pos: usize, size_pos: (i32, usize)) -> u64 {
        let result_cache_key = (line_pos, size_pos.0, size_pos.1);

        // check if result already calculated
        if self.cache.contains_key(&result_cache_key) {
            return *self.cache.get(&result_cache_key).unwrap();
        }

        // reach end of line ?
        if line_pos == self.line.len() {
            return if size_pos.0 > 0 || size_pos.1 != self.sizes.len() { 0 } else { 1 };
        }

        let mut result: u64 = 0;

        match self.line[line_pos] {
            LineItem::Unknown => {
                result += self.handle_separator(line_pos, size_pos);
                result += self.handle_spring(line_pos, size_pos);
            }

            LineItem::Separator => {
                result += self.handle_separator(line_pos, size_pos);
            }

            LineItem::Spring => {
                result += self.handle_spring(line_pos, size_pos);
            }
        }

        self.cache.insert(result_cache_key, result);

        return result;
    }
}

pub fn solve_part1(input: &str) -> String {
    let result: i64 = input.split('\n')
        .map(|line| DamagedLine::create_single(line))
        .map(|mut it| it.calc_arrangements(0, (-1, 0)))
        .fold(0, |acc, it| acc + it as i64);
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let result: i64 = input.split('\n')
        .map(|line| DamagedLine::create_unfolded(line, 5))
        .map(|mut it| it.calc_arrangements(0, (-1, 0)))
        .fold(0, |acc, it| acc + it as i64);
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "525152");
    }
}