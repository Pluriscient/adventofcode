fn solve_part_one(passes: Vec<PassWithPolicy>) -> usize {
    passes.iter().filter(|&pass| pass.is_valid_one()).count()
}

fn solve_part_two(passes: Vec<PassWithPolicy>) -> usize {
    passes.iter().filter(|&pass| pass.is_valid_two()).count()
}

impl<'a> PassWithPolicy<'a> {
    fn is_valid_one(&self) -> bool {
        let c_count = self.pass.chars().filter(|&c| c == self.c).count();
        c_count >= self.min && c_count <= self.max
    }

    fn is_valid_two(&self) -> bool {
        let chars = self.pass.as_bytes();
        (chars[self.min - 1] == self.c as u8) ^ (chars[self.max - 1] == self.c as u8)
    }
}

#[derive(Debug)]
struct PassWithPolicy<'a> {
    c: char,
    min: usize,
    max: usize,
    pass: &'a str,
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;
    use std::str::FromStr;


    fn parse_line(line: &str) -> PassWithPolicy {
        let parts = line.split(":").collect::<Vec<_>>();
        let rule = parts[0].trim().split_ascii_whitespace().collect::<Vec<_>>();
        let minmax = rule[0].split("-").map(|x| usize::from_str(x).unwrap()).collect::<Vec<_>>();
        PassWithPolicy {
            c: rule[1].chars().next().unwrap(),
            min: minmax[0],
            max: minmax[1],
            pass: parts[1].trim(),
        }
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = read_to_string("inputs/day2.txt")?;

        // todo transform lines to our input somehow
        let passes = input.lines().map(|l| parse_line(l)).collect::<Vec<_>>();
        // println!("passes: {:?}", passes);
        let solution = solve_part_one(passes);
        println!("[day2] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = read_to_string("inputs/day2.txt")?;
        let passes = input.lines().map(|l| parse_line(l)).collect::<Vec<_>>();
        let solution = solve_part_two(passes);
        // println!("[day2] solution part 1: {}", solution);
        println!("[day2] solution part 2: {}", solution);
        Ok(())
    }
}