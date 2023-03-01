use std::cmp::Reverse;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use advent_lib::read::read_input;

type Coord3D = (i64, i64, i64);

#[derive(Clone)]
struct Particle {
    pos: Coord3D,
    vel: Coord3D,
    acc: Coord3D,
}

impl FromStr for Particle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"p=.(-?\d+),(-?\d+),(-?\d+)., v=.(-?\d+),(-?\d+),(-?\d+)., a=.(-?\d+),(-?\d+),(-?\d+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let px = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let py = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let pz = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let vx = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let vy = caps.get(5).unwrap().as_str().parse::<i64>().unwrap();
            let vz = caps.get(6).unwrap().as_str().parse::<i64>().unwrap();
            let ax = caps.get(7).unwrap().as_str().parse::<i64>().unwrap();
            let ay = caps.get(8).unwrap().as_str().parse::<i64>().unwrap();
            let az = caps.get(9).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Particle {
                pos: (px, py, pz),
                vel: (vx, vy, vz),
                acc: (ax, ay, az),
            })
        }
        else {
            Err(())
        }
    }
}

impl Particle {
    fn update(&mut self) {
        self.vel.0 += self.acc.0;
        self.vel.1 += self.acc.1;
        self.vel.2 += self.acc.2;
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }
}

fn part1(input: &Vec<Particle>) -> usize {
    input.iter()
        .enumerate()
        .map(|(idx, p)| (idx, p.acc.0.abs() + p.acc.1.abs() + p.acc.2.abs()))
        .min_by_key(|(_, v)| *v)
        .map(|(idx, _)| idx)
        .unwrap()
}

fn part2(input: &Vec<Particle>) -> usize {
    let mut particles = input.clone();

    for _ in 0..1000 {
        let mut positions: HashMap<Coord3D, Vec<usize>> = HashMap::new();
        for (idx, p) in particles.iter_mut().enumerate() {
            p.update();
            positions.entry(p.pos)
                .and_modify(|e| e.push(idx))
                .or_insert(vec![idx]);
        }
        for idx in positions.values()
            .filter(|v| v.len() > 1)
            .flatten()
            .sorted_by_key(|v| Reverse(*v)) {
                particles.remove(*idx);
        }
    }
    particles.len()
}

fn main() {
    let input: Vec<Particle> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day20_test() {
        let input: Vec<Particle> = test_input(include_str!("day20.testinput"));
        assert_eq!(part1(&input), 0);
        let input: Vec<Particle> = test_input(include_str!("day20.testinput2"));
        assert_eq!(part2(&input), 1);
    }
}
