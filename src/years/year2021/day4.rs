use super::util::AResult;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Output = isize;
const DAY: usize = 4;

fn solve_part_one(mut input: Input) -> Output {
    for number in input.drawn_numbers {
        for board in input.boards.iter_mut() {
            board.mark_number(number);
            if board.check_complete() {
                return board.get_checksum(number);
            }
        }
    }
    panic!("No solution found");
}

fn solve_part_two(mut input: Input) -> Output {
    let mut last_board_score = 0;
    for number in input.drawn_numbers {
        for board in input.boards.iter_mut() {
            if !board.done {
                board.mark_number(number);
                if board.check_complete() {
                    last_board_score = board.get_checksum(number);
                    board.done();
                }
            }
        }
    }
    last_board_score
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Board {
    rows: Vec<Vec<(isize, bool)>>,
    done: bool,
}
impl Board {
    fn check_complete(&self) -> bool {
        let mut done = false;
        // row-wise check
        done |= self
            .rows
            .iter()
            .any(|row| row.iter().all(|(_, complete)| *complete));
        // column-wise check
        for col in 0..self.rows[0].len() {
            done |= self.rows.iter().all(|row| row[col].1);
        }
        done
    }
    fn done(&mut self) {
        self.done = true;
    }

    fn mark_number(&mut self, number: isize) -> bool {
        let mut marked = false;
        for row in &mut self.rows {
            for (n, b) in row.iter_mut() {
                if *n == number {
                    *b = true;
                    marked = true;
                }
            }
        }
        return marked;
    }

    fn get_checksum(&self, last_called: isize) -> isize {
        let unmarked_sum: isize = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|(_, b)| !*b)
                    .map(|(i, _)| i)
                    .sum::<isize>()
            })
            .sum();
        unmarked_sum * last_called
    }
}
impl FromStr for Board {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let rows = s
            .lines()
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|s| s.parse::<isize>().unwrap())
                    .map(|n| (n, false))
                    .collect::<Vec<(isize, bool)>>()
            })
            .collect::<Vec<Vec<(isize, bool)>>>();
        Ok(Board { rows, done: false })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    drawn_numbers: Vec<isize>,
    boards: Vec<Board>,
}
impl Input {}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> AResult<Self> {
        let mut lines = s.lines();
        let drawn_numbers: Vec<isize> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        let lines = lines.filter(|l| !l.is_empty()).collect::<Vec<&str>>();
        let boards = lines
            .chunks(5)
            .map(|chunk| Board::from_str(&chunk.join("\n")).unwrap())
            .collect::<Vec<Board>>();

        Ok(Input {
            drawn_numbers,
            boards,
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::super::util::{read_input, read_input_test};
    use super::*;
    use super::{solve_part_one, solve_part_two};
    use std::io::Error;
    use std::str::FromStr;

    fn parse_input() -> std::result::Result<Input, Error> {
        let input = read_input(super::DAY)?;
        Ok(Input::from_str(input.trim()).expect("Could not parse input"))
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_one(input);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_two(input);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}
