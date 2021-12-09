use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;
type Day = Day8;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day8 {
    string_literals: Vec<String>,
}

impl Day {
    fn parse_string_literal(input: &str) -> AResult<String> {
        // let input = input.trim_matches('"');
        let input = &input[1..input.len() - 1]; // trim matches was the killer
        let mut chars = input.chars();
        let mut result = String::new();
        let mut escaped = false;
        while let Some(c) = chars.next() {
            if escaped {
                match c {
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    'x' => {
                        let hex = [chars.next().unwrap(), chars.next().unwrap()]
                            .iter()
                            .collect::<String>();
                        let hex = u8::from_str_radix(hex.as_str(), 16).unwrap();
                        // let hex = 71u8;
                        result.push(hex as char);
                    }
                    _ => {
                        return Err(format!("Invalid escape sequence: {}", c).into());
                    }
                }
                escaped = false;
            } else {
                match c {
                    '\\' => escaped = true,
                    '"' => break,
                    _ => result.push(c),
                }
            }
        }
        assert_eq!(chars.next(), None);
        result.shrink_to_fit();
        Ok(result)
    }

    fn unparse_string_literal(input: &str) -> AResult<String> {
        let mut result = String::new();
        result.push('"');
        for c in input.chars() {
            match c {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                // 'x' => result.push_str("\\x"),
                _ => result.push(c),
            }
        }
        result.push('"');
        result.shrink_to_fit();
        Ok(result)
    }
    fn count_chars(s: &str) -> usize {
        s.as_bytes().len()
    }
    fn count_code_chars(s: &str) -> usize {
        let after = Self::parse_string_literal(s).unwrap();
        after.chars().count()
    }
}

impl AOCDay for Day {
    const DAY: usize = 8;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        self.string_literals
            .iter()
            .map(|s| (Day::count_chars(s), Day::count_code_chars(s)))
            .map(|(chars, code_chars)| chars - code_chars)
            .sum()
    }
    fn part_two(&mut self) -> Self::Output {
        self.string_literals
            .iter()
            .map(|s| {
                (
                    Day::count_chars(s),
                    Day::unparse_string_literal(s).unwrap().chars().count(),
                )
            })
            .map(|(chars, code_chars)| code_chars - chars)
            .sum()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        Ok(Self {
            string_literals: s
                .lines()
                .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect::<String>())
                .collect(),
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
        //answer is between 1288 and 1355 (exclusive)
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
