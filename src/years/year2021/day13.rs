use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day13;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dot {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FoldDirection {
    X,
    Y,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fold {
    dir: FoldDirection,
    location: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day13 {
    dots: Vec<Dot>,
    folds: Vec<Fold>,
}
impl Day13 {
    fn apply_fold(&mut self, fold: Fold) {
        // we apply a fold by mirroring the dots in the fold direction
        // that are past the fold location
        // we first have the dots before the fold
        let (unfolded, folded): (Vec<Dot>, Vec<Dot>) =
            self.dots.iter().partition(|&dot| match fold.dir {
                FoldDirection::X => dot.x < fold.location,
                FoldDirection::Y => dot.y < fold.location,
            });
        // then we mirror the dots in the fold direction
        let folded: Vec<Dot> = folded
            .iter()
            .map(|dot| match fold.dir {
                FoldDirection::X => Dot {
                    x: 2 * fold.location - dot.x,
                    y: dot.y,
                },
                FoldDirection::Y => Dot {
                    x: dot.x,
                    y: 2 * fold.location - dot.y, // because y = F.y - (y - F.y)
                },
            })
            .collect();
        // then we concatenate the two
        self.dots = unfolded.into_iter().chain(folded).unique().collect();
    }

    fn print_dot_grid(&self) {
        let min_x = self.dots.iter().map(|dot| dot.x).min().unwrap();
        let max_x = self.dots.iter().map(|dot| dot.x).max().unwrap();
        let min_y = self.dots.iter().map(|dot| dot.y).min().unwrap();
        let max_y = self.dots.iter().map(|dot| dot.y).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dot = Dot { x, y };
                if self.dots.contains(&dot) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl AOCDay for Day {
    const DAY: usize = 13;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        let fold = self.folds[0];
        self.apply_fold(fold);
        self.dots.len()
    }
    fn part_two(&mut self) -> Self::Output {
        let folds = self.folds.clone();
        for fold in folds {
            self.apply_fold(fold);
        }
        self.print_dot_grid();
        0
    }
}
// ####..##..####.#..#.#....#..#.####.####
// #....#..#.#....#..#.#....#..#....#.#...
// ###..#....###..####.#....####...#..###.
// #....#....#....#..#.#....#..#..#...#...
// #....#..#.#....#..#.#....#..#.#....#...
// ####..##..#....#..#.####.#..#.####.#...

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let mut lines = s.lines();
        let dots = lines
            .take_while_ref(|line| line.split(",").count() == 2)
            .map(|line| {
                let mut parts = line.split(",");
                let x = parts.next().unwrap().parse()?;
                let y = parts.next().unwrap().parse()?;
                Ok(Dot { x, y })
            })
            .collect::<AResult<Vec<Dot>>>()?;
        lines.next();
        let folds = lines
            .map(|line| {
                let word = line.split_ascii_whitespace().last().unwrap();
                let parts = word.split("=").collect::<Vec<_>>();
                let dir = match parts[0] {
                    "x" => FoldDirection::X,
                    "y" => FoldDirection::Y,
                    _ => panic!("invalid fold direction"),
                };
                let location = parts[1].parse()?;
                Ok(Fold { dir, location })
            })
            .collect::<AResult<Vec<Fold>>>()?;

        Ok(Day13 { dots, folds })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_day_part_one, test_day_part_two};
    use super::*;
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
