use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl FromStr for Bit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('0') => Ok(Bit::Zero),
            Some('1') => Ok(Bit::One),
            _ => Err(()),
        }
    }
}

enum Input {
    Begin(char),
    Diagnostic(usize),
    State(char),
    If(Bit),
    Write(Bit),
    MoveLeft,
    MoveRight,
    NextState(char),
    Blank,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words:Vec<&str> = s.split_whitespace().collect();
        if words.is_empty() {
            return Ok(Input::Blank);
        }
        match (words[0], words[1]) {
            ("Begin", "in") => {
                let c = words[words.len()-1].chars().next().unwrap();
                Ok(Input::Begin(c))
            },
            ("Perform", "a") => {
                let steps = words[5].parse::<usize>().unwrap();
                Ok(Input::Diagnostic(steps))
            },
            ("In", "state") => {
                let c = words[words.len()-1].chars().next().unwrap();
                Ok(Input::State(c))
            },
            ("If", "the") => {
                let b = words[words.len()-1].parse::<Bit>().unwrap();
                Ok(Input::If(b))
            },
            ("-", "Write") => {
                let b = words[words.len()-1].parse::<Bit>().unwrap();
                Ok(Input::Write(b))
            },
            ("-", "Move") => {
                match words[words.len()-1] {
                    "right." => Ok(Input::MoveRight),
                    "left." => Ok(Input::MoveLeft),
                    _ => Err(()),
                }
            },
            ("-", "Continue") => {
                let c = words[words.len()-1].chars().next().unwrap();
                Ok(Input::NextState(c))
            },
            _ => Err(()),
        }
    }
}

enum Dir { Left, Right }

struct Actions {
    write: Bit,
    mov: Dir,
    next: char,
}
impl Actions {
    fn from_iter(iter: &mut dyn Iterator<Item=&Input>) -> Self {
        let mut write: Option<Bit> = None;
        let mut mov: Option<Dir> = None;
        let mut next: Option<char> = None;
        while write.is_none() || mov.is_none() || next.is_none() {
            match iter.next() {
                Some(Input::Write(b)) => { write = Some(*b); },
                Some(Input::MoveLeft) => { mov = Some(Dir::Left); },
                Some(Input::MoveRight) => { mov = Some(Dir::Right); },
                Some(Input::NextState(c)) => { next = Some(*c); },
                _ => panic!(),
            }
        }
        Self {
            write: write.unwrap(),
            mov: mov.unwrap(),
            next: next.unwrap(),
        }
    }
}

struct State {
    zero_actions: Actions,
    one_actions: Actions,
}
impl State {
    fn from_iter(iter: &mut dyn Iterator<Item=&Input>) -> Self {
        let mut zero_actions: Option<Actions> = None;
        let mut one_actions: Option<Actions> = None;
        while zero_actions.is_none() || one_actions.is_none() {
            match iter.next() {
                Some(Input::If(Bit::Zero)) => {
                    zero_actions = Some(Actions::from_iter(iter));
                },
                Some(Input::If(Bit::One)) => {
                    one_actions = Some(Actions::from_iter(iter));
                },
                _ => panic!(),
            }
        }
        Self {
            zero_actions: zero_actions.unwrap(),
            one_actions: one_actions.unwrap(),
        }
    }
}

struct Program {
    initial_state: char,
    states: HashMap<char, State>,
    checksum_step: usize,
}
impl Program {
    fn from_input(input: &[Input]) -> Self {
        let mut iter = input.iter().peekable();
        let mut initial_state: Option<char> = None;
        let mut checksum_step: Option<usize> = None;
        let mut states: HashMap<char, State> = HashMap::new();

        // top level
        while let Some(row) = iter.next() {
            match row {
                Input::Begin(c) => {
                    initial_state = Some(*c);
                },
                Input::Diagnostic(s) => {
                    checksum_step = Some(*s);
                },
                Input::State(s) => {
                    states.insert(*s, State::from_iter(&mut iter));
                },
                Input::Blank => {},
                _ => panic!(),
            }
        }

        Self {
            initial_state: initial_state.unwrap(),
            states,
            checksum_step: checksum_step.unwrap(),
        }
    }
}

struct TuringMachine {
    tape: HashMap<i64, Bit>,
    program: Program,
    state: char,
    cursor: i64,
}
impl TuringMachine {
    fn from_input(input: &[Input]) -> Self {
        let program = Program::from_input(input);
        Self {
            tape: HashMap::new(),
            state: program.initial_state,
            program,
            cursor: 0,
        }
    }
    fn step(&mut self) {
        let state = &self.program.states[&self.state];
        let val = self.tape.get(&self.cursor).unwrap_or(&Bit::Zero);
        let actions = match val {
            Bit::Zero => &state.zero_actions,
            Bit::One => &state.one_actions,
        };
        self.tape.insert(self.cursor, actions.write);
        self.cursor += match actions.mov {
            Dir::Right => 1,
            Dir::Left => -1,
        };
        self.state = actions.next;
    }
}

fn part1(input: &[Input]) -> usize {
    let mut tm = TuringMachine::from_input(input);
    for _ in 0..tm.program.checksum_step {
        tm.step();
    }
    tm.tape.values().filter(|&&v| v == Bit::One).count()
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day25_test() {
        let input: Vec<Input> = test_input(include_str!("day25.testinput"));
        assert_eq!(part1(&input), 3);
    }
}
