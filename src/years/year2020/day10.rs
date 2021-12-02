use itertools::Itertools;

fn solve_part_one(mut jolts: Vec<usize>) -> usize {
    let mut onejolts = 0;
    let mut threejolts = 0;
    jolts.sort_unstable();
    jolts.insert(0, 0);
    jolts.push(jolts.iter().max().unwrap() + 3);
    for i in 0..jolts.len() - 1 {
        let (cur, next) = (jolts[i], jolts[i + 1]);
        let diff = next - cur;
        match diff {
            1 => onejolts += 1,
            2 => (),
            3 => threejolts += 1,
            e => panic!("Difference between consecutive elements was: {}", e)
        }
    }
    onejolts * threejolts
}


fn solve_part_two(mut jolts: Vec<usize>) -> usize {
    jolts.sort_unstable();
    jolts.insert(0, 0);
    jolts.push(jolts.iter().max().unwrap() + 3);
    // dynamic programming
    let mut res = (0..jolts.len()).map(|_| 1).collect_vec();
    for i in 1..jolts.len() {
        res[i] = res[i - 1];
        if i > 1 && jolts[i] - jolts[i - 2] <= 3 {
            res[i] += res[i - 2];
        }
        if i > 2 && jolts[i] - jolts[i - 3] <= 3 {
            res[i] += res[i - 3];
        }
    }
    *res.last().unwrap()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;
    use itertools::Itertools;
    use std::str::FromStr;

    fn parse_input() -> Result<Vec<usize>, Error> {
        let input = read_to_string("../../../inputs/year2020/day10.txt")?;
        let lines = input.lines();
        let jolts = lines.map(|l| usize::from_str(l).unwrap()).collect_vec();
        Ok(jolts)
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let jolts = parse_input()?;
        let solution = solve_part_one(jolts);
        println!("[day10] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let jolts = parse_input()?;
        let solution = solve_part_two(jolts);
        println!("[day10] solution part 2: {}", solution);
        Ok(())
    }
}