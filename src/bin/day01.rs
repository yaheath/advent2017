use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;

fn part1(nums: &Vec<i64>) -> i64 {
    nums
        .iter()
        .circular_tuple_windows()
        .fold(0, |sum, (a, b)| if a == b { sum + a } else { sum })
}

fn part2(nums: &Vec<i64>) -> i64 {
    let mid = nums.len() / 2;
    nums.iter()
        .enumerate()
        .fold(0, |sum, (idx, n)| {
            if nums[(idx + mid) % nums.len()] == *n {
                sum + n
            } else {
                sum
            }
        })
}

fn setup(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|c| (c as u8 - b'0') as i64)
        .collect()
}

fn main() {
    let input: Vec<String> = read_input();
    let nums = setup(&input[0]);
    println!("Part 1: {}", part1(&nums));
    println!("Part 2: {}", part2(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_test() {
        let nums = setup("1122");
        assert_eq!(part1(&nums), 3);
        let nums = setup("1111");
        assert_eq!(part1(&nums), 4);
        let nums = setup("1234");
        assert_eq!(part1(&nums), 0);
        let nums = setup("91212129");
        assert_eq!(part1(&nums), 9);
        let nums = setup("1212");
        assert_eq!(part2(&nums), 6);
        let nums = setup("1221");
        assert_eq!(part2(&nums), 0);
        let nums = setup("123425");
        assert_eq!(part2(&nums), 4);
        let nums = setup("123123");
        assert_eq!(part2(&nums), 12);
        let nums = setup("12131415");
        assert_eq!(part2(&nums), 4);
    }
}
