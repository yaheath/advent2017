use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Clone,Copy,Debug)]
enum HexDir {
    NW,
    N,
    NE,
    SW,
    S,
    SE,
}
impl FromStr for HexDir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nw" => Ok(HexDir::NW),
            "n"  => Ok(HexDir::N),
            "ne" => Ok(HexDir::NE),
            "se" => Ok(HexDir::SE),
            "s"  => Ok(HexDir::S),
            "sw" => Ok(HexDir::SW),
            _ => Err(()),
        }
    }
}
impl HexDir {
    fn mov(&self, c: (i64, i64)) -> (i64, i64) {
        let mut x = c.0;
        let mut y = c.1;
        match self {
            HexDir::N => { y += 1; },
            HexDir::S => { y -= 1; },
            HexDir::NE if x & 1 == 0 => { x += 1; },
            HexDir::NE               => { x += 1; y += 1; },
            HexDir::NW if x & 1 == 0 => { x -= 1; },
            HexDir::NW               => { x -= 1; y += 1; },
            HexDir::SE if x & 1 == 0 => { x += 1; y -= 1; },
            HexDir::SE               => { x += 1; },
            HexDir::SW if x & 1 == 0 => { x -= 1; y -= 1; },
            HexDir::SW               => { x -= 1; },
        }
        (x, y)
    }
    fn dist(frm: (i64, i64), to: (i64, i64)) -> i64 {
        if to.0 == frm.0 { return (to.1 - frm.1).abs(); }
        let xdiff = (to.0 - frm.0).abs();
        let to_ys = to.1 * 2 + (to.0 & 1);
        let frm_ys = frm.1 * 2 + (frm.0 & 1);
        let ydiff = (to_ys - frm_ys).abs();

        if ydiff <= xdiff {
            return xdiff;
        }
        (ydiff - xdiff) / 2 + xdiff
    }
}

struct Movements(Vec<HexDir>);
impl FromStr for Movements {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(',').map(|s| s.parse::<HexDir>().unwrap()).collect()))
    }
}

fn part1(input: &Movements) -> i64 {
    let mut p = (0, 0);
    for m in &input.0 {
        p = m.mov(p);
    }
    HexDir::dist(p, (0, 0))
}

fn part2(input: &Movements) -> i64 {
    let mut p = (0, 0);
    let mut max = 0;
    for m in &input.0 {
        p = m.mov(p);
        max = max.max(HexDir::dist(p, (0,0)));
    }
    max
}

fn main() {
    let input: Vec<Movements> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_test() {
        assert_eq!(HexDir::dist((3, 0), (0, 0)), 3);
        assert_eq!(HexDir::dist((2, -3), (0, 0)), 4);
        assert_eq!(HexDir::dist((-3, -3), (0, 0)), 4);
        assert_eq!(HexDir::dist((3, -4), (0, 0)), 5);
        assert_eq!(HexDir::dist((2, 2), (0, 0)), 3);
        assert_eq!(HexDir::dist((3, 2), (0, 0)), 4);

        let m = "ne,ne,ne".parse::<Movements>().unwrap();
        assert_eq!(part1(&m), 3);
        let m = "ne,ne,sw,sw".parse::<Movements>().unwrap();
        assert_eq!(part1(&m), 0);
        let m = "ne,ne,s,s".parse::<Movements>().unwrap();
        assert_eq!(part1(&m), 2);
        let m = "se,sw,se,sw,sw".parse::<Movements>().unwrap();
        assert_eq!(part1(&m), 3);
    }
}
