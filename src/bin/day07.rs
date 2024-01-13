use std::collections::{HashSet, HashMap};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

#[derive(Clone, Debug)]
struct Input {
    name: String,
    weight: i64,
    branches: HashSet<String>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\w+) \((\d+)\)(?: -> (.*))?"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let name:String = caps.get(1).unwrap().as_str().into();
            let weight:i64 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let branches:HashSet<String> = if let Some(b) = caps.get(3) {
                b.as_str().split(", ").map(|s| s.to_string()).collect()
            } else {
                HashSet::new()
            };
            Ok(Input {name, weight, branches})
        }
        else {
            Err(())
        }
    }
}

fn part1(input: &[Input]) -> String {
    let mut parented: HashSet<String> = HashSet::new();
    let all_nodes: HashSet<String> = HashSet::from_iter(
        input.iter().map(|r| &r.name).cloned()
    );
    input.iter().for_each(|r| {
        r.branches.iter().for_each(|b| {
            parented.insert(b.clone());
        });
    });
    let mut roots: Vec<String> = all_nodes.difference(&parented).cloned().collect();
    assert!(roots.len() == 1);
    roots.pop().unwrap()
}

fn weight_of(node_name: &String, tree: &HashMap<String, &Input>) -> i64 {
    let w: i64 = tree[node_name].branches.iter().map(|b| weight_of(b, tree)).sum();
    tree[node_name].weight + w
}

fn rebalance(node_name: &String, tree: &HashMap<String, &Input>) -> Option<(String, i64)> {
    if tree[node_name].branches.len() < 2 { return None; }
    let weights: Vec<(&String, i64)> = tree[node_name].branches
        .iter()
        .map(|b| (b, weight_of(b, tree)))
        .collect();
    if weights.iter().map(|(_,w)| *w).all_equal() {
        return None;
    }
    assert!(weights.len() > 2);
    let common = weights.iter().map(|(_,w)| *w).duplicates().next().unwrap();
    let oddball: (&String, i64) = *weights.iter().find(|(_,w)| *w != common).unwrap();
    match rebalance(oddball.0, tree) {
        None => {
            let diff = common - oddball.1;
            Some((oddball.0.clone(), tree[oddball.0].weight + diff))
        },
        Some(ret) => Some(ret),
    }
}

fn part2(input: &[Input]) -> i64 {
    let mut parented: HashSet<String> = HashSet::new();
    let mut tree: HashMap<String, &Input> = HashMap::new();
    input.iter().for_each(|r| {
        r.branches.iter().for_each(|b| {
            parented.insert(b.clone());
        });
        tree.insert(r.name.clone(), r);
    });
    let root = input.iter().find(|r| !parented.contains(&r.name)).unwrap();
    if let Some((_, w)) = rebalance(&root.name, &tree) {
        w
    } else {
        panic!()
    }
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day07_test() {
        let input: Vec<Input> = test_input(include_str!("day07.testinput"));
        assert_eq!(part1(&input), "tknk".to_string());
        assert_eq!(part2(&input), 60);
    }
}
