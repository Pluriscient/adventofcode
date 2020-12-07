fn solve_part_one(landscape: Vec<Vec<bool>>) -> usize {
    solve_slope(&landscape, 3, 1)
}

fn solve_part_two(landscape: Vec<Vec<bool>>) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|(right, down)| solve_slope(&landscape, *right, *down))
        .product()
}

fn solve_slope(landscape: &Vec<Vec<bool>>, right: usize, down: usize) -> usize {
    let mut cur = right;
    let mut res = 0;
    let line_length = landscape[0].len();
    for line in landscape.iter().skip(down).step_by(down) {
        if line[cur] {
            res += 1;
        }
        cur += right;

        if cur >= line_length {
            cur -= line_length;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;

    use super::*;

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = read_to_string("inputs/day3.txt")?;
        let lines = input.lines();
        // todo transform lines to our input somehow
        let landscape = lines
            .map(|line| line.trim().chars().map(|c| c == '#')
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        // println!("{:?}", landscape);
        let solution = solve_part_one(landscape);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = read_to_string("inputs/day3.txt")?;
        let lines = input.lines();
        let landscape = lines
            .map(|line| line.trim().chars().map(|c| c == '#')
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let solution = solve_part_two(landscape);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}