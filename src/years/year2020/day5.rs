fn solve_part_one(passes: Vec<BoardingPass>) -> usize {
    passes.iter().map(|pass| pass.id).max().unwrap()
}

fn solve_part_two(passes: Vec<BoardingPass>) -> usize {
    let mut ids: Vec<_> = passes.iter().map(|pass| pass.id).collect();
    ids.sort_unstable();
    ids.iter()
        .scan(ids[0], |prev, &x| {
            let diff = x - *prev;
            *prev = x;
            Some((diff, x))
        })
        .filter(|&tuple| tuple.0 > 1)
        .next().unwrap()
        .1 - 1
}


struct BoardingPass {
    row: usize,
    seat: usize,
    id: usize,
}

impl BoardingPass {
    fn read_from_line(line: &str) -> Self {
        let chars = line.as_bytes();
        let mut row = 0;
        let mut step = 64;
        for &c in &chars[..7] {
            match c as char {
                'F' => (),
                'B' => row += step,
                _ => panic!()
            }
            step /= 2;
        }
        // row -= 1;
        let mut seat = 0;
        step = 4;
        for &c in &chars[7..] {
            match c as char {
                'R' => seat += step,
                'L' => (),
                _ => panic!()
            }
            step /= 2;
        }
        // println!("{} corresponds to seat {}, {}", line, row, seat);
        let id = row * 8 + seat;
        Self {
            row,
            seat,
            id,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = read_to_string("../../../inputs/year2020/day5.txt")?;
        let lines = input.lines();
        let passes = lines
            .map(|line| BoardingPass::read_from_line(line.trim()))
            .collect();
        let solution = solve_part_one(passes);
        println!("[day5] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = read_to_string("../../../inputs/year2020/day5.txt")?;
        let lines = input.lines();
        // todo transform lines to our input somehow
        let passes = lines
            .map(|line| BoardingPass::read_from_line(line.trim()))
            .collect();
        let solution = solve_part_two(passes);
        println!("[day5] solution part 2: {}", solution);
        Ok(())
    }
}