use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}


#[derive(Debug)]
struct Schema {
    cols: i32,
    rows: i32,
    data: Vec<Vec<u8>>,
}

impl Schema {
    const ADJACENT_POSITIONS: [[i32; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    fn create(input: &str) -> Schema {
        let data: Vec<Vec<u8>> = input.split("\n").map(
            |line| line.as_bytes().iter().map(|it| it.to_owned()).collect()
        ).collect();

        return Schema {
            cols: data.get(0).unwrap().len() as i32,
            rows: data.len() as i32,
            data,
        };
    }

    fn item(&self, idy: i32, idx: i32) -> &u8 {
        return self.data.get(idy as usize).unwrap().get(idx as usize).unwrap();
    }

    fn has_adjacent_symbol(&self, idy: i32, idx: i32) -> bool {
        for adj_pos in Schema::ADJACENT_POSITIONS {
            if self.is_symbol(idy + adj_pos[0], idx + adj_pos[1]) {
                return true;
            }
        }
        return false;
    }

    fn find_adjacent_gears(&self, idy: i32, idx: i32) -> HashSet<u32> {
        let mut gears_positions: HashSet<u32> = HashSet::new();
        for adj_pos in Schema::ADJACENT_POSITIONS {
            if self.is_gear(idy + adj_pos[0], idx + adj_pos[1]) {
                gears_positions.insert(((idy + adj_pos[0]) * (self.cols - 1) + idx + adj_pos[1]) as u32);
            }
        }
        return gears_positions;
    }

    fn is_valid_position(&self, idy: i32, idx: i32) -> bool {
        idy >= 0 && idy < self.rows && idx >= 0 && idx < self.cols
    }

    fn is_symbol(&self, idy: i32, idx: i32) -> bool {
        if !self.is_valid_position(idy, idx) {
            return false;
        }

        let char: &u8 = self.item(idy, idx);
        return !Schema::is_digit(char) && char != &b'.';
    }

    fn is_gear(&self, idy: i32, idx: i32) -> bool {
        if !self.is_valid_position(idy, idx) {
            return false;
        }
        return self.item(idy, idx) == &b'*';
    }

    fn is_digit(char: &u8) -> bool {
        char.is_ascii_digit()
    }
}

#[derive(Clone)]
struct SchemaItem {
    item: u8,
    idy: i32,
    idx: i32,
    last_in_row: bool,
    is_digit: bool,
}

impl Display for SchemaItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "item '{}' at '{}:{}' |  last_in_row: {}; is_digit: {}",
            self.item as char, self.idy, self.idx, self.last_in_row, self.is_digit
        );
    }
}

struct SchemaIterator<'a> {
    schema: &'a Schema,
    curr: Option<SchemaItem>,
}

impl<'a> SchemaIterator<'a> {
    fn for_schema(schema: &Schema) -> SchemaIterator {
        return SchemaIterator {
            schema,
            curr: None,
        };
    }
}

impl<'a> Iterator for SchemaIterator<'a> {
    type Item = SchemaItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() {
            let item = self.schema.item(0, 0);
            self.curr = Some(SchemaItem {
                item: item.to_owned(),
                idy: 0,
                idx: 0,
                last_in_row: !self.schema.is_valid_position(0, 1),
                is_digit: Schema::is_digit(item),
            });
            return self.curr.clone();
        }

        let curr: SchemaItem = self.curr.clone().unwrap();
        let last_in_row = !self.schema.is_valid_position(curr.idy, curr.idx + 1);

        let next_idx = if last_in_row { 0 } else { curr.idx + 1 };
        let next_idy = if last_in_row { curr.idy + 1 } else { curr.idy };

        if self.schema.is_valid_position(next_idy, next_idx) {
            let item = self.schema.item(next_idy, next_idx);
            self.curr = Some(SchemaItem {
                item: item.to_owned(),
                idy: next_idy,
                idx: next_idx,
                last_in_row: !self.schema.is_valid_position(next_idy, next_idx + 1),
                is_digit: Schema::is_digit(item),
            });
            return self.curr.clone();
        }

        return None;
    }
}

pub fn solve_part1(input: &str) -> String {
    let schema: Schema = Schema::create(input);

    let mut next_digit: Vec<u8> = Vec::new();
    let mut has_adj_symbol: bool = false;

    let result: u32 = SchemaIterator::for_schema(&schema).into_iter().map(
        |it| {
            if it.is_digit {
                next_digit.push(it.item);
                has_adj_symbol = has_adj_symbol || schema.has_adjacent_symbol(it.idy, it.idx);
            }

            if (it.last_in_row || !it.is_digit) && !next_digit.is_empty() {
                let result = if has_adj_symbol { String::from_utf8(next_digit.clone()).unwrap().parse().unwrap() } else { 0 };
                next_digit.clear();
                has_adj_symbol = false;
                return result;
            }

            return 0;
        }
    ).sum();

    return result.to_string();
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct IntTuple(i32, i32);

pub fn solve_part2(input: &str) -> String {
    let schema: Schema = Schema::create(input);

    let mut next_digit: Vec<u8> = Vec::new();
    let mut adj_gears: HashSet<u32> = HashSet::new();
    let mut gears: HashMap<i32, Vec<i32>> = HashMap::new();

    SchemaIterator::for_schema(&schema).into_iter().for_each(
        |it| {
            if it.is_digit {
                next_digit.push(it.item);
                adj_gears.extend(schema.find_adjacent_gears(it.idy, it.idx));
            }

            if (it.last_in_row || !it.is_digit) && !next_digit.is_empty() {
                let digit = String::from_utf8(next_digit.clone()).unwrap().parse().unwrap();
                for gear in adj_gears.clone() {
                    if !gears.contains_key(&(gear as i32)) {
                        gears.insert(gear as i32, Vec::new());
                    }
                    gears.get_mut(&(gear as i32)).unwrap().push(digit);
                }

                next_digit.clear();
                adj_gears.clear();
            }
        }
    );

    let result: i32 = gears.values().filter_map(
        |it| {
            if it.len() == 2 {
                return Some(it.get(0).unwrap() * it.get(1).unwrap());
            }
            return None;
        }
    ).sum();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";


    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "467835");
    }
}