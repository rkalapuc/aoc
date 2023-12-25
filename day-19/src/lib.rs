use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Error};
use clap::Parser;
use itertools::Itertools;
use queues::{IsQueue, Queue};

use crate::Action::Forward;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser, default_value = "./")]
    pub data_dir: PathBuf,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Part<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T> Debug for Part<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={:?},m={:?},a={:?},s={:?}}}", self.x, self.m, self.a, self.s)
    }
}

impl FromStr for Part<u32> {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut part: Part<u32> = Part { x: 0, m: 0, a: 0, s: 0 };
        input.trim_matches(|it| it == '{' || it == '}').split(',').for_each(|category| {
            let parts: Vec<&str> = category.split('=').map(|it| it.trim()).collect();
            let value: u32 = parts.last().unwrap().parse::<u32>().unwrap();
            match parts.first().unwrap().parse::<Category>().unwrap() {
                Category::Extreme => part.x = value,
                Category::Musical => part.m = value,
                Category::Aerodynamic => part.a = value,
                Category::Shiny => part.s = value,
            }
        });
        return Ok(part);
    }
}

impl Part<u32> {
    fn score(&self) -> u32 {
        return self.x + self.m + self.a + self.s;
    }
}

impl Part<(u32, u32)> {
    fn score(&self) -> u64 {
        return [self.x, self.m, self.a, self.s].iter()
            .fold(1, |acc, it| acc * (it.1 as u64 - it.0 as u64 + 1));
    }
}

impl Part<(u32, u32)> {
    fn create(min: u32, max: u32) -> Part<(u32, u32)> {
        return Part {
            x: (min, max),
            m: (min, max),
            a: (min, max),
            s: (min, max),
        };
    }
}

enum Category {
    Extreme,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Extreme => write!(f, "e"),
            Category::Musical => write!(f, "m"),
            Category::Aerodynamic => write!(f, "a"),
            Category::Shiny => write!(f, "s"),
        }
    }
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return match input {
            "x" => Ok(Category::Extreme),
            "m" => Ok(Category::Musical),
            "a" => Ok(Category::Aerodynamic),
            "s" => Ok(Category::Shiny),
            _ => Err(anyhow!("Unexpected category: {}", input))
        };
    }
}

enum Condition {
    LESS(u32),
    GREATER(u32),
}

impl Debug for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::LESS(it) => write!(f, "<{}", it),
            Condition::GREATER(it) => write!(f, ">{}", it),
        }
    }
}

impl FromStr for Condition {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value: u32 = input[1..].parse::<u32>().unwrap();
        let op: char = input.chars().next().unwrap();
        return match op {
            '<' => Ok(Condition::LESS(value)),
            '>' => Ok(Condition::GREATER(value)),
            _ => Err(anyhow!("Unexpected operation: {}", op))
        };
    }
}

impl Condition {
    fn matches(&self, value: u32) -> bool {
        return match self {
            Condition::LESS(than) => value < *than,
            Condition::GREATER(than) => value > *than
        };
    }

    fn adjust(&self, adjusted: (u32, u32), next: (u32, u32)) -> ((u32, u32), (u32, u32)) {
        match self {
            Condition::LESS(than) => ((than + 0, adjusted.1), (next.0, than - 1)),
            Condition::GREATER(than) => ((adjusted.0, than + 0), (than + 1, next.1))
        }
    }
}

enum Action {
    Accept,
    Reject,
    Forward(String),
}

impl Debug for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Accept => write!(f, "A"),
            Action::Reject => write!(f, "R"),
            Forward(name) => write!(f, "{}", name)
        }
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return Ok(match input {
            "A" => Action::Accept,
            "R" => Action::Reject,
            next => Forward(next.to_string())
        });
    }
}

struct Workflow {
    conditions: Vec<(Category, Condition, Action)>,
    default_action: Action,
}

impl Debug for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let conditions = self.conditions.iter().map(|(category, condition, action)| {
            return format!("{:?}{:?}:{:?}", category, condition, action);
        }).join(",");
        write!(f, "{},{:?}", conditions, self.default_action)
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split(',').collect();
        return Ok(Workflow {
            conditions: parts.iter().take(parts.len() - 1).map(|it| {
                let (category, condition_action) = it.split_at(1);
                let parts: Vec<&str> = condition_action.split(':').collect();
                return (
                    category.parse::<Category>().unwrap(),
                    parts.first().unwrap().parse::<Condition>().unwrap(),
                    parts.last().unwrap().parse::<Action>().unwrap()
                );
            }).collect(),
            default_action: parts.last().unwrap().parse::<Action>().unwrap(),
        });
    }
}

