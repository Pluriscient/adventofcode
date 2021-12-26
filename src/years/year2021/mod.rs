mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

pub trait AOCDay {
    type Output;
    const DAY: usize;
    fn part_one(&mut self) -> Self::Output;
    fn part_two(&mut self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::util::{read_input, read_input_test};
    use super::AOCDay;
    use std::fmt::Debug;
    use std::io::Error;
    use std::str::FromStr;
    pub fn test_day_part_one<D>(test: bool) -> Result<(), Error>
    where
        D: AOCDay + FromStr + Debug,
        <D as AOCDay>::Output: Debug,
        <D as FromStr>::Err: Debug,
    {
        let text = if test {
            read_input_test(D::DAY)?
        } else {
            read_input(D::DAY)?
        };
        let mut day = D::from_str(text.trim()).expect("Failed to parse input");
        let result = day.part_one();
        println!(
            "[Day {}] Part one {}: {:?}",
            D::DAY,
            if test { "[test]" } else { "" },
            result
        );
        Ok(())
    }
    pub fn test_day_part_two<D>(test: bool) -> Result<(), Error>
    where
        D: AOCDay + FromStr + Debug,
        <D as AOCDay>::Output: Debug,
        <D as FromStr>::Err: Debug,
    {
        let text = if test {
            read_input_test(D::DAY)?
        } else {
            read_input(D::DAY)?
        };
        let mut day = D::from_str(text.trim()).expect("Failed to parse input");
        let result = day.part_two();
        println!(
            "[Day {}] Part two {}: {:?}",
            D::DAY,
            if test { "[test]" } else { "" },
            result
        );
        Ok(())
    }
}
