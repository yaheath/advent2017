use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn part1(input: &[i64]) -> usize {
    let mut list = input.to_owned();
    let mut steps = 0_usize;
    let mut ip = 0_i64;
    while ip >= 0 && ip < list.len() as i64 {
        let old_ip = ip;
        ip += list[ip as usize];
        list[old_ip as usize] += 1;
        steps += 1
    }
    steps
}

fn part2(input: &[i64]) -> usize {
    let mut list = input.to_owned();
    let mut steps = 0_usize;
    let mut ip = 0_i64;
    while ip >= 0 && ip < list.len() as i64 {
        let old_ip = ip as usize;
        ip += list[ip as usize];
        if list[old_ip] >= 3 {
            list[old_ip] -= 1
        }
        else {
            list[old_ip] += 1;
        }
        steps += 1
    }
    steps
}

fn main() {
    let input: Vec<i64> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day05_test() {
        let input: Vec<i64> = test_input("0\n3\n0\n1\n-3\n".into());
        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 10);
    }
}
