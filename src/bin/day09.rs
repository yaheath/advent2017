use std::vec::Vec;
use std::iter::Iterator;
use std::str::FromStr;
use ya_advent_lib::read::read_input;

enum GG {
    Group(Box<Group>),
    Garbage(String),
}

struct Group {
    children: Vec<GG>,
}
impl Group {
    fn parse(stream: &mut dyn Iterator<Item=char>) -> Self {
        let mut children: Vec<GG> = Vec::new();
        loop {
            match stream.next().unwrap() {
                ',' => { continue; },
                '{' => {
                    children.push(
                        GG::Group(Box::new(Self::parse(stream)))
                    );
                },
                '<' => {
                    children.push(
                        GG::Garbage(Self::parse_garbage(stream))
                    );
                },
                '}' => { break; },
                _ => panic!(),
            }
        }
        Group { children }
    }
    fn parse_garbage(stream: &mut dyn Iterator<Item=char>) -> String {
        let mut ret = String::new();
        loop {
            match stream.next().unwrap() {
                '!' => { stream.next().unwrap(); },
                '>' => { break; },
                c => { ret.push(c); },
            }
        }
        ret
    }
}
impl FromStr for Group {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        assert_eq!(iter.next(), Some('{'));
        Ok(Group::parse(&mut iter))
    }
}

fn rscore(group: &Group, level: i64) -> i64 {
    level + group.children.iter()
        .filter(|gg| matches!(gg, GG::Group(_)))
        .map(|gg| match gg { GG::Group(g) => g, _ => panic!() })
        .map(|g| rscore(g, level + 1))
        .sum::<i64>()
}

fn score(group: &Group) -> i64 {
    rscore(group, 1)
}

fn count_canceled(group: &Group) -> i64 {
    group.children
        .iter()
        .map(|g| match g {
            GG::Garbage(s) => s.len() as i64,
            GG::Group(sg) => count_canceled(sg),
        })
        .sum()
}

fn part1(input: &[Group]) -> i64 {
    score(&input[0])
}

fn part2(input: &[Group]) -> i64 {
    count_canceled(&input[0])
}

fn main() {
    let input: Vec<Group> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_test() {
        let g = "{}".parse::<Group>().unwrap();
        assert_eq!(score(&g), 1);
        let g = "{{{}}}".parse::<Group>().unwrap();
        assert_eq!(score(&g), 6);
        let g = "{{},{}}".parse::<Group>().unwrap();
        assert_eq!(score(&g), 5);
    }
}
