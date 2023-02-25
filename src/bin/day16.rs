use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}
impl FromStr for DanceMove {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        let ss = s.get(1..).unwrap();
        match c {
            's' => Ok(DanceMove::Spin(ss.parse::<usize>().unwrap())),
            'x' => {
                let mut i = ss.split('/');
                let a = i.next().unwrap().parse::<usize>().unwrap();
                let b = i.next().unwrap().parse::<usize>().unwrap();
                Ok(DanceMove::Exchange(a, b))
            },
            'p' => {
                let mut i = ss.split('/');
                let a = i.next().unwrap().chars().next().unwrap();
                let b = i.next().unwrap().chars().next().unwrap();
                Ok(DanceMove::Partner(a, b))
            },
            _ => Err(()),
        }
    }
}

struct DanceMoves(Vec<DanceMove>);

impl FromStr for DanceMoves {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self (
            s.split(',')
             .map(|ss| ss.parse::<DanceMove>().unwrap())
             .collect()
        ))
    }
}

fn dance(moves: &Vec<DanceMove>, mut progs: Vec<char>) -> Vec<char> {
    let n_progs = progs.len();
    let mut front: usize = 0;
    for m in moves {
        match m {
            DanceMove::Spin(n) => {
                front = (n_progs + front - n) % n_progs;
            },
            DanceMove::Exchange(i, j) => {
                let ii = (front + i) % n_progs;
                let jj = (front + j) % n_progs;
                progs.swap(ii, jj);
            },
            DanceMove::Partner(a, b) => {
                let i = progs.iter().position(|&x| x == *a).unwrap();
                let j = progs.iter().position(|&x| x == *b).unwrap();
                progs.swap(i, j);
            },
        }
    }
    (0..n_progs)
        .map(|idx| (front + idx) % n_progs)
        .map(|idx| progs[idx])
        .collect()
}

fn dances(moves: &Vec<DanceMove>, n_progs: usize, n_dances: usize) -> String {
    let mut progs: Vec<char> = Vec::from_iter('a'..(b'a' + n_progs as u8) as char);
    let mut patterns: HashMap<Vec<char>, usize> = HashMap::new();
    patterns.insert(progs.clone(), 0);
    for i in 1..=n_dances {
        progs = dance(moves, progs);
        if i == n_dances { break; }
        if patterns.contains_key(&progs) {
            let rpt_start = patterns[&progs];
            let rpt_len = i - rpt_start;
            let idx = (n_dances - rpt_start) % rpt_len + rpt_start;
            progs = patterns.iter().find(|(_, &n)| n == idx).unwrap().0.clone();
            break;
        }
        patterns.insert(progs.clone(), i);
    }
    progs.iter().collect()
}

fn part1(input: &Vec<DanceMove>) -> String {
    dances(input, 16, 1)
}

fn part2(input: &Vec<DanceMove>) -> String {
    dances(input, 16, 1_000_000_000)
}

fn main() {
    let input: Vec<DanceMoves> = read_input();
    println!("Part 1: {}", part1(&input[0].0));
    println!("Part 2: {}", part2(&input[0].0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_test() {
        let input = "s1,x3/4,pe/b".parse::<DanceMoves>().unwrap();
        assert_eq!(dances(&input.0, 5, 1), "baedc".to_string());
        assert_eq!(dances(&input.0, 5, 2), "ceadb".to_string());
    }
}
