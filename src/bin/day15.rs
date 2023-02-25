use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Clone)]
struct Generator {
    factor: u64,
    current: u64,
    mult: u64,
}

impl Generator {
    fn new(generator: char, seed: u64) -> Self {
        let (factor, mult) = match generator {
            'A' => (16807, 4),
            'B' => (48271, 8),
            _ => panic!(),
        };
        Self { factor, mult, current: seed }
    }
    fn next(&mut self) -> u64 {
        self.current = (self.current * self.factor) % 2147483647;
        self.current
    }
    fn next2(&mut self) -> u64 {
        loop {
            self.current = (self.current * self.factor) % 2147483647;
            if self.current % self.mult == 0 {
                break;
            }
        }
        self.current
    }
}

impl FromStr for Generator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items:Vec<&str> = s.split(' ').collect();
        let generator = items[1].chars().next().unwrap();
        let seed = items[4].parse::<u64>().unwrap();
        Ok(Self::new(generator, seed))
    }
}

fn part1(input: &Vec<Generator>) -> usize {
    let mut gen0 = input[0].clone();
    let mut gen1 = input[1].clone();
    (0..40_000_000).filter(|_| {
        let v0 = gen0.next();
        let v1 = gen1.next();
        (v0 & 0xffff) == (v1 & 0xffff)
    })
    .count()
}

fn part2(input: &Vec<Generator>) -> usize {
    let mut gen0 = input[0].clone();
    let mut gen1 = input[1].clone();
    (0..5_000_000).filter(|_| {
        let v0 = gen0.next2();
        let v1 = gen1.next2();
        (v0 & 0xffff) == (v1 & 0xffff)
    })
    .count()
}

fn main() {
    let input: Vec<Generator> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_test() {
        let mut ga = Generator::new('A', 65);
        assert_eq!(ga.next(), 1092455);
        assert_eq!(ga.next(), 1181022009);
        assert_eq!(ga.next(), 245556042);
        assert_eq!(ga.next(), 1744312007);
        assert_eq!(ga.next(), 1352636452);

        let input = vec![
            Generator::new('A', 65),
            Generator::new('B', 8921),
        ];
        assert_eq!(part1(&input), 588);

        let mut ga = Generator::new('A', 65);
        assert_eq!(ga.next2(), 1352636452);
        assert_eq!(ga.next2(), 1992081072);
        assert_eq!(ga.next2(), 530830436);
        assert_eq!(ga.next2(), 1980017072);
        assert_eq!(ga.next2(), 740335192);

        assert_eq!(part2(&input), 309);
    }
}
