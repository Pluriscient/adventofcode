use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day4;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day4 {
    drawn_numbers: Vec<isize>,
    boards: Vec<Board>,
}

impl AOCDay for Day {
    const DAY: usize = 4;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        for number in &self.drawn_numbers {
            for board in self.boards.iter_mut() {
                board.mark_number(*number);
                if board.check_complete() {
                    return board.get_checksum(*number);
                }
            }
        }
        panic!("No solution found");
    }
    fn part_two(&mut self) -> Self::Output {
        let mut last_board_score = 0;
        for number in &self.drawn_numbers {
            for board in self.boards.iter_mut() {
                if !board.done {
                    board.mark_number(*number);
                    if board.check_complete() {
                        last_board_score = board.get_checksum(*number);
                        board.done();
                    }
                }
            }
        }
        last_board_score
    }
}

impl FromStr for Day {
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

        Ok(Self {
            drawn_numbers,
            boards,
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
