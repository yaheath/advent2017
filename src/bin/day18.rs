use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Clone, Copy)]
enum RI {
    Reg(char),
    Imm(i64),
}
impl FromStr for RI {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = s.chars().next().unwrap();
        match f {
            'a' ..= 'z' => Ok(RI::Reg(f)),
            _ => Ok(RI::Imm(s.parse::<i64>().unwrap())),
        }
    }
}

enum Instruction {
    Snd(RI),
    Set(char, RI),
    Add(char, RI),
    Mul(char, RI),
    Mod(char, RI),
    Rcv(char),
    Jgz(RI, RI),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(" ");
        let opcode = itr.next().unwrap();
        let x = itr.next().unwrap();
        let a = x.parse::<RI>().unwrap();
        let b = itr.next().map(|v| v.parse::<RI>().unwrap());
        let x = x.chars().next().unwrap();
        match opcode {
            "snd" => Ok(Instruction::Snd(a)),
            "set" => Ok(Instruction::Set(x, b.unwrap())),
            "add" => Ok(Instruction::Add(x, b.unwrap())),
            "mul" => Ok(Instruction::Mul(x, b.unwrap())),
            "mod" => Ok(Instruction::Mod(x, b.unwrap())),
            "rcv" => Ok(Instruction::Rcv(x)),
            "jgz" => Ok(Instruction::Jgz(a, b.unwrap())),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum RunResult {
    Ok,
    Halt,
    Snd(i64),
    Rcv(char),
}

struct VM<'a> {
    registers: HashMap<char, i64>,
    program: &'a [Instruction],
    pc: i64,
}
impl<'a> VM<'a> {
    fn new(program: &'a [Instruction], id: i64) -> Self {
        let registers = HashMap::from_iter([('p', id)]);
        Self { registers, program, pc: 0 }
    }
    fn step(&mut self) -> RunResult {
        if self.pc < 0 || self.pc >= self.program.len() as i64 {
            return RunResult::Halt;
        }
        let inst = &self.program[self.pc as usize];
        match inst {
            Instruction::Snd(x) => {
                self.pc += 1;
                return RunResult::Snd(self.resolve(*x));
            }
            Instruction::Set(x, y) => {
                *self.registers.entry(*x).or_insert(0) = self.resolve(*y);
            }
            Instruction::Add(x, y) => {
                *self.registers.entry(*x).or_insert(0) += self.resolve(*y);
            }
            Instruction::Mul(x, y) => {
                *self.registers.entry(*x).or_insert(0) *= self.resolve(*y);
            }
            Instruction::Mod(x, y) => {
                *self.registers.entry(*x).or_insert(0) %= self.resolve(*y);
            }
            Instruction::Rcv(x) => {
                self.pc += 1;
                return RunResult::Rcv(*x);
            },
            Instruction::Jgz(x, y) => {
                if self.resolve(*x) > 0 {
                    self.pc += self.resolve(*y) - 1;
                }
            },
        }
        self.pc += 1;
        if self.pc < 0 || self.pc >= self.program.len() as i64 {
            RunResult::Halt
        }
        else {
            RunResult::Ok
        }
    }
    fn resolve(&self, ri: RI) -> i64 {
        match ri {
            RI::Imm(x) => x,
            RI::Reg(r) => *self.registers.get(&r).unwrap_or(&0),
        }
    }
    fn get_reg(&self, r: char) -> i64 {
        *self.registers.get(&r).unwrap_or(&0)
    }
    fn set_reg(&mut self, r: char, v: i64) {
        self.registers.insert(r, v);
    }
    fn run(&mut self) -> RunResult {
        loop {
            let r = self.step();
            match r {
                RunResult::Ok => {},
                _ => return r,
            }
        }
    }
}

fn part1(input: &[Instruction]) -> i64 {
    let mut vm = VM::new(input, 0);
    let mut freq = 0_i64;
    loop {
        let r = vm.step();
        match r {
            RunResult::Ok => {},
            RunResult::Halt => panic!(),
            RunResult::Rcv(r) => {
                if vm.get_reg(r) != 0 {
                    return freq;
                }
            },
            RunResult::Snd(v) => {
                freq = v;
            },
        }
    }
}

struct Worker<'a> {
    vm: VM<'a>,
    queue: VecDeque<i64>,
    state: RunResult,
    n_sends: i64,
}

enum WorkerResult {
    Ok,
    Blocked,
    Terminated,
}

impl<'a> Worker<'a> {
    fn new(program: &'a [Instruction], id: i64) -> Self {
        Self {
            vm: VM::new(program, id),
            queue: VecDeque::new(),
            state: RunResult::Ok,
            n_sends: 0,
        }
    }
    fn run(&mut self, other: &mut Worker) -> WorkerResult {
        match self.state {
            RunResult::Halt => {return WorkerResult::Terminated;},
            RunResult::Rcv(reg) => {
                if let Some(val) = self.queue.pop_front() {
                    self.vm.set_reg(reg, val);
                }
                else {
                    return WorkerResult::Blocked;
                }
            },
            RunResult::Snd(_) => panic!(),
            RunResult::Ok => {},
        }
        loop {
            self.state = self.vm.run();
            match self.state {
                RunResult::Halt => {return WorkerResult::Terminated;},
                RunResult::Rcv(reg) => {
                    if let Some(val) = self.queue.pop_front() {
                        self.vm.set_reg(reg, val);
                        continue;
                    }
                    return WorkerResult::Blocked;
                },
                RunResult::Snd(val) => {
                    other.queue.push_back(val);
                    self.n_sends += 1;
                    self.state = RunResult::Ok;
                    return WorkerResult::Ok;
                }
                RunResult::Ok => panic!(),
            }
        }
    }
    fn is_blocked(&self) -> bool {
        match self.state {
            RunResult::Halt => true,
            RunResult::Rcv(_) => self.queue.is_empty(),
            _ => false,
        }
    }
}

fn part2(input: &[Instruction]) -> i64 {
    let mut vm0 = Worker::new(input, 0);
    let mut vm1 = Worker::new(input, 1);
    while !vm0.is_blocked() || !vm1.is_blocked() {
        vm0.run(&mut vm1);
        vm1.run(&mut vm0);
    }
    vm1.n_sends
}

fn main() {
    let input: Vec<Instruction> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day18_test() {
        let input: Vec<Instruction> = test_input(include_str!("day18.testinput"));
        assert_eq!(part1(&input), 4);
        let input: Vec<Instruction> = test_input(include_str!("day18.testinput2"));
        assert_eq!(part2(&input), 3);
    }
}
