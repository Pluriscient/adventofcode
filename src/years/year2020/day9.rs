use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

fn solve_part_one(encoding: &Vec<usize>) -> usize {
    let preamble = 25;
    for i in preamble..encoding.len() {
        let pre = &encoding[i - preamble..i];
        let search = &encoding[i];
        if find_pair(*search, pre).is_err() {
            return *search;
        }
    }
    unreachable!("Did not find a solution for the given input");
}

fn find_pair(num: usize, preamble: &[usize]) -> Result<(usize, usize), ()> {
    for i in 0..preamble.len() - 1 {
        for j in i + 1..preamble.len() {
            if (preamble[i] + preamble[j]) == num {
                return Ok((i, j));
            }
        }
    }
    Err(())
}


fn solve_part_two(encoding: Vec<usize>) -> usize {
    let invalid_num = solve_part_one(&encoding);
    // now we need to find a set of at least two numbers that adds up to invalid_num
    for i in 0..encoding.len() {
        let mut cur_val = encoding[i];
        if cur_val >= invalid_num {
            continue;
        }
        let mut j = i + 1;
        while cur_val < invalid_num {
            cur_val += encoding[j];
            j += 1;
        }
        if cur_val == invalid_num {
            if let MinMax(min, max) = &encoding[i..j].iter().minmax() {
                return *min + *max;
            }
        }
    }
    unreachable!("Did not find a solution!");
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;
    use std::str::FromStr;
    use itertools::Itertools;


    fn parse_input() -> Result<Vec<usize>, Error> {
        let input = read_to_string("inputs/day9.txt")?;
        let lines = input.lines();
        let encoding = lines.map(|l| usize::from_str(l).unwrap()).collect_vec();
        Ok(encoding)
    }


    #[test]
    fn test_part_one() -> Result<(), Error> {
        let encoding = parse_input();
        let solution = solve_part_one(&encoding?);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let encoding = parse_input();
        let solution = solve_part_two(encoding?);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}