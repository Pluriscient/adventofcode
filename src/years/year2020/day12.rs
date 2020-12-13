use std::str::{Lines, FromStr};
use self::Direction::*;
use std::str;
use num_enum::FromPrimitive;

#[derive(Debug, Clone, FromPrimitive, Copy)]
#[repr(u8)]
enum Direction {
    #[num_enum(default)]
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

fn solve_part_one(instructions: Lines) -> usize {
    let mut position: (isize, isize) = (0, 0);
    let mut direction = East;
    let pos_move = |(x, y): (isize, isize), dir: Direction, val: isize| match dir {
        East => (x + val, y),
        North => (x, y - val),
        South => (x, y + val),
        West => (x - val, y)
    };
    for instruction in instructions {
        let bs = instruction.as_bytes();
        match (bs[0] as char, isize::from_str(str::from_utf8(&bs[1..]).unwrap()).unwrap()) {
            (c, degrees) if ['L', 'R'].contains(&c) => {
                let mut turns = (degrees / 90) % 4;
                if c == 'L' { turns = -turns; };
                let mut temp_dir = direction as i8 + turns as i8;
                if temp_dir < 0 {
                    temp_dir = 4 + temp_dir;
                }
                let new_dir: Direction = ((temp_dir % 4) as u8).into();
                direction = new_dir;
            }
            ('F', val) => position = pos_move(position, direction, val),
            (c, val) => position = pos_move(position, match c {
                'N' => North,
                'E' => East,
                'S' => South,
                'W' => West,
                _ => unimplemented!()
            }, val),
        }
    }
    (position.0.abs() + position.1.abs()) as usize
}

fn solve_part_two(instructions: Lines) -> usize {
    let mut position: (isize, isize) = (0, 0);
    let mut waypoint: (isize, isize) = (10, 1);
    let pos_move = |(x, y): (isize, isize), dir: Direction, val: isize| match dir {
        East => (x + val, y),
        North => (x, y + val),
        South => (x, y - val),
        West => (x - val, y)
    };
    let rotate = |(x, y): (isize, isize), turns: isize| match turns {
        0 => (x, y),
        1 => (y, -x),
        2 => (-x, -y),
        3 => (-y, x),
        _ => unreachable!()
    };
    // we could do the below as a fold, though I don't think it'd add much clarity
    for instruction in instructions {
        let bs = instruction.as_bytes();
        match (bs[0] as char, isize::from_str(str::from_utf8(&bs[1..]).unwrap()).unwrap()) {
            (c, degrees) if ['L', 'R'].contains(&c) => {
                let mut turns = (degrees / 90) % 4;
                if c == 'L' { turns = 4 - turns; };
                waypoint = rotate(waypoint, turns);
            }
            ('F', val) => position = (position.0 + waypoint.0 * val, position.1 + waypoint.1 * val),
            (c, val) => waypoint = pos_move(waypoint, match c {
                'N' => North,
                'E' => East,
                'S' => South,
                'W' => West,
                _ => unimplemented!()
            }, val),
        }
    }
    (position.0.abs() + position.1.abs()) as usize
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = read_to_string("inputs/day12.txt")?;
        let lines = input.lines();
        // todo transform lines to our input somehow
        let instructions = lines;
        let solution = solve_part_one(instructions);
        println!("[day12] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = read_to_string("inputs/day12.txt")?;
        let lines = input.lines();
        // todo transform lines to our input somehow
        let instructions = lines;
        let solution = solve_part_two(instructions);
        println!("[day12] solution part 2: {}", solution);
        Ok(())
    }
}