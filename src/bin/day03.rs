use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;
use advent_lib::infinite_grid::InfiniteGrid;

fn calc(target: i64) -> (i64, i64) {
    if target <= 1 {
        return (0, 0);
    }
    let mut counter = 9;
    let mut prev_counter = 1;
    let mut level = 1;

    while counter < target {
        let outer = counter - prev_counter;
        let addl = (outer / 4 + 2) * 4;
        prev_counter = counter;
        counter += addl;
        level += 1;
    }
    let outer = counter - prev_counter;
    let offset = target - prev_counter - 1;
    let side_len = outer / 4;
    let side_offset = 1 - side_len / 2 + (offset % side_len);
    match offset / side_len {
        0 => (level, side_offset),
        1 => (-side_offset, level),
        2 => (-level, -side_offset),
        3 => (side_offset, -level),
        _ => panic!(),
    }
}

fn part1(input: i64) -> i64 {
    let (x, y) = calc(input);
    //println!("input: {input} coord: {x}, {y}");
    x.abs() + y.abs()
}

fn part2(input: i64) -> i64 {
    let mut grid: InfiniteGrid<i64> = InfiniteGrid::new(0);
    grid.set(0, 0, 1);
    let mut lastnum = 1;
    let mut index = 2;
    while lastnum <= input {
        let (x, y) = calc(index);
        let sum: i64 = (-1..=1).cartesian_product(-1..=1)
            .filter(|(xo, yo)| *xo != 0 || *yo != 0)
            .map(|(xo, yo)| grid.get(x + xo, y + yo))
            .sum();
        grid.set(x, y, sum);
        lastnum = sum;
        index += 1;
    }
    lastnum
}

fn main() {
    let input: Vec<i64> = read_input();
    println!("Part 1: {}", part1(input[0]));
    println!("Part 2: {}", part2(input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_test() {
        assert_eq!(part1(1), 0);
        assert_eq!(part1(12), 3);
        assert_eq!(part1(23), 2);
        assert_eq!(part1(22), 3);
        assert_eq!(part1(16), 3);
        assert_eq!(part1(15), 2);
        assert_eq!(part1(1024), 31);
        assert_eq!(part2(10), 11);
        assert_eq!(part2(60), 122);
    }
}
