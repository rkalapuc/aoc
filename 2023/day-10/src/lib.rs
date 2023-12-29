use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::PathBuf;

use clap::Parser;
use ndarray::Array2;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Debug, PartialEq, Clone)]
enum Pipe {
    START,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
}

impl Pipe {
    fn create(item: char) -> Option<Pipe> {
        return match item {
            'S' => Some(Pipe::START),
            '|' => Some(Pipe::NS),
            '-' => Some(Pipe::EW),
            'L' => Some(Pipe::NE),
            'J' => Some(Pipe::NW),
            '7' => Some(Pipe::SW),
            'F' => Some(Pipe::SE),
            _ => None
        };
    }

    fn candidates() -> Vec<Pipe> {
        return vec![Pipe::NS, Pipe::EW, Pipe::NE, Pipe::NW, Pipe::SE, Pipe::SW];
    }

    fn connections(&self, pos: [usize; 2]) -> Vec<[i32; 2]> {
        return match self {
            Pipe::START => Vec::new(),
            Pipe::NS => vec![[pos[0] as i32 - 1, pos[1] as i32], [pos[0] as i32 + 1, pos[1] as i32]], // UP + DOWN
            Pipe::EW => vec![[pos[0] as i32, pos[1] as i32 - 1], [pos[0] as i32, pos[1] as i32 + 1]], // LEFT + RIGHT
            Pipe::NE => vec![[pos[0] as i32 - 1, pos[1] as i32], [pos[0] as i32, pos[1] as i32 + 1]], // UP + RIGHT
            Pipe::NW => vec![[pos[0] as i32 - 1, pos[1] as i32], [pos[0] as i32, pos[1] as i32 - 1]], // UP + LEFT
            Pipe::SE => vec![[pos[0] as i32 + 1, pos[1] as i32], [pos[0] as i32, pos[1] as i32 + 1]], // DOWN + RIGHT
            Pipe::SW => vec![[pos[0] as i32 + 1, pos[1] as i32], [pos[0] as i32, pos[1] as i32 - 1]], // DOWN + LEFT
        };
    }

    fn is_vertical(&self) -> bool {
        return match self {
            Pipe::NS => true,
            Pipe::SW => true,
            Pipe::SE => true,
            _ => false
        };
    }
}

#[derive(Debug)]
struct Grid {
    data: Array2<Option<Pipe>>,
}

impl Grid {
    fn create(input: &str) -> Grid {
        let data: Vec<Vec<Option<Pipe>>> = input.split('\n')
            .map(|line| line.chars().map(|it| Pipe::create(it)).collect())
            .collect();

        return Grid {
            data: Array2::from_shape_vec(
                (data.len(), data.first().unwrap().len()),
                data.into_iter().flatten().collect(),
            ).unwrap()
        };
    }

    fn cmp_pos(pos1: &[usize; 2], pos2: &[usize; 2]) -> Ordering {
        return [pos1[1], pos1[0]].cmp(&[pos2[1], pos2[0]]);
    }

    fn find(&self, item: Option<Pipe>) -> Option<[usize; 2]> {
        return self.data.indexed_iter().find_map(|it| {
            if *it.1 == item {
                return Some([it.0.0, it.0.1]);
            }
            return None;
        });
    }

    fn belongs(&self, pos: &[i32; 2]) -> bool {
        let shape = self.data.shape();
        return pos[0] >= 0 && pos[0] < shape[0] as i32 && pos[1] >= 0 && pos[1] < shape[1] as i32;
    }

    fn connections(&self, pos: [usize; 2]) -> Vec<[usize; 2]> {
        return match self.data[pos].as_ref() {
            Some(pipe) => pipe.connections(pos).iter().filter_map(|it| {
                if self.belongs(it) {
                    return Some([it[0] as usize, it[1] as usize]);
                }
                return None;
            }).collect(),
            None => Vec::new(),
        };
    }

    fn detect_start_pipe(&self, start_pos: &[usize; 2]) -> Pipe {
        let directions: Vec<[i32; 2]> = vec![
            // UP    RIGHT   DOWN    LEFT
            [-1, 0], [0, 1], [1, 0], [0, -1],
        ];

        let mut start_connections: Vec<[usize; 2]> = directions.iter().filter_map(|it| {
            if !self.belongs(&[start_pos[0] as i32 + it[0], start_pos[1] as i32 + it[1]]) {
                return None;
            }

            let pos: [usize; 2] = [(start_pos[0] as i32 + it[0]) as usize, (start_pos[1] as i32 + it[1]) as usize];

            return if self.connections(pos).contains(start_pos) { Some(pos) } else { None };
        }).collect();

        start_connections.sort_by(|a, b| Grid::cmp_pos(a, b));

        return Pipe::candidates().iter().find_map(|it| {
            let mut connections: Vec<[usize; 2]> = it.connections(*start_pos).iter()
                .filter_map(|it| if self.belongs(it) { Some([it[0] as usize, it[1] as usize]) } else { None })
                .collect();
            connections.sort_by(|a, b| Grid::cmp_pos(a, b));
            return if connections == start_connections { Some(it.clone()) } else { None };
        }).expect("Failed to detect start pipe");
    }

    fn find_loop(&self, start_pos: [usize; 2]) -> Vec<[usize; 2]> {
        let mut prev: [usize; 2] = start_pos.clone();
        let mut pos = self.connections(start_pos).first().unwrap().clone();
        let mut main_loop: Vec<[usize; 2]> = vec![start_pos];

        loop {
            if pos == start_pos {
                break;
            }
            main_loop.push(pos);

            let connections: Vec<[usize; 2]> = self.connections(pos);
            let next_pos: [usize; 2] = if connections[0] == prev { connections[1] } else { connections[0] };

            prev = pos;
            pos = next_pos;
        }

        return main_loop;
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut grid: Grid = Grid::create(input);

    let start_pos: [usize; 2] = grid.find(Some(Pipe::START)).expect("Animal position not found!");
    let start_pipe: Pipe = grid.detect_start_pipe(&start_pos);

    grid.data[start_pos] = Some(start_pipe);

    let result: usize = grid.find_loop(start_pos).len() / 2;

    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let mut grid: Grid = Grid::create(input);

    let start_pos: [usize; 2] = grid.find(Some(Pipe::START)).expect("Animal position not found!");
    let start_pipe: Pipe = grid.detect_start_pipe(&start_pos);

    grid.data[start_pos] = Some(start_pipe);

    let main_loop: HashSet<[usize; 2]> = HashSet::from_iter(grid.find_loop(start_pos));
    let shape: &[usize] = grid.data.shape();

    let mut enclosed_count: usize = 0;

    for idx in 0..shape[0] {
        let mut enclosed: bool = false;
        for idy in 0..shape[1] {
            let pos: [usize; 2] = [idx, idy];

            let pipe = grid.data[pos].as_ref();
            let is_vertical = pipe.map(|pipe| pipe.is_vertical()).or(Some(false)).unwrap();
            let in_loop: bool = main_loop.contains(&pos);

            if in_loop && is_vertical {
                enclosed = !enclosed;
            }

            if !in_loop && enclosed {
                enclosed_count += 1;
            }
        }
    }

    return enclosed_count.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1A: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const INPUT1B: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const INPUT2A: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT2B: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";


    const INPUT2C: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT1A);
        assert_eq!(result, "4");

        let result = solve_part1(INPUT1B);
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT2A);
        assert_eq!(result, "4");

        let result = solve_part2(INPUT2B);
        assert_eq!(result, "8");

        let result = solve_part2(INPUT2C);
        assert_eq!(result, "10");
    }
}