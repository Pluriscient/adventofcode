use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day10;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day10 {
    lines: Vec<String>,
}

impl Day {
    /// Checks the syntax of the given line.
    fn check_syntax(line: &str) -> Result<Vec<char>, char> {
        let mut chars = line.chars();
        let mut queue = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                '(' | '[' | '<' | '{' => queue.push(c),
                ')' | ']' | '>' | '}' => match queue.pop() {
                    Some(c2) => {
                        if c != match c2 {
                            '(' => ')',
                            '[' => ']',
                            '<' => '>',
                            '{' => '}',
                            _ => unreachable!(),
                        } {
                            return Err(c);
                        }
                    }
                    None => return Err(c),
                },
                _ => unreachable!(),
            }
        }
        Ok(queue)
    }
}

impl AOCDay for Day {
    const DAY: usize = 10;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        self.lines
            .iter()
            .filter_map(|line| Self::check_syntax(line.as_str()).err())
            .map(|c| match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            })
            .sum()
    }
    fn part_two(&mut self) -> Self::Output {
        let mut scores = self
            .lines
            .iter()
            .filter_map(|line| Self::check_syntax(line.as_str()).ok())
            .map(|queue| {
                queue
                    .iter()
                    .map(|c| match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    })
                    .rev()
                    .fold(0, |acc, p| acc * 5 + p)
            })
            .collect::<Vec<_>>();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        Ok(Self {
            lines: s.lines().map(|l| l.trim().to_string()).collect(),
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
