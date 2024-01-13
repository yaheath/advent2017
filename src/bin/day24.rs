use std::collections::HashSet;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Component {
    a: u64,
    b: u64,
}

impl FromStr for Component {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splt = s.split('/');
        let a = splt.next().unwrap().parse::<u64>().unwrap();
        let b = splt.next().unwrap().parse::<u64>().unwrap();
        Ok(Component{a, b})
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Bridge(Vec<Component>);

impl Bridge {
    fn new(c: &Component) -> Self {
        assert!(c.a == 0 || c.b == 0);
        Bridge(vec![c.clone()])
    }
    fn maybe_append(&self, c: &Component) -> Option<Self> {
        if self.0.contains(c) {
            // this component already part of the bridge
            return None;
        }
        let idx = self.0.len() - 1;
        let free =
            if idx > 0 {
                if self.0[idx - 1].a == self.0[idx].a || self.0[idx - 1].b == self.0[idx].a {
                    self.0[idx].b
                }
                else {
                    self.0[idx].a
                }
            }
            else {
                if self.0[0].a == 0 {
                    self.0[0].b
                }
                else {
                    self.0[0].a
                }
            };
        if c.a == free || c.b == free {
            Some(self.append(c))
        }
        else {
            None
        }
    }
    fn append(&self, c: &Component) -> Self {
        let mut v = self.0.clone();
        v.push(c.clone());
        Self(v)
    }
    fn strength(&self) -> u64 {
        self.0.iter().map(|c| c.a + c.b).sum()
    }
}

fn build_all(input: &[Component]) -> HashSet<Bridge> {
    let mut set: HashSet<Bridge> = HashSet::new();
    let mut queue: Vec<Bridge> = Vec::from_iter(
        input.iter()
        .filter(|c| c.a == 0 || c.b == 0)
        .map(|c| Bridge::new(c))
    );
    queue.iter().for_each(|b| {set.insert(b.clone());});
    while let Some(b) = queue.pop() {
        let mut new:Vec<Bridge> = input.iter()
            .flat_map(|c| b.maybe_append(c))
            .filter(|nb| !set.contains(nb))
            .collect();
        for nb in new.iter() {
            set.insert(nb.clone());
        }
        queue.append(&mut new);
    }
    set
}

fn part1(set: &HashSet<Bridge>) -> u64 {
    set.iter().map(|b| b.strength()).max().unwrap()
}

fn part2(set: &HashSet<Bridge>) -> u64 {
    let maxlen = set.iter().map(|b| b.0.len()).max().unwrap();
    set.iter().filter(|b| b.0.len() == maxlen).map(|b| b.strength()).max().unwrap()
}

fn main() {
    let input: Vec<Component> = read_input();
    let set = build_all(&input);
    println!("Part 1: {}", part1(&set));
    println!("Part 2: {}", part2(&set));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day24_test() {
        let input: Vec<Component> = test_input(include_str!("day24.testinput"));
        let set = build_all(&input);
        assert_eq!(part1(&set), 31);
        assert_eq!(part2(&set), 19);
    }
}
