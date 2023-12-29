use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::u64;

use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

struct Hand {
    bid: u32,
    rank: u8,
    base13: u64,
}

lazy_static! {
    static ref RANKS: HashMap<&'static str, u8> = vec![
        ("11111", 0),
        ("1111J", 1),
        ("2111",1),
        ("221", 2),
        ("211J", 3),
        ("111JJ", 3),
        ("311", 3),
        ("22J", 4),
        ("32", 4),
        ("11JJJ", 5),
        ("21JJ", 5),
        ("31J", 5),
        ("41", 5),
        ("JJJJJ", 6),
        ("1JJJJ", 6),
        ("2JJJ", 6),
        ("3JJ", 6),
        ("4J", 6),
        ("5", 6),
    ].into_iter().collect();

    static ref PUZZLE1_BASE13: HashMap<u8, u8> = "23456789TJQKA".bytes()
        .enumerate()
        .map(|(idx, ch)| (ch, idx as u8))
        .collect();

    static ref PUZZLE2_BASE13: HashMap<u8, u8> = "J23456789TQKA".bytes()
        .enumerate()
        .map(|(idx, ch)| (ch, idx as u8))
        .collect();
}

impl Hand {
    fn create(input: &str) -> Hand {
        let parts: Vec<&str> = input.split(' ').collect();

        let mut cards: Vec<u8> = parts.first().unwrap().as_bytes().iter().map(|it| *it).collect();
        let base13: u64 = Hand::decode(cards.clone(), &PUZZLE1_BASE13);

        cards.sort();

        let mut sizes: Vec<usize> = cards.iter().group_by(|b| b.to_owned()).into_iter()
            .map(|it| it.1.count())
            .collect();
        sizes.sort_by(|a, b| b.cmp(a));

        let rank_key: String = sizes.iter().map(|it| it.to_string()).intersperse("".to_owned()).collect();

        return Hand {
            bid: parts.last().unwrap().parse().unwrap(),
            rank: RANKS.get(rank_key.as_str()).unwrap().to_owned(),
            base13,
        };
    }

    fn create_with_jokers(input: &str) -> Hand {
        let parts: Vec<&str> = input.split(' ').collect();

        let cards: Vec<u8> = parts.first().unwrap().as_bytes().iter().map(|it| *it).collect();
        let base13: u64 = Hand::decode(cards.clone(), &PUZZLE2_BASE13);

        let mut no_joker_cards: Vec<u8> = cards.iter().filter(|it| *it != &b'J').map(|it| it.to_owned()).collect();
        let jokers_count: u8 = (5 - no_joker_cards.len()) as u8;

        no_joker_cards.sort();

        let mut sizes: Vec<usize> = no_joker_cards.iter().group_by(|b| b.to_owned()).into_iter()
            .map(|it| it.1.count())
            .collect();
        sizes.sort_by(|a, b| b.cmp(a));

        let rank_key: String = sizes.iter()
            .map(|it| it.to_string())
            .merge(std::iter::repeat('J'.to_string()).take(jokers_count as usize))
            .intersperse("".to_owned())
            .collect();

        return Hand {
            bid: parts.last().unwrap().parse().unwrap(),
            rank: RANKS.get(rank_key.as_str()).unwrap().to_owned(),
            base13,
        };
    }

    fn decode(cards: Vec<u8>, base: &HashMap<u8, u8>) -> u64 {
        let base_len: usize = base.len();

        let mut reversed = cards.clone();
        reversed.reverse();

        return reversed.iter()
            .enumerate()
            .fold(0, |acc, (idx, ch)| {
                return acc + base.get(ch).unwrap().to_owned() as usize * base_len.pow(idx as u32);
            }) as u64;
    }

    fn compare(&self, other: &Hand) -> Ordering {
        if self.rank > other.rank {
            return Ordering::Greater;
        }

        if self.rank < other.rank {
            return Ordering::Less;
        }

        if self.base13 > other.base13 {
            return Ordering::Greater;
        }

        if self.base13 < other.base13 {
            return Ordering::Less;
        }

        return Ordering::Equal;
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.rank == other.rank && self.base13 == other.base13;
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.compare(other));
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.compare(other);
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut hands: Vec<Hand> = input.split("\n").map(|line| Hand::create(line)).collect();
    hands.sort();

    let result: u64 = hands.iter().enumerate().fold(0, |acc, (idx, hand)| acc + (idx + 1) * hand.bid as usize ) as u64;
    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let mut hands: Vec<Hand> = input.split("\n").map(|line| Hand::create_with_jokers(line)).collect();
    hands.sort();

    let result: u64 = hands.iter().enumerate().fold(0, |acc, (idx, hand)| acc + (idx + 1) * hand.bid as usize ) as u64;
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "6440");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "5905");
    }
}