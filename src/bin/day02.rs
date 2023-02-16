use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use advent_lib::read::read_input;

struct Row(Vec<i64>);
impl FromStr for Row {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Row(s.split('\t').map(|n| n.parse::<i64>().unwrap()).collect()))
    }
}

fn part1(input: &Vec<Row>) -> i64 {
    input
        .iter()
        .map(|row| match row.0.iter().minmax() {
            MinMax(&min, &max) => max - min,
            _ => 0,
        })
        .sum()
}

fn part2(input: &Vec<Row>) -> i64 {
    input
        .iter()
        .flat_map(|row|
            row.0.iter()
            .sorted()
            .tuple_combinations()
            .find(|(&a, &b)| b % a == 0)
            .map(|(&a, &b)| b / a)
        )
        .sum()
}

fn main() {
    let input: Vec<Row> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day02_test() {
        let input: Vec<Row> = test_input(include_str!("day02.testinput"));
        assert_eq!(part1(&input), 18);
        let input: Vec<Row> = test_input(include_str!("day02.testinput2"));
        assert_eq!(part2(&input), 9);
    }
}
