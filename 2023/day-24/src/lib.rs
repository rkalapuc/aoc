use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;
use generator::{Generator, Gn};
use itertools::Itertools;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug)]
struct Hailstone {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

impl FromStr for Hailstone {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let pos_vel: Vec<i64> = input.replace(" @ ", ",")
            .split(",")
            .map(|it| it.trim().parse::<i64>().unwrap())
            .collect();
        return Ok(Hailstone {
            position: (pos_vel[0], pos_vel[1], pos_vel[2]),
            velocity: (pos_vel[3], pos_vel[4], pos_vel[5]),
        });
    }
}

impl Hailstone {
    fn bias(&self) -> (f64, f64) {
        let bias = self.velocity.1 as f64 / self.velocity.0 as f64;
        return (self.position.1 as f64 - (self.position.0 as f64 * bias), bias);
    }

    fn intersect(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let bias1: (f64, f64) = self.bias();
        let bias2: (f64, f64) = other.bias();

        let x: f64 = (bias2.0 - bias1.0) / (bias1.1 - bias2.1);
        let y: f64 = bias1.1 * x + bias1.0;

        return if self.is_before(x, y) || other.is_before(x, y) { None } else { Some((x, y)) };
    }

    fn is_before(&self, x: f64, y: f64) -> bool {
        return ((x - self.position.0 as f64) / self.velocity.0 as f64) < 0f64
            || ((y - self.position.1 as f64) / self.velocity.1 as f64) < 0f64;
    }
}

pub fn solve_part1(input: &str, from: f64, to: f64) -> String {
    let hailstones: Vec<Hailstone> = input.split("\n")
        .map(|it| it.parse::<Hailstone>().unwrap())
        .collect();

    let result = hailstones.iter().enumerate().fold(0, |acc, (idx, hs1)| {
        return acc + hailstones[idx + 1..].iter().fold(0, |acc, hs2| {
            if let Some(intersection) = hs1.intersect(hs2) {
                return if intersection.0 >= from && intersection.0 <= to && intersection.1 >= from && intersection.1 <= to {
                    acc + 1
                } else {
                    acc
                };
            }
            return acc;
        });
    });

    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let hailstones: Vec<Hailstone> = input.split("\n")
        .map(|it| it.parse::<Hailstone>().unwrap())
        .collect();

    let bruteforce: Generator<(), (i64, i64, i64)> = Gn::new_scoped(|mut scope| {
        let mut idx = 1;
        loop {
            for idy in 1..=idx {
                for idz in 1..=idx {
                    [-1, 1].iter().cartesian_product([-1, 1].iter().cartesian_product([-1, 1].iter())).for_each(|(x, (y, z))| {
                        scope.yield_((idx * x, idy * y, idz * z));
                    });
                }
            }
            idx += 1;
        }
    });

    let hs1: &Hailstone = &hailstones[0];
    let hs2: &Hailstone = &hailstones[1];

    let find_position = |velocity: (i64, i64, i64)| -> Option<(i64, i64, i64)> {
        let vxd: (i64, i64) = (hs1.velocity.0 - velocity.0, hs2.velocity.0 - velocity.0);
        let vyd: (i64, i64) = (hs1.velocity.1 - velocity.1, hs2.velocity.1 - velocity.1);

        let div = vxd.0 * vyd.1 - vyd.0 * vxd.1;

        if div != 0 {
            let time = (vyd.1 * (hs2.position.0 - hs1.position.0) - vxd.1 * (hs2.position.1 - hs1.position.1)) / div;
            return Some((
                hs1.position.0 + hs1.velocity.0 * time - velocity.0 * time,
                hs1.position.1 + hs1.velocity.1 * time - velocity.1 * time,
                hs1.position.2 + hs1.velocity.2 * time - velocity.2 * time
            ));
        }

        return None;
    };

    let check_velocity = |position: (i64, i64, i64), velocity: (i64, i64, i64)| -> bool {
        return hailstones.iter().all(|hs| {
            let time: i64;
            if hs.velocity.0 != velocity.0 {
                time = (position.0 - hs.position.0) / (hs.velocity.0 - velocity.0);
            } else if hs.velocity.1 != velocity.1 {
                time = (position.1 - hs.position.1) / (hs.velocity.1 - velocity.1);
            } else if hs.velocity.2 != velocity.2 {
                time = (position.2 - hs.position.2) / (hs.velocity.2 - velocity.2);
            } else {
                panic!("Cannot resolve time for position '{:?}' and velocity '{:?}'", position, velocity)
            }
            return position.0 + time * velocity.0 == hs.position.0 + time * hs.velocity.0
                && position.1 + time * velocity.1 == hs.position.1 + time * hs.velocity.1
                && position.2 + time * velocity.2 == hs.position.2 + time * hs.velocity.2;
        });
    };

    for candidate in bruteforce {
        if let Some(position) = find_position(candidate) {
            if check_velocity(position, candidate) {
                return (position.0 + position.1 + position.2).to_string();
            }
        }
    }

    panic!("Solution not found!")
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";


    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT, 7f64, 27f64);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "47");
    }
}