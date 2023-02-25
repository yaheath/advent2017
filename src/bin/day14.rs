use std::collections::HashSet;
use std::vec::Vec;
use lazy_static::lazy_static;
use advent_lib::read::read_input;
use advent2017::knot_hash;

lazy_static! {
    static ref ONEBITS: Vec<usize> = {
        let mut v = Vec::with_capacity(256);
        v.push(0);
        for i in 1..256 {
            let n = (i & 1) + v[i / 2];
            v.push(n);
        }
        v
    };
}

struct HashGrid {
    rows: Vec<Vec<u8>>,
}
impl HashGrid {
    fn new(input: &str) -> Self {
        let rows = (0..128).map(|n| {
            let key = format!("{input}-{n}");
            knot_hash(&key)
        })
        .collect();
        Self { rows }
    }
    fn get(&self, x: usize, y: usize) -> bool {
        let xi = x / 8;
        let xb = 7 - (x % 8);
        (self.rows[y][xi] >> xb) & 1 == 1
    }
    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize,usize)> {
        let mut ret = Vec::new();
        if x > 0 { ret.push((x - 1, y)); }
        if x < 127 { ret.push((x + 1, y)); }
        if y > 0 { ret.push((x, y - 1)); }
        if y < 127 { ret.push((x, y + 1)); }
        ret
    }
}

fn part1(input: &str) -> usize {
    let grid = HashGrid::new(input);
    grid.rows
        .iter()
        .map(|hash| hash.iter().map(|b| ONEBITS[*b as usize]).sum::<usize>())
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = HashGrid::new(input);
    let mut found: HashSet<(usize,usize)> = HashSet::new();
    let mut ngroups = 0;
    for y in 0..128 {
        for x in 0..128 {
            if grid.get(x, y) && !found.contains(&(x, y)) {
                let mut queue: Vec<(usize, usize)> = Vec::new();
                found.insert((x, y));
                queue.extend_from_slice(&grid.neighbors(x, y));
                while let Some((nx, ny)) = queue.pop() {
                    if found.contains(&(nx, ny)) { continue; }
                    if grid.get(nx, ny) {
                        found.insert((nx, ny));
                        queue.extend_from_slice(&grid.neighbors(nx, ny));
                    }
                }
                ngroups += 1;
            }
        }
    }
    ngroups
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
    fn day14_test() {
        assert_eq!(part1("flqrgnkx"), 8108);
        assert_eq!(part2("flqrgnkx"), 1242);
    }
}
