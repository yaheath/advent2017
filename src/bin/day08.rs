use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
}
impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Op::Gt),
            ">=" => Ok(Op::Gte),
            "<" => Ok(Op::Lt),
            "<=" => Ok(Op::Lte),
            "==" => Ok(Op::Eq),
            "!=" => Ok(Op::Neq),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    register: String,
    incr: i64,
    condition_register: String,
    condition_op: Op,
    condition_value: i64,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\w+) (inc|dec) ([-0-9]+) if (\w+) ([<>=!]+) ([-0-9]+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let register = caps.get(1).unwrap().as_str().to_string();
            let inc_or_dec = caps.get(2).unwrap().as_str();
            let mut incr = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            if inc_or_dec == "dec" { incr = -incr; }
            let condition_register = caps.get(4).unwrap().as_str().to_string();
            let condition_op = caps.get(5).unwrap().as_str().parse::<Op>().unwrap();
            let condition_value = caps.get(6).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Input {
                register, incr, condition_register, condition_op, condition_value
            })
        }
        else {
            Err(())
        }
    }
}

struct VM {
    registers: HashMap<String, i64>,
    max_reg_value: i64,
}
impl VM {
    fn new() -> Self {
        Self { registers: HashMap::new(), max_reg_value: 0 }
    }
    fn get_register(&self, name: &str) -> i64 {
        if let Some(v) = self.registers.get(name) {
            *v
        } else {
            0
        }
    }
    fn execute_instruction(&mut self, inst: &Input) {
        let cr_val = self.get_register(&inst.condition_register);
        if match inst.condition_op {
            Op::Gt => cr_val > inst.condition_value,
            Op::Gte => cr_val >= inst.condition_value,
            Op::Lt => cr_val < inst.condition_value,
            Op::Lte => cr_val <= inst.condition_value,
            Op::Eq => cr_val == inst.condition_value,
            Op::Neq => cr_val != inst.condition_value,
        } {
            let new_val = self.get_register(&inst.register) + inst.incr;
            self.registers.insert(inst.register.clone(), new_val);
            self.max_reg_value = self.max_reg_value.max(new_val);
        }
    }
    fn run(&mut self, instructions: &[Input]) {
        for i in instructions {
            self.execute_instruction(i);
        }
    }
}

fn part1(input: &[Input]) -> i64 {
    let mut vm = VM::new();
    vm.run(input);
    *vm.registers.values().max().unwrap()
}
fn part2(input: &[Input]) -> i64 {
    let mut vm = VM::new();
    vm.run(input);
    vm.max_reg_value
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day08_test() {
        let input: Vec<Input> = test_input(include_str!("day08.testinput"));
        assert_eq!(part1(&input), 1);
        assert_eq!(part2(&input), 10);
    }
}
