use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use rayon::prelude::*;
use std::error::Error;
use std::str::FromStr;
type Day = Day24;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Register {
    X,
    Y,
    W,
    Z,
}

impl FromStr for Register {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "w" => Ok(Register::W),
            "z" => Ok(Register::Z),
            _ => Err(From::from(format!("Invalid register: {}", s))),
        }
    }
}
fn digits(n: isize) -> Vec<isize> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 0 {
        digits.insert(0, n % 10);
        n /= 10;
    }
    digits
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Instruction {
    MonoOperation(Opcode, Register),
    Operation(Opcode, Register, Value),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        let opcode: Opcode = words[0].parse()?;
        if matches!(opcode, Opcode::Inp) {
            Ok(Self::MonoOperation(opcode, words[1].parse()?))
        } else {
            Ok(Self::Operation(
                opcode,
                words[1].parse()?,
                words[2].parse()?,
            ))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Value {
    Register(Register),
    Immediate(isize),
}

impl FromStr for Value {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s[0..1].parse::<Register>().is_ok() {
            Ok(Value::Register(s.parse()?))
        } else {
            Ok(Value::Immediate(s.parse()?))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Opcode {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl FromStr for Opcode {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inp" => Ok(Opcode::Inp),
            "add" => Ok(Opcode::Add),
            "mul" => Ok(Opcode::Mul),
            "div" => Ok(Opcode::Div),
            "mod" => Ok(Opcode::Mod),
            "eql" => Ok(Opcode::Eql),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid opcode",
            ))),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day24 {
    instructions: Vec<Instruction>,
    input: Vec<isize>,
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl Day {
    fn execute(&mut self) {
        let instrs = self.instructions.clone();
        for instruction in instrs.iter() {
            match instruction {
                Instruction::MonoOperation(opcode, register) => {
                    assert_eq!(*opcode, Opcode::Inp);
                    self.read_input(*register);
                }
                Instruction::Operation(opcode, register, value) => match opcode {
                    Opcode::Add => {
                        self.set_register(
                            *register,
                            self.get_register(*register) + self.get_value(*value),
                        );
                    }
                    Opcode::Mul => {
                        self.set_register(
                            *register,
                            self.get_register(*register) * self.get_value(*value),
                        );
                    }
                    Opcode::Div => {
                        self.set_register(
                            *register,
                            self.get_register(*register) / self.get_value(*value),
                        );
                    }
                    Opcode::Mod => {
                        self.set_register(
                            *register,
                            self.get_register(*register) % self.get_value(*value),
                        );
                    }
                    Opcode::Eql => {
                        self.set_register(
                            *register,
                            if self.get_register(*register) == self.get_value(*value) {
                                1
                            } else {
                                0
                            },
                        );
                    }
                    _ => panic!("Invalid opcode"),
                },
            }
        }
    }

    fn get_value(&self, value: Value) -> isize {
        match value {
            Value::Register(register) => self.get_register(register),
            Value::Immediate(value) => value,
        }
    }

    fn get_register(&self, register: Register) -> isize {
        match register {
            Register::X => self.x,
            Register::Y => self.y,
            Register::W => self.w,
            Register::Z => self.z,
        }
    }

    fn set_register(&mut self, register: Register, value: isize) {
        match register {
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::W => self.w = value,
            Register::Z => self.z = value,
        }
    }

    fn reset(&mut self, input: Vec<isize>) {
        self.x = 0;
        self.y = 0;
        self.z = 0;
        self.w = 0;
        self.input = input;
    }

    fn read_input(&mut self, register: Register) {
        if self.input.is_empty() {
            panic!("No input available");
        }
        match register {
            Register::X => self.x = self.input.remove(0),
            Register::Y => self.y = self.input.remove(0),
            Register::W => self.w = self.input.remove(0),
            Register::Z => self.z = self.input.remove(0),
        }
    }
}

impl AOCDay for Day {
    const DAY: usize = 24;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        for i in (1..=99999999999999).rev() {
            println!("testing {}", i);
            let input = digits(i);
            // println!("input: {:?}", input);
            if input.contains(&0) {
                continue;
            }
            if input.len() != 14 {
                panic!("Done already");
            }
            self.reset(input);
            // self.x = i;
            self.execute();
            if self.z == 0 {
                return i;
            }
        }
        unreachable!()
    }
    fn part_two(&mut self) -> Self::Output {
        todo!()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let instructions = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Day24 {
            input: vec![100, 300],
            instructions,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_day_part_one, test_day_part_two};
    use super::*;
    #[test]
    fn part_one_test() -> Result<(), std::io::Error> {
        test_day_part_one::<Day>(true)
    }
    #[test]
    fn part_one() -> Result<(), std::io::Error> {
        test_day_part_one::<Day>(false)
    }
    #[test]
    fn part_two_test() -> Result<(), std::io::Error> {
        test_day_part_two::<Day>(true)
    }
    #[test]
    fn part_two() -> Result<(), std::io::Error> {
        test_day_part_two::<Day>(false)
    }
}
