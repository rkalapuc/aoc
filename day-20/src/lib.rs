use std::cmp::min;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::Parser;

use common::math::lcm_vec;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

trait Module {
    fn current(&self) -> bool;
    fn handle(&mut self, from: &str, signal: bool) -> bool;
}

#[derive(Debug)]
struct Switch {
    state: bool,
}

impl Module for Switch {
    fn current(&self) -> bool {
        return self.state;
    }

    fn handle(&mut self, _: &str, signal: bool) -> bool {
        if signal {
            return false;
        }
        self.state = !self.state;
        return true;
    }
}

#[derive(Debug)]
struct Conjunction {
    state: HashMap<String, bool>,
}

impl Module for Conjunction {
    fn current(&self) -> bool {
        return !self.state.iter().fold(true, |acc, it| acc && *it.1);
    }

    fn handle(&mut self, from: &str, signal: bool) -> bool {
        self.state.insert(from.to_string(), signal);
        return true;
    }
}

#[derive(Debug)]
struct Broadcast {}

impl Module for Broadcast {
    fn current(&self) -> bool {
        return false;
    }

    fn handle(&mut self, _: &str, _: bool) -> bool {
        return false;
    }
}

struct DesertMachine {
    modules: HashMap<String, (Box<dyn Module>, Vec<String>)>,
}

impl FromStr for DesertMachine {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut kids: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut parents: HashMap<&str, Vec<&str>> = HashMap::new();

        input.split('\n').for_each(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let parent: &str = parts.first().unwrap().trim_matches(|it| it == '%' || it == '&');
            let next_kids: Vec<&str> = parts.last().unwrap().split(", ").collect();
            next_kids.iter().for_each(|kid| {
                if parents.contains_key(kid) {
                    parents.get_mut(kid).unwrap().push(parent);
                } else {
                    parents.insert(kid, vec![parent]);
                }
            });
            kids.insert(parent, next_kids);
        });

        let modules: HashMap<String, (Box<dyn Module>, Vec<String>)> = input.split('\n').map(|line| {
            let module_part = line.split(" -> ").take(1).next().unwrap();
            let module_name: &str = &module_part.trim_matches(|it| it == '%' || it == '&');
            let module: Box<dyn Module> = match &module_part[..1] {
                "%" => Box::new(Switch { state: false }),
                "&" => Box::new(Conjunction {
                    state: parents.get(module_name).unwrap().iter()
                        .map(|it| (it.to_string(), false))
                        .into_iter()
                        .collect()
                }),
                "b" => Box::new(Broadcast {}),
                _ => panic!("Invalid module")
            };

            return (
                module_name.to_string(),
                (
                    module,
                    kids.get(module_name).unwrap().iter()
                        .map(|it| it.to_string())
                        .collect()
                )
            );
        }).into_iter().collect();

        return Ok(DesertMachine { modules });
    }
}

impl DesertMachine {
    fn apply(&mut self) -> (u32, u32) {
        let mut total_low: u32 = 1;
        let mut total_high: u32 = 0;

        let mut active_modules: Vec<String> = vec!["broadcaster".to_string()];
        while !active_modules.is_empty() {
            let mut next_modules: Vec<String> = Vec::new();

            let (low, high) = active_modules.iter().fold((0, 0), |acc, module_name| {
                let (module, connections) = self.modules.get(module_name).unwrap();
                let signal: bool = module.current();
                return connections.clone().iter().fold(acc, |acc, next_name| {
                    if self.modules.contains_key(next_name) {
                        let (next_module, _) = self.modules.get_mut(next_name).unwrap();
                        if next_module.handle(module_name.as_str(), signal) {
                            next_modules.push(next_name.clone());
                        }
                    }
                    return if signal { (acc.0, acc.1 + 1) } else { (acc.0 + 1, acc.1) };
                });
            });

            total_low += low;
            total_high += high;

            active_modules = next_modules;
        }

        return (total_low, total_high);
    }

    fn find_parents(&self, module_name: &String) -> Vec<String> {
        return self.modules.iter().filter_map(|(module, (_, modules))| {
            return if modules.first().unwrap() == module_name { Some(module.clone()) } else { None };
        }).collect();
    }

    fn calc_min_steps(&mut self, final_modules: &Vec<String>) -> Vec<u64> {
        let mut idx: u64 = 0;

        let mut min_steps: HashMap<&String, u64> = final_modules.iter()
            .map(|module_name| (module_name, u64::MAX))
            .into_iter()
            .collect();

        loop {
            let mut active_modules: Vec<String> = vec!["broadcaster".to_string()];
            while !active_modules.is_empty() {
                let mut next_modules: Vec<String> = Vec::new();

                for module_name in active_modules {
                    let (module, connections) = self.modules.get(module_name.as_str()).unwrap();
                    let signal: bool = module.current();
                    for next_name in connections.clone() {
                        if self.modules.contains_key(next_name.as_str()) {
                            let (next_module, _) = self.modules.get_mut(&next_name).unwrap();
                            if next_module.handle(module_name.as_str(), signal) {
                                next_modules.push(next_name.clone());
                                if signal && min_steps.contains_key(&module_name) {
                                    if let Some(value) = min_steps.get_mut(&module_name) {
                                        *value = min(*value, idx + 1)
                                    }
                                    if min_steps.iter().all(|it| *it.1 != u64::MAX) {
                                        return min_steps.iter().map(|it| *it.1).collect();
                                    }
                                }
                            }
                        }
                    };
                };
                active_modules = next_modules;
            }
            idx += 1;
        }
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut machine: DesertMachine = input.parse::<DesertMachine>().unwrap();

    let (low, high) = (0..1000).fold((0, 0), |acc, _| {
        let (low, high) = machine.apply();
        return (acc.0 + low, acc.1 + high);
    });

    let result: u64 = low as u64 * high as u64;

    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let mut machine: DesertMachine = input.parse::<DesertMachine>().unwrap();

    let rx_parent: String = machine.find_parents(&"rx".to_string()).first()
        .expect("Cannot find parent module for 'rx'")
        .clone();

    let min_steps: Vec<u64> = machine.calc_min_steps(&machine.find_parents(&rx_parent));
    let result = lcm_vec(&min_steps).expect("Failed to calculate LCM");

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT1);
        assert_eq!(result, "32000000");

        let result = solve_part1(INPUT2);
        assert_eq!(result, "11687500");
    }
}