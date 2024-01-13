use std::collections::HashMap;
use std::vec::Vec;
use std::iter::Iterator;
use std::str::FromStr;
use ya_advent_lib::read::read_input;

struct Input {
    layer: usize,
    range: usize,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(": ");
        let layer = iter.next().unwrap().parse::<usize>().unwrap();
        let range = iter.next().unwrap().parse::<usize>().unwrap();
        Ok(Input{layer, range})
    }
}

struct Scanner {
    range: usize,
}
impl Scanner {
    fn new(range: usize) -> Self {
        Self{range}
    }
    fn period(&self) -> usize {
        self.range * 2 - 2
    }
    fn loc_at(&self, time: usize) -> usize {
        if self.range == 2 { return time % 2; }
        let loc = time % self.period();
        if loc >= self.range {
            self.range - (loc - self.range + 2)
        } else {
            loc
        }
    }
}

fn part1(input: &[Input]) -> usize {
    let map: HashMap<usize, Scanner> = HashMap::from_iter(
        input.iter().map(|i| (i.layer, Scanner::new(i.range)))
    );
    let maxdepth = map.keys().cloned().max().unwrap();
    (0..=maxdepth).into_iter()
        .filter(|d| map.contains_key(d))
        .filter(|d| map[d].loc_at(*d) == 0)
        .map(|d| map[&d].range * d)
        .sum()
}

fn part2(input: &[Input]) -> usize {
    let map: HashMap<usize, Scanner> = HashMap::from_iter(
        input.iter().map(|i| (i.layer, Scanner::new(i.range)))
    );
    let maxdepth = map.keys().cloned().max().unwrap();
    for delay in 1.. {
        if (0..=maxdepth).into_iter()
                .filter(|d| map.contains_key(d))
                .all(|d| map[&d].loc_at(d + delay) != 0) {
            return delay;
        }
    }
    panic!()
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
    fn day13_test() {
        let s = Scanner::new(3);
        let vals = [0, 1, 2, 1, 0, 1, 2];
        vals.iter().enumerate().for_each(|(t, v)| assert_eq!(s.loc_at(t), *v));

        let s = Scanner::new(4);
        let vals = [0, 1, 2, 3, 2, 1, 0, 1];
        vals.iter().enumerate().for_each(|(t, v)| assert_eq!(s.loc_at(t), *v));

        let input: Vec<Input> = test_input("0: 3\n1: 2\n4: 4\n6: 4\n".into());
        assert_eq!(part1(&input), 24);
        assert_eq!(part2(&input), 10);
    }
}
