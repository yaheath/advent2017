use std::vec::Vec;
use linked_list::LinkedList;
use advent_lib::read::read_input;

fn part1(steps: usize) -> usize {
    let mut ring: LinkedList<usize> = LinkedList::new();
    ring.push_front(0);
    let mut current_pos = 0;
    for n in 1..=2017 {
        current_pos = (current_pos + steps) % ring.len();
        ring.insert(current_pos + 1, n);
        current_pos += 1;
    }
    let mut cursor = ring.cursor();
    cursor.seek_forward(current_pos + 1);
    *cursor.peek_next().unwrap()
}

fn part2(steps: usize) -> usize {
    let mut current_pos = 0;
    let mut ring_len = 1;
    let mut last_after_zero = 0;
    for n in 1..=50_000_000 {
        current_pos = (current_pos + steps) % ring_len;
        if current_pos == 0 { last_after_zero = n; }
        current_pos += 1;
        ring_len += 1;
    }
    last_after_zero
}

fn main() {
    let input: Vec<usize> = read_input();
    println!("Part 1: {}", part1(input[0]));
    println!("Part 2: {}", part2(input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_test() {
        assert_eq!(part1(3), 638);
    }
}
