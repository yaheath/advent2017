use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::infinite_grid::InfiniteGrid;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Dir {
    N, S, E, W,
}
impl Dir {
    fn left(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }
    fn right(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    fn x_off(&self) -> i64 {
        match self {
            Dir::E => 1,
            Dir::W => -1,
            _ => 0,
        }
    }
    fn y_off(&self) -> i64 {
        match self {
            Dir::S => 1,
            Dir::N => -1,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

struct Virus {
    grid: InfiniteGrid<NodeState>,
    dir: Dir,
    pos: (i64, i64),
    num_infected: usize,
}

impl Virus {
    fn new(initial_grid: &InfiniteGrid<bool>) -> Self {
        let xrange = initial_grid.x_bounds();
        let yrange = initial_grid.y_bounds();
        Self {
            grid: InfiniteGrid::from_other(
                initial_grid,
                NodeState::Clean,
                |v| if v {Some(NodeState::Infected)} else {None}
            ),
            dir: Dir::N,
            pos: (
                xrange.start + (xrange.end - xrange.start) / 2,
                yrange.start + (yrange.end - yrange.start) / 2,
            ),
            num_infected: 0,
        }
    }
    fn step(&mut self) {
        let state = self.grid.get(self.pos.0, self.pos.1);
        self.dir = match state {
            NodeState::Infected => self.dir.right(),
            NodeState::Clean => self.dir.left(),
            _ => panic!(),
        };
        let newstate = match state {
            NodeState::Infected => NodeState::Clean,
            NodeState::Clean => NodeState::Infected,
            _ => panic!(),
        };
        self.grid.set(self.pos.0, self.pos.1, newstate);
        if newstate == NodeState::Infected {
            self.num_infected += 1;
        }
        self.pos = (self.pos.0 + self.dir.x_off(), self.pos.1 + self.dir.y_off())
    }
    fn step_evolved(&mut self) {
        let state = self.grid.get(self.pos.0, self.pos.1);
        self.dir = match state {
            NodeState::Clean => self.dir.left(),
            NodeState::Weakened => self.dir,
            NodeState::Infected => self.dir.right(),
            NodeState::Flagged => self.dir.right().right(),
        };
        let newstate = match state {
            NodeState::Clean => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged => NodeState::Clean,
        };
        self.grid.set(self.pos.0, self.pos.1, newstate);
        if newstate == NodeState::Infected {
            self.num_infected += 1;
        }
        self.pos = (self.pos.0 + self.dir.x_off(), self.pos.1 + self.dir.y_off())
    }
}

fn part1(input: &InfiniteGrid<bool>) -> usize {
    let mut virus = Virus::new(input);
    for _ in 0..10000 {
        virus.step();
    }
    virus.num_infected
}

fn part2(input: &InfiniteGrid<bool>) -> usize {
    let mut virus = Virus::new(input);
    for _ in 0..10_000_000 {
        virus.step_evolved();
    }
    virus.num_infected
}

fn make_grid(input: Vec<String>) -> InfiniteGrid<bool> {
    InfiniteGrid::from_input(&input, false, |c, _, _| Some(c == '#'))
}

fn main() {
    let input: Vec<String> = read_input();
    let grid = make_grid(input);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day22_test() {
        let input: Vec<String> = test_input(include_str!("day22.testinput"));
        let grid = make_grid(input);
        assert_eq!(part1(&grid), 5587);
        assert_eq!(part2(&grid), 2511944);
    }
}
