use self::SeatOccupation::*;
use std::fmt::{Debug, Formatter, Display};
use std::fmt;

fn solve_part_one(mut seatmap: SeatMap) -> usize {
    loop {
        let next = seatmap.update();
        if next == seatmap {
            break;
        }
        seatmap = next;
    }
    seatmap.rows.iter().flat_map(|x| x.iter()).filter(|s| matches!(s, Occupied)).count()
}

fn solve_part_two(mut seatmap: SeatMap) -> usize {

    loop {
        let next = seatmap.update_two();
        if next == seatmap {
            break;
        }
        seatmap = next;
    }
    seatmap.rows.iter().flat_map(|x| x.iter()).filter(|s| matches!(s, Occupied)).count()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SeatOccupation {
    Floor,
    Empty,
    Occupied,
}

impl Display for SeatOccupation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Occupied => "#",
            Empty => "L",
            Floor => "."
        })
    }
}

impl SeatOccupation {
    fn from_char(c: char) -> Result<SeatOccupation, String> {
        match c {
            '#' => Ok(Occupied),
            'L' => Ok(Empty),
            '.' => Ok(Floor),
            _ => Err("could not parse char".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SeatMap {
    rows: Vec<Vec<SeatOccupation>>
}

impl Display for SeatMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        for row in &self.rows {
            for seat in row {
                string.push_str(format!("{}", seat).as_str());
            }
            string.push('\n');
        }
        f.write_str(string.as_str())
    }
}

impl SeatMap {
    fn adjacents(&self, i: i32, j: i32) -> Vec<SeatOccupation> {
        // generate the square around the seat
        let n = self.rows.len() as i32;
        let m = self.rows[0].len() as i32;
        let mut res = vec![];
        for x in i - 1..=i + 1 {
            for y in j - 1..=j + 1 {
                if (x == i && y == j) ||
                    (x < 0) || (y < 0) ||
                    (x >= n) || y >= m
                {
                    continue;
                }

                res.push(self.rows[x as usize][y as usize])
            }
        }
        res
    }

    fn visible(&self, i: i32, j: i32) -> Vec<SeatOccupation> {
        // generate the square around the seat
        let n = self.rows.len() as i32;
        let m = self.rows[0].len() as i32;
        let mut res = vec![];
        let directions = [(-1, 1), (-1, 0), (-1, -1), (0, 1), (0, -1), (1, 1), (1, 0), (1, -1)];
        let inbounds = |x, y| x >= 0 && x < n && y >= 0 && y < m;
        for (di, dj) in directions.iter() {
            let (mut ci, mut cj) = (i + di, j + dj);
            while inbounds(ci, cj) {
                match self.rows[ci as usize][cj as usize] {
                    Occupied => {
                        res.push(Occupied);
                        break;
                    }
                    Empty => {
                        res.push(Empty);
                        break;
                    }
                    _ => ()
                }
                ci += di;
                cj += dj;
            }
        }
        res
    }

    fn update_two(&self) -> Self {
        let n = self.rows.len();
        let m = self.rows[0].len();
        let mut res = self.clone();
        for i in 0..n {
            for j in 0..m {
                let seat = self.rows[i][j];
                if matches!(seat, Floor) {
                    continue;
                }
                let visible = self.visible(i as i32, j as i32);
                let occupied = visible.iter()
                    .filter(|x| matches!(x, Occupied)).count();
                res.rows[i][j] = match (seat, occupied) {
                    (Floor, _) => Floor,
                    (Empty, 0) => Occupied,
                    (Empty, _) => Empty,
                    (Occupied, x) if x >= 5 => Empty,
                    (Occupied, _) => Occupied
                };
            }
        }
        res
    }

    fn update(&self) -> Self {
        let n = self.rows.len();
        let m = self.rows[0].len();
        let mut res = self.clone();
        for i in 0..n {
            for j in 0..m {
                let seat = self.rows[i][j].clone();
                let adjacents = self.adjacents(i as i32, j as i32);
                let occupied = adjacents.iter()
                    .filter(|x| matches!(x, Occupied)).count();
                res.rows[i][j] = match (seat, occupied) {
                    (Floor, _) => Floor,
                    (Empty, 0) => Occupied,
                    (Empty, _) => Empty,
                    (Occupied, x) if x >= 4 => Empty,
                    (Occupied, _) => Occupied
                };
            }
        }
        res
    }
}


#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;
    use itertools::Itertools;

    fn parse_input() -> Result<SeatMap, Error> {
        let input = read_to_string("inputs/day11.txt")?;
        let lines = input.lines();
        let rows = lines.map(|l| l.trim().chars()
            .map(|c| SeatOccupation::from_char(c).unwrap()).collect_vec()).collect_vec();
        Ok(SeatMap {
            rows
        })
    }

    #[test]
    fn test_part_one() -> Result<(), Error> {
        let seatmap = parse_input()?;
        let solution = solve_part_one(seatmap);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let seatmap = parse_input()?;
        let solution = solve_part_two(seatmap);
        println!("solution part 2: {}", solution);
        Ok(())
    }
}