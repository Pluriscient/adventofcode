use itertools::Itertools;

fn solve_part_one(measurements: &[i32]) -> usize {
    measurements
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn solve_part_two(measurements: &[i32], size: usize) -> usize {
    // we can simply steal from the previous assignment
    solve_part_one(
        &measurements
            .windows(size)
            .map(|window| window.iter().sum())
            .collect_vec(),
    )
}

#[cfg(test)]
mod test {
    use super::super::util::read_input;
    use super::{solve_part_one, solve_part_two};
    use std::io::Error;
    use std::str::FromStr;

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = read_input(1)?;
        let lines = input.split_ascii_whitespace();
        let measurements: Vec<i32> = lines
            .map(|line| i32::from_str(line.trim()))
            .collect::<Result<Vec<i32>, _>>()
            .unwrap();
        let solution = solve_part_one(&measurements);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = read_input(1)?;
        let lines = input.split_ascii_whitespace();
        let measurements: Vec<i32> = lines
            .map(|line| i32::from_str(line.trim()))
            .collect::<Result<Vec<i32>, _>>()
            .unwrap();
        let solution = solve_part_two(&measurements, 3);
        // let solution = solve_part_two(&measurements, 2);

        println!("solution part 2: {}", solution);
        Ok(())
    }
}
