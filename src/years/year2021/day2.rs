use super::util::AResult;
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

fn solve_part_one(steps: &[Step]) -> isize {
    let final_position = steps
        .iter()
        .fold((0, 0), |pos, step| step.apply_step_a(pos));
    final_position.0 * final_position.1
}

fn solve_part_two(steps: &[Step]) -> isize {
    let final_position = steps
        .iter()
        .fold((0, 0, 0), |pos, step| step.apply_step_b(pos));
    final_position.0 * final_position.1
}
enum Direction {
    Forward,
    Up,
    Down,
}

struct Step {
    size: isize,
    direction: Direction,
}
impl Step {
    fn apply_step_a(&self, position: (isize, isize)) -> (isize, isize) {
        match self.direction {
            Direction::Forward => (position.0 + self.size, position.1),
            Direction::Up => (position.0, position.1 - self.size),
            Direction::Down => (position.0, position.1 + self.size),
        }
    }
    fn apply_step_b(&self, position: (isize, isize, isize)) -> (isize, isize, isize) {
        match self.direction {
            Direction::Forward => (
                position.0 + self.size,
                position.1 + self.size * position.2,
                position.2,
            ),
            Direction::Up => (position.0, position.1, position.2 - self.size),
            Direction::Down => (position.0, position.1, position.2 + self.size),
        }
    }
}

impl FromStr for Step {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> AResult<Self> {
        let words: Vec<&str> = s.split_whitespace().collect_vec();
        let size = words[1].parse::<isize>()?;
        let direction = match words[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            other => return Err(format!("Invalid direction {}", other).into()),
        };
        Ok(Self { size, direction })
    }
}

#[cfg(test)]
mod test {
    use super::super::util::read_input;
    use super::*;
    use super::{solve_part_one, solve_part_two};
    use std::io::Error;
    use std::str::FromStr;

    fn parse_input() -> std::result::Result<Vec<Step>, Error> {
        let input = read_input(2)?;
        Ok(input
            .trim()
            .lines()
            .map(Step::from_str)
            .collect::<AResult<Vec<Step>>>()
            .unwrap())
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_one(&input);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = parse_input()?;
        let solution = solve_part_two(&input);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}
