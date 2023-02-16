use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Clone)]
struct Bank(Vec<i64>);

impl FromStr for Bank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bank(s.split('\t').map(|s| s.parse::<i64>().unwrap()).collect()))
    }
}

fn realloc(bank: &mut Bank) {
    let mut idx = bank.0.iter()
        .enumerate()
        .rev()
        .max_by_key(|(_,v)| *v)
        .map(|(idx,_)| idx)
        .unwrap();
    let mut blocks = bank.0[idx];
    bank.0[idx] = 0;
    while blocks > 0 {
        idx = (idx + 1) % bank.0.len();
        bank.0[idx] += 1;
        blocks -= 1;
    }
}

fn bothparts(input: &Bank) -> (usize, usize) {
    let mut sets: HashMap<Vec<i64>, usize> = HashMap::new();
    let mut bank = input.clone();
    sets.insert(bank.0.clone(), 0);
    let mut iters = 0;
    loop {
        iters += 1;
        realloc(&mut bank);
        if sets.contains_key(&bank.0) {
            return (iters, iters - sets[&bank.0]);
        }
        sets.insert(bank.0.clone(), iters);
    }
}

fn main() {
    let input: Vec<Bank> = read_input();
    let (part1, part2) = bothparts(&input[0]);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_test() {
        let mut b = "0\t2\t7\t0".parse::<Bank>().unwrap();
        realloc(&mut b);
        assert_eq!(b.0, vec![2_i64, 4, 1, 2]);
        realloc(&mut b);
        assert_eq!(b.0, vec![3_i64, 1, 2, 3]);
        realloc(&mut b);
        assert_eq!(b.0, vec![0_i64, 2, 3, 4]);
        realloc(&mut b);
        assert_eq!(b.0, vec![1_i64, 3, 4, 1]);
        realloc(&mut b);
        assert_eq!(b.0, vec![2_i64, 4, 1, 2]);

        let b = "0\t2\t7\t0".parse::<Bank>().unwrap();
        let (part1, part2) = bothparts(&b);
        assert_eq!(part1, 5);
        assert_eq!(part2, 4);
    }
}
