fn solve_part_one(schedule: Vec<Option<isize>>, departure: isize) -> isize {
    schedule.iter()
        .filter_map(|x| *x)
        .map(|x| (x, x - (departure % x)))
        // .inspect(|(x, y)| println!("Earliest bus for {} is {} minutes (at {})", x, y, departure + y))
        .min_by_key(|(_, y)| *y)
        .map(|(x, y)| x * y).unwrap()
}

///copied from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: isize, n: isize) -> Option<isize> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}


/// Basically the Chinese Remainder Theorem
fn solve_part_two(schedule: Vec<Option<isize>>, _: isize) -> isize {
    let prod = schedule.iter().filter_map(|x| *x).product::<isize>();

    let (res, _) = schedule.iter().fold((0, 0), |(res, t_val), y|
        match y {
            None => (res, t_val + 1),
            Some(z) => {
                let z_ = *z;
                let p = prod / z_;
                let residue = (z_ - t_val) % z_;
                let n = residue * mod_inv(p, z_).unwrap() * p;
                return (res + n, t_val + 1);
            }
        },
    );
    res % prod
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use super::*;
    use std::str::FromStr;
    use itertools::Itertools;
    use std::error::Error;


    fn parse_input() -> Result<(Vec<Option<isize>>, isize), Box<dyn Error>> {
        let input = read_to_string("inputs/day13.txt")?;
        let mut lines = input.lines();
        let departure_time = isize::from_str(lines.next().unwrap())?;
        let schedule = lines.next().unwrap().split(",")
            .map(|l| l.parse::<isize>().ok()).collect_vec();

        Ok((schedule, departure_time))
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let (schedule, departure) = parse_input()?;
        let solution = solve_part_one(schedule, departure);
        println!("[day13] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let (schedule, departure) = parse_input()?;
        let solution = solve_part_two(schedule, departure);
        println!("[day13] solution part 2: {}", solution);
        Ok(())
    }
}