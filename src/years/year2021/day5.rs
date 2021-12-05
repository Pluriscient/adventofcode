use super::util::AResult;
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

type Output = isize;
const DAY: usize = 5;

// mod a {

//     fn solve_part_one(input: Input) -> Output {
//         // only consider horizontal and vertical segments
//         let non_diagonal = input
//             .lines_segments
//             .iter()
//             .filter(|l| l.a.x == l.b.x || l.a.y == l.b.y)
//             .collect::<Vec<_>>();
//         let mut covered_points = HashSet::new();
//         let mut done = HashSet::new();
//         for l in non_diagonal {
//             for p in l.points() {
//                 if !done.contains(&p) {
//                     if covered_points.contains(&p) {
//                         done.insert(p);
//                     } else {
//                         covered_points.insert(p);
//                     }
//                 }
//             }
//         }
//         done.len() as Output
//     }

//     fn solve_part_two(input: Input) -> Output {
//         let mut covered_points = HashSet::new();
//         let mut done = HashSet::new();
//         for l in input.lines_segments.iter() {
//             for p in l.points() {
//                 if !done.contains(&p) {
//                     if covered_points.contains(&p) {
//                         done.insert(p);
//                     } else {
//                         covered_points.insert(p);
//                     }
//                 }
//             }
//         }
//         done.len() as Output
//     }
// }

use super::AOCDay;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day5 {
    lines_segments: Vec<Line>,
}

impl AOCDay for Day5 {
    const DAY: usize = 5;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        // only consider horizontal and vertical segments
        let non_diagonal = self
            .lines_segments
            .iter()
            .filter(|l| l.a.x == l.b.x || l.a.y == l.b.y)
            .collect::<Vec<_>>();
        let mut covered_points = HashSet::new();
        let mut done = HashSet::new();
        for l in non_diagonal {
            for p in l.points() {
                if !done.contains(&p) {
                    if covered_points.contains(&p) {
                        done.insert(p);
                    } else {
                        covered_points.insert(p);
                    }
                }
            }
        }
        done.len() as Output
    }
    fn part_two(&mut self) -> Self::Output {
        let mut covered_points = HashSet::new();
        let mut done = HashSet::new();
        for l in self.lines_segments.iter() {
            for p in l.points() {
                if !done.contains(&p) {
                    if covered_points.contains(&p) {
                        done.insert(p);
                    } else {
                        covered_points.insert(p);
                    }
                }
            }
        }
        done.len() as Output
    }
}
impl FromStr for Day5 {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let lines_segments = s
            .lines()
            .map(|l| Line::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { lines_segments })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Line {
    a: Point,
    b: Point,
}
impl Line {
    /// returns a vector of points that this line passes through
    /// this is a bit of a hack, but it works
    /// Only works for horizontal and vertical lines
    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let mut x = self.a.x;
        let mut y = self.a.y;
        while x != self.b.x || y != self.b.y {
            points.push(Point { x, y });
            match self.a.x.cmp(&self.b.x) {
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Less => x += 1,
                std::cmp::Ordering::Greater => x -= 1,
            }
            match self.a.y.cmp(&self.b.y) {
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Less => y += 1,
                std::cmp::Ordering::Greater => y -= 1,
            }
        }
        points.push(Point { x, y });
        // println!("{:?}", points);
        points
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        // println!("parsing {}", s);
        let els = s
            .split("->")
            .map(|s| {
                s.trim()
                    .split(",")
                    .map(|x| isize::from_str(x))
                    .collect::<Result<Vec<isize>, _>>()
            })
            .collect::<Result<Vec<Vec<isize>>, _>>()?;
        let x1 = els[0][0];
        let y1 = els[0][1];
        let x2 = els[1][0];
        let y2 = els[1][1];
        let a = Point { x: x1, y: y1 };
        let b = Point { x: x2, y: y2 };
        Ok(Self { a, b })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_day_part_one, test_day_part_two};
    use super::*;
    type Day = Day5;
    #[test]
    fn part_one_test() -> Result<(), std::io::Error> {
        test_day_part_one::<Day>(true)
    }
    #[test]
    fn part_one() -> Result<(), std::io::Error> {
        test_day_part_one::<Day>(false)
    }
    #[test]
    fn part_two_test() -> Result<(), std::io::Error> {
        test_day_part_two::<Day>(true)
    }
    #[test]
    fn part_two() -> Result<(), std::io::Error> {
        test_day_part_two::<Day>(false)
    }
}