impl Workflow {
    fn run(&self, part: &Part<u32>) -> &Action {
        return self.conditions.iter().find_map(|(category, condition, action)| {
            let matches: bool = match category {
                Category::Extreme => condition.matches(part.x),
                Category::Musical => condition.matches(part.m),
                Category::Aerodynamic => condition.matches(part.a),
                Category::Shiny => condition.matches(part.s),
            };
            return if matches { Some(action) } else { None };
        }).unwrap_or(&self.default_action);
    }

    fn adjust(&self, part: &Part<(u32, u32)>) -> (Vec<(&str, Part<(u32, u32)>)>, Vec<Part<(u32, u32)>>) {
        let mut accepted: Vec<Part<(u32, u32)>> = Vec::new();
        let mut queued: Vec<(&str, Part<(u32, u32)>)> = Vec::new();

        let mut adjusted: Part<(u32, u32)> = part.clone();
        for (category, condition, action) in &self.conditions {
            let mut next: Part<(u32, u32)> = adjusted.clone();
            match category {
                Category::Extreme => (adjusted.x, next.x) = condition.adjust(adjusted.x, next.x),
                Category::Musical => (adjusted.m, next.m) = condition.adjust(adjusted.m, next.m),
                Category::Aerodynamic => (adjusted.a, next.a) = condition.adjust(adjusted.a, next.a),
                Category::Shiny => (adjusted.s, next.s) = condition.adjust(adjusted.s, next.s),
            }
            match action {
                Action::Accept => accepted.push(next),
                Forward(next_workflow) => queued.push((next_workflow, next)),
                Action::Reject => {}
            }
        }

        match &self.default_action {
            Action::Accept => accepted.push(adjusted),
            Forward(next_workflow) => queued.push((next_workflow, adjusted)),
            Action::Reject => {}
        }

        return (queued, accepted);
    }
}

pub fn solve_part1(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let workflows: HashMap<&str, Workflow> = parts.first().unwrap().split('\n').fold(
        HashMap::new(),
        |mut acc, line| {
            let parts: Vec<&str> = line.split('{').collect();
            acc.insert(
                *parts.first().unwrap(),
                parts.last().unwrap().strip_suffix("}").unwrap().parse::<Workflow>().unwrap(),
            );
            return acc;
        },
    );

    let parts: Vec<Part<u32>> = parts.last().unwrap().split('\n')
        .map(|line| line.parse::<Part<u32>>().unwrap())
        .collect();

    let result = parts.iter().fold(0, |acc, part| {
        let mut next_action: &Action = workflows.get("in").unwrap().run(part);
        loop {
            match next_action {
                Action::Accept => {
                    return acc + part.score();
                }
                Action::Reject => break,
                Forward(next_workflow) => {
                    next_action = workflows.get(next_workflow.as_str()).unwrap().run(part);
                }
            }
        }
        return acc;
    });

    return result.to_string();
}

pub fn solve_part2(input: &str) -> String {
    let workflows: HashMap<&str, Workflow> = input.split('\n').fold(
        HashMap::new(),
        |mut acc, line| {
            let parts: Vec<&str> = line.split('{').collect();
            acc.insert(
                *parts.first().unwrap(),
                parts.last().unwrap().strip_suffix("}").unwrap().parse::<Workflow>().unwrap(),
            );
            return acc;
        },
    );

    let mut queue: Queue<(&str, Part<(u32, u32)>)> = Queue::new();
    queue.add(("in", Part::create(1, 4000))).unwrap();

    let mut accepted: HashSet<Part<(u32, u32)>> = HashSet::new();

    while queue.size() > 0 {
        let (workflow_name, part) = queue.remove().unwrap();
        let adjusted = workflows.get(workflow_name).unwrap().adjust(&part);
        adjusted.0.iter().for_each(|it| {
            queue.add(*it).unwrap();
        });
        accepted.extend(adjusted.1);
    }

    let result: u64 = accepted.iter().fold(0, |acc, it| acc + it.score());
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    const INPUT2: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}";

    #[test]
    fn test_part1() {
        let result = solve_part1(INPUT1);
        assert_eq!(result, "19114");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(INPUT2);
        assert_eq!(result, "167409079868000");
    }
}