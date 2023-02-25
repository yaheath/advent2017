use std::vec::Vec;
use advent_lib::read::read_input;
use advent2017::{knot_hash, knot_hash_raw};

fn part1(input: &str) -> usize {
    let nums: Vec<usize> = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut list = Vec::from_iter(0u8..=255);
    knot_hash_raw(&mut list, &nums, 1);
    list[0] as usize * list[1] as usize
}

fn part2(input: &str) -> String {
    knot_hash(input)
        .iter()
        .map(|&n| format!("{n:02x}"))
        .collect()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_test() {
        let mut list = Vec::from_iter(0u8..5);
        let input = vec![3,4,1,5];
        knot_hash_raw(&mut list, &input, 1);
        assert_eq!(list[0] * list[1], 12);

        assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
