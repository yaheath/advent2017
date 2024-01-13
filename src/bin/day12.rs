use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct Input {
    program: usize,
    neighbors: Vec<usize>,
}
impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(" <-> ");
        let program = itr.next().unwrap().parse::<usize>().unwrap();
        let itr = itr.next().unwrap().split(", ");
        let neighbors: Vec<usize> = Vec::from_iter(
            itr.map(|ss| ss.parse::<usize>().unwrap())
        );
        Ok(Input{program, neighbors})
    }
}

fn get_neighbors(node: usize, map: &HashMap<usize, Vec<usize>>, traversed: &mut HashSet<usize>) {
    if traversed.contains(&node) { return; }
    traversed.insert(node);
    for n in map[&node].iter() {
        get_neighbors(*n, map, traversed);
    }
}

fn part1(input: &[Input]) -> usize {
    let map: HashMap<usize, Vec<usize>> = HashMap::from_iter(
        input.iter().map(|row| (row.program, row.neighbors.clone()))
    );
    let mut neighbors = HashSet::new();
    get_neighbors(0, &map, &mut neighbors);
    neighbors.len()
}

fn part2(input: &[Input]) -> usize {
    let map: HashMap<usize, Vec<usize>> = HashMap::from_iter(
        input.iter().map(|row| (row.program, row.neighbors.clone()))
    );
    let mut remaining: HashSet<usize> = HashSet::from_iter(map.keys().cloned());
    let mut ngroups = 0;
    while remaining.len() > 0 {
        let p = *remaining.iter().next().unwrap();
        let mut group = HashSet::new();
        get_neighbors(p, &map, &mut group);
        for n in group {
            remaining.remove(&n);
        }
        ngroups += 1;
    }
    ngroups
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
    fn day12_test() {
        let input: Vec<Input> = test_input(include_str!("day12.testinput"));
        assert_eq!(part1(&input), 6);
        assert_eq!(part2(&input), 2);
    }
}
