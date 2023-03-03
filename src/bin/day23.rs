use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::math::is_prime;

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
    Set(char, RI),
    Sub(char, RI),
    Mul(char, RI),
    Jnz(RI, RI),
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
            "set" => Ok(Instruction::Set(x, b.unwrap())),
            "sub" => Ok(Instruction::Sub(x, b.unwrap())),
            "mul" => Ok(Instruction::Mul(x, b.unwrap())),
            "jnz" => Ok(Instruction::Jnz(a, b.unwrap())),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum RunResult {
    Ok,
    Halt,
    Break,
}

struct VM<'a> {
    registers: HashMap<char, i64>,
    program: &'a Vec<Instruction>,
    pc: i64,
    trap: Option<&'a mut dyn FnMut(&'a Instruction, &HashMap<char, i64>) -> bool>,
}
impl<'a> VM<'a> {
    fn new(program: &'a Vec<Instruction>, id: i64) -> Self {
        let registers = HashMap::from_iter([('p', id)]);
        Self { registers, program, pc: 0, trap: None }
    }
    fn set_trap(&mut self, trap: &'a mut dyn FnMut(&'a Instruction, &HashMap<char, i64>) -> bool) {
        self.trap = Some(trap);
    }
    fn step(&mut self) -> RunResult {
        if self.pc < 0 || self.pc >= self.program.len() as i64 {
            return RunResult::Halt;
        }
        let inst = &self.program[self.pc as usize];
        match self.trap.as_mut() {
            Some(trap) => {
                if (*trap)(&inst, &self.registers) {
                    return RunResult::Break;
                }
            },
            None => {},
        }
        match inst {
            Instruction::Set(x, y) => {
                *self.registers.entry(*x).or_insert(0) = self.resolve(*y);
            }
            Instruction::Sub(x, y) => {
                *self.registers.entry(*x).or_insert(0) -= self.resolve(*y);
            }
            Instruction::Mul(x, y) => {
                *self.registers.entry(*x).or_insert(0) *= self.resolve(*y);
            }
            Instruction::Jnz(x, y) => {
                if self.resolve(*x) != 0 {
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

fn part1(input: &Vec<Instruction>) -> usize {
    let mut vm = VM::new(input, 0);
    let mut mul_count = 0;
    let mut trap = |inst, _:&HashMap<char, i64>| match inst {
            &Instruction::Mul(_,_) => {mul_count += 1; false},
            _ => false,
        };
    vm.set_trap(&mut trap);
    vm.run();
    mul_count
}

fn part2(input: &Vec<Instruction>) -> usize {
    let mut vm = VM::new(input, 0);

    // break when it gets to the "set f 1" instruction, at
    // which point the b and c registers contain the range to
    // search
    let mut trap = |inst, _:&HashMap<char, i64>| match inst {
            &Instruction::Set(r,_) if r == 'f' => true,
            _ => false,
        };
    vm.set_reg('a', 1);
    vm.set_trap(&mut trap);
    vm.run();

    let from = vm.get_reg('b');
    let to = vm.get_reg('c');

    // count the non-primes from b to c, inclusive
    (from..=to).step_by(17).filter(|n| !is_prime(*n as u64)).count()
}

fn main() {
    let input: Vec<Instruction> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
