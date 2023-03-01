use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use advent_lib::read::read_input;
use advent_lib::grid::Grid;

struct Input {
    frm: Grid<bool>,
    to: Grid<bool>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut frmto = s.split(" => ");
        let frmlines: Vec<String> =
            frmto.next().unwrap().split('/').map(|s| s.to_string()).collect();
        let tolines: Vec<String> =
            frmto.next().unwrap().split('/').map(|s| s.to_string()).collect();
        Ok(Self {
            frm: Grid::from_input(&frmlines, false, 0, |c| c == '#'),
            to: Grid::from_input(&tolines, false, 0, |c| c == '#'),
        })
    }
}

lazy_static! {
static ref START: Vec<String> = vec![
    ".#.".to_string(),
    "..#".to_string(),
    "###".to_string(),
];
}

fn iterate(input: &Vec<Input>, n_times: usize) -> Grid<bool> {
    let mut map: HashMap<Vec<bool>, &Grid<bool>> = HashMap::new();
    for i in input {
        map.insert(i.frm.data().clone(), &i.to);
        map.insert(i.frm.h_flip().data().clone(), &i.to);
        map.insert(i.frm.rot90().data().clone(), &i.to);
        map.insert(i.frm.rot90().h_flip().data().clone(), &i.to);
        map.insert(i.frm.rot180().data().clone(), &i.to);
        map.insert(i.frm.rot180().h_flip().data().clone(), &i.to);
        map.insert(i.frm.rot180().rot90().data().clone(), &i.to);
        map.insert(i.frm.rot180().rot90().h_flip().data().clone(), &i.to);
    }
    let mut grid: Grid<bool> = Grid::from_input(&START, false, 0, |c| c == '#');
    for _ in 0..n_times {
        let splitsize = if grid.x_bounds().end % 2 == 0 { 2 } else { 3 };
        let tiles: Vec<Grid<bool>> = grid.tile_split(splitsize, splitsize)
            .map(|g| map[g.data()].clone())
            .collect();
        let nt = grid.x_bounds().end as usize / splitsize;
        grid = Grid::from_tiles(&tiles, nt, nt);
        //grid.print(|c| if c { '#' } else { '.' });
        //println!("");
    }
    grid
}

fn part1(input: &Vec<Input>) -> usize {
    let result = iterate(input, 5);
    result.iter().filter(|c| **c).count()
}

fn part2(input: &Vec<Input>) -> usize {
    let result = iterate(input, 18);
    result.iter().filter(|c| **c).count()
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day21_test() {
        let input: Vec<Input> = test_input(include_str!("day21.testinput"));
        let result = iterate(&input, 2);
        let n_on = result.iter().filter(|c| **c).count();
        assert_eq!(n_on, 12);
        assert_eq!(part2(&input), 3);
    }
}
