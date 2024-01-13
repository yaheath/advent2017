use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Vert,
    Horiz,
    Corner,
    Label(char),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            ' ' => Cell::Empty,
            '|' => Cell::Vert,
            '-' => Cell::Horiz,
            '+' => Cell::Corner,
            'A'..='Z' => Cell::Label(c),
            _ => panic!(),
        }
    }
}

fn setup(input: &[String]) -> Grid<Cell> {
    Grid::from_input(input, Cell::Empty, 1)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn x_off(&self) -> i64 {
        match self {
            Dir::Left => -1,
            Dir::Right => 1,
            _ => 0,
        }
    }
    fn y_off(&self) -> i64 {
        match self {
            Dir::Up => -1,
            Dir::Down => 1,
            _ => 0,
        }
    }
}

fn bothparts(grid: &Grid<Cell>) -> (String, usize) {
    let mut result = String::new();
    let x_range = grid.x_bounds();
    let mut y = 0;
    let mut x = x_range.into_iter().find(|x| grid.get(*x, y) == Cell::Vert).unwrap();
    let mut dir = Dir::Down;
    let mut steps = 0;
    loop {
        steps += 1;
        let nx = x + dir.x_off();
        let ny = y + dir.y_off();
        let nc = grid.get(nx, ny);
        match (nc, dir) {
            (Cell::Empty, _) => break,
            (Cell::Corner, Dir::Down)
            | (Cell::Corner, Dir::Up) => {
                match (grid.get(nx - 1, ny), grid.get(nx + 1, ny)) {
                    (Cell::Horiz, _) |
                    (Cell::Label(_), Cell::Empty) |
                    (Cell::Label(_), Cell::Vert)
                        => { dir = Dir::Left; },
                    (_, Cell::Horiz) |
                    (Cell::Empty, Cell::Label(_)) |
                    (Cell::Vert, Cell::Label(_))
                        => { dir = Dir::Right; },
                    _ => panic!(),
                }
            },
            (Cell::Corner, Dir::Left)
            | (Cell::Corner, Dir::Right) => {
                match (grid.get(nx, ny - 1), grid.get(nx, ny + 1)) {
                    (Cell::Vert, _) |
                    (Cell::Label(_), Cell::Empty) |
                    (Cell::Label(_), Cell::Horiz)
                        => { dir = Dir::Up; },
                    (_, Cell::Vert) |
                    (Cell::Empty, Cell::Label(_)) |
                    (Cell::Horiz, Cell::Label(_))
                        => { dir = Dir::Down; },
                    _ => panic!(),
                }
            },
            (Cell::Label(c), _) => result.push(c),
            (Cell::Vert, _) | (Cell::Horiz, _) => {},
        }

        x = nx;
        y = ny;
    }
    (result, steps)
}

fn main() {
    let input: Vec<String> = read_input();
    let grid = setup(&input);
    let (part1, part2) = bothparts(&grid);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day19_test() {
        let input: Vec<String> = test_input(include_str!("day19.testinput"));
        let grid = setup(&input);
        let (part1, part2) = bothparts(&grid);
        assert_eq!(part1, "ABCDEF".to_string());
        assert_eq!(part2, 38);
    }
}
