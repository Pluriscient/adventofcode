use itertools::{Itertools};

use std::str::FromStr;
use self::Instruction::*;

fn solve_part_one(instructions: Vec<Instruction>) -> isize {
    Instruction::terminates(&instructions)
        .expect_err("Programing terminated without looping")
}

fn solve_part_two(mut instructions: Vec<Instruction>) -> isize {
    for i in 0..instructions.len() {
        let instruction = instructions[i];
        if let ACC(_) = instruction {
            continue;
        }
        let temp = instructions[i].clone();
        let new_instr = match instruction {
            JMP(v) => NOP(v),
            NOP(v) => JMP(v),
            _ => unreachable!()
        };
        instructions[i] = new_instr;
        if let Ok(acc) = Instruction::terminates(&instructions) {
            return acc;
        }
        instructions[i] = temp;
    }
    panic!("Solution not found!");
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

impl Instruction {
    fn parse_line(line: &str) -> Self {
        let words = line.trim().split_ascii_whitespace().collect_vec();
        let val = isize::from_str(words[1]).unwrap();
        match words[0] {
            "nop" => Self::NOP(val),
            "acc" => Self::ACC(val),
            "jmp" => Self::JMP(val),
            e => panic!("{} was not an instruction", e)
        }
    }

    fn terminates(instructions: &Vec<Self>) -> Result<isize, isize> {
        let mut history: Vec<isize> = vec![];
        let mut ci = 0;
        let mut acc = 0;
        while !history.contains(&ci) && (ci as usize) < instructions.len() {
            history.push(ci);
            let current = &instructions[ci as usize];
            match current {
                Instruction::NOP(_) => ci += 1,
                Instruction::ACC(val) => {
                    acc += val;
                    ci += 1
                }
                Instruction::JMP(val) => {
                    ci += val
                }
            }
        }
        if ci as usize == instructions.len() {
            Ok(acc)
        } else {
            Err(acc)
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;
    use itertools::Itertools;

    fn parse_input() -> Result<Vec<Instruction>, Error> {
        let input = read_to_string("inputs/day8.txt")?;
        let lines = input.lines();
        Ok(lines.map(Instruction::parse_line).collect_vec())
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let instructions = parse_input()?;
        let solution = solve_part_one(instructions);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let instructions = parse_input()?;
        let solution = solve_part_two(instructions);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}