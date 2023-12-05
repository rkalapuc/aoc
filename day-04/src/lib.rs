use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug)]
struct Card {
    index: u32,
    winning: Vec<u32>,
    input: Vec<u32>,
}

fn parse_numbers(input: &str) -> Vec<u32> {
    return input.trim().split(" ").filter_map(
        |it| match it {
            "" => None,
            num => Some(num.trim().parse::<u32>().unwrap())
        }
    ).collect();
}

impl Card {
    fn create(card_input: &str) -> Card {
        let first_parts: Vec<&str> = card_input.split("|").collect();
        let second_parts: Vec<&str> = first_parts.first().unwrap().split(":").collect();
        let index = second_parts.first().unwrap().strip_prefix("Card ").unwrap();

        return Card {
            index: index.trim().parse().unwrap(),
            winning: parse_numbers(second_parts.last().unwrap()),
            input: parse_numbers(first_parts.last().unwrap()),
        };
    }

    fn matches_count(&self) -> u32 {
        return self.winning.iter().filter_map(
            |it| match self.input.contains(it) {
                true => Some(1),
                false => None
            }
        ).sum();
    }
}

pub fn solve_part1(input: &str) -> String {
    let cards: Vec<Card> = input.split("\n").map(|it| Card::create(it)).collect();

    let result: u32 = cards.iter().filter_map(|it| {
        match it.matches_count() {
            0 => None,
            1 => Some(1),
            n => Some(u32::pow(2, n - 1))
        }
    }).sum();

    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let cards: Vec<Card> = input.split("\n").map(|it| Card::create(it)).collect();
    let cards_count: usize = cards.len();

    let mut copies: HashMap<u32, u32> = HashMap::new();
    for card in cards {
        let copies_count: u32 = copies.get(&card.index).or(Some(&0)).unwrap().to_owned();
        let matches_count: u32 = card.matches_count();
        if matches_count > 0 {
            for idx in 1..matches_count + 1 {
                let copy_card_idx: u32 = card.index + idx;
                copies.insert(copy_card_idx, match copies.clone().get(&copy_card_idx) {
                    Some(count) => count + copies_count + 1,
                    None => copies_count + 1
                });
            }
        }
    }

    let result: u32 = copies.values().sum::<u32>() + (cards_count as u32);
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "30");
    }
}