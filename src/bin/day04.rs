use std::collections::HashSet;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

struct Passphrase(Vec<String>);
impl FromStr for Passphrase {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Passphrase(s.split(' ').map(|s| s.to_string()).collect()))
    }
}
impl Passphrase {
    fn is_valid(&self) -> bool {
        let set: HashSet<String> = HashSet::from_iter(self.0.iter().cloned());
        set.len() == self.0.len()
    }
    fn is_valid2(&self) -> bool {
        let set: HashSet<String> = HashSet::from_iter(
            self.0.iter().map(|s| s.chars().sorted().collect::<String>())
        );
        set.len() == self.0.len()
    }
}

fn part1(input: &[Passphrase]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

fn part2(input: &[Passphrase]) -> usize {
    input.iter().filter(|p| p.is_valid2()).count()
}

fn main() {
    let input: Vec<Passphrase> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_test() {
        let p = "aa bb cc dd ee".parse::<Passphrase>().unwrap();
        assert!(p.is_valid());
        let p = "aa bb cc dd aa".parse::<Passphrase>().unwrap();
        assert!(!p.is_valid());
        let p = "aa bb cc dd aaa".parse::<Passphrase>().unwrap();
        assert!(p.is_valid());
        let p = "abcde fghij".parse::<Passphrase>().unwrap();
        assert!(p.is_valid2());
        let p = "abcde xyz ecdab".parse::<Passphrase>().unwrap();
        assert!(!p.is_valid2());
        let p = "a ab abc abd abf abj".parse::<Passphrase>().unwrap();
        assert!(p.is_valid2());
        let p = "iiii oiii ooii oooi oooo".parse::<Passphrase>().unwrap();
        assert!(p.is_valid2());
        let p = "oiii ioii iioi iiio".parse::<Passphrase>().unwrap();
        assert!(!p.is_valid2());
    }
}
