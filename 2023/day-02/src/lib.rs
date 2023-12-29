use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

pub struct CubesLimits {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug)]
struct GameRound {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubesLimits {
    fn default() -> CubesLimits {
        CubesLimits { red: 0, green: 0, blue: 0 }
    }

    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

impl GameRound {
    fn default() -> GameRound {
        GameRound { red: 0, green: 0, blue: 0 }
    }

    fn new(round_input: &str) -> GameRound {
        let mut round: GameRound = GameRound::default();
        let parts: Vec<&str> = round_input.split(",").collect();
        for part in parts {
            if part.ends_with(" red") {
                round.red = round.red + part.strip_suffix(" red").unwrap().trim().parse::<u32>().unwrap();
            } else if part.ends_with(" green") {
                round.green = round.green + part.strip_suffix(" green").unwrap().trim().parse::<u32>().unwrap();
            } else if part.ends_with(" blue") {
                round.blue = round.blue + part.strip_suffix(" blue").unwrap().trim().parse::<u32>().unwrap();
            }
        }
        return round;
    }

    fn is_valid(&self, limits: &CubesLimits) -> bool {
        return self.red <= limits.red && self.green <= limits.green && self.blue <= limits.blue;
    }
}

#[derive(Debug)]
struct Game {
    index: u32,
    rounds: Vec<GameRound>,
}

impl Game {
    fn new(game_input: &str) -> Game {
        let parts: Vec<&str> = game_input.split(":").collect();

        let index = parts.first().unwrap().strip_prefix("Game ").unwrap();
        let rounds: Vec<GameRound> = parts.last().unwrap().split(";").map(|it| GameRound::new(it)).collect();
        return Game { index: index.parse().unwrap(), rounds };
    }

    fn is_possible(&self, limits: &CubesLimits) -> bool {
        return !self.rounds.iter().any(|it| !it.is_valid(limits));
    }

    fn get_min_limits(&self) -> CubesLimits {
        let mut limit = CubesLimits::default();

        self.rounds.iter().for_each(|it| {
            if limit.red < it.red {
                limit.red = it.red;
            }

            if limit.green < it.green {
                limit.green = it.green;
            }

            if limit.blue < it.blue {
                limit.blue = it.blue;
            }
        });

        return limit;
    }
}

pub fn solve_part1(input: &str, limits: &CubesLimits) -> String {
    let result: u32 = input.split("\n")
        .map(|it| Game::new(it))
        .map(|it| if it.is_possible(limits) { it.index } else { 0 })
        .sum();

    return result.to_string();
}


pub fn solve_part2(input: &str) -> String {
    let result: u32 = input.split("\n")
        .map(|it| Game::new(it))
        .map(|it| it.get_min_limits())
        .map(|it| it.power())
        .sum();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT, &CubesLimits { red: 12, green: 13, blue: 14 });
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "2286");
    }
}