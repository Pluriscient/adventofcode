use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day22;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    turn_on: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl FromStr for Cube {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let mut words = s.split_whitespace();
        let turn_on = words.next().unwrap() == "on";
        let mut iter = words.next().unwrap().split(',');
        let x = iter.next().unwrap()[2..]
            .split("..")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let y = iter.next().unwrap()[2..]
            .split("..")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let z = iter.next().unwrap()[2..]
            .split("..")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Cube { turn_on, x, y, z })
    }
}
fn invalid_range(r: &std::ops::Range<i64>) -> bool {
    r.end - r.start < 0
}

impl Cube {
    fn size(&self) -> i64 {
        return (self.x.1 - self.x.0 + 1).abs()
            * (self.y.1 - self.y.0 + 1).abs()
            * (self.z.1 - self.z.0 + 1).abs();
    }

    fn elements(&self) -> Vec<(i64, i64, i64)> {
        let mut res = Vec::with_capacity(self.size() as usize);
        for xx in self.x.0..=self.x.1 {
            for yy in self.y.0..=self.y.1 {
                for zz in self.z.0..=self.z.1 {
                    res.push((xx, yy, zz));
                }
            }
        }
        res
    }

    fn non_overlapping_cubes(&self, other: &Cube) -> Vec<Cube> {
        let overlap = self.overlapping_cube(other);
        if overlap.is_none() {
            return vec![self.clone()];
        }
        let mut result = vec![];
        let overlap = overlap.unwrap();
        // consider this from the x dimension; potentially the overlap was in the middle
        let x_pre_cube = self.x.0..overlap.x.0 - 1;
        let y_pre_cube = self.y.0..(overlap.y.0 - 1);
        let z_pre_cube = self.z.0..(overlap.z.0 - 1);
        let x_post_cube = overlap.x.1 + 1..self.x.1;
        let y_post_cube = overlap.y.1 + 1..self.y.1;
        let z_post_cube = overlap.z.1 + 1..self.z.1;
        // now we need to add all the cuboids that can be made from this to the vec

        // let's start 2D because I can actually grasp that
        // we are going to get overlapping parts but that's just ripperoni until it becomes a larger problem
        for x in [x_pre_cube, x_post_cube] {
            if !invalid_range(&x) {
                result.push(Cube {
                    x: (x.start, x.end),
                    ..*self
                });
            }
        }
        for y in [y_pre_cube, y_post_cube] {
            if !invalid_range(&y) {
                result.push(Cube {
                    y: (y.start, y.end),
                    x: overlap.x,
                    ..*self
                });
            }
        }
        for z in [z_pre_cube, z_post_cube] {
            if !invalid_range(&z) {
                result.push(Cube {
                    z: (z.start, z.end),
                    x: overlap.x,
                    y: overlap.y,
                    ..*self
                });
            }
        }
        result
    }

    fn overlapping_cube(&self, other: &Cube) -> Option<Cube> {
        let x = self.x.0.max(other.x.0)..self.x.1.min(other.x.1);
        let y = self.y.0.max(other.y.0)..self.y.1.min(other.y.1);
        let z = self.z.0.max(other.z.0)..self.z.1.min(other.z.1);
        if invalid_range(&x) || invalid_range(&y) || invalid_range(&z) {
            return None;
        }
        let res = Some(Cube {
            turn_on: self.turn_on,
            x: (x.start, x.end),
            y: (y.start, y.end),
            z: (z.start, z.end),
        });
        res
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day22 {
    cube_instructions: Vec<Cube>,
}

impl AOCDay for Day {
    const DAY: usize = 22;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        let init_cube = Cube {
            turn_on: false,
            x: (-50, 50),
            y: (-50, 50),
            z: (-50, 50),
        };
        let mut on_cubes = vec![];
        for cuboid in self
            .cube_instructions
            .iter()
            .filter_map(|cuboid| cuboid.overlapping_cube(&init_cube))
        {
            if cuboid.turn_on {
                // we need to ensure we don't add overlapping parts as those are hard to calc with

                let remainder = on_cubes.iter().fold(vec![cuboid], |acc, other_cube| {
                    // we want to transform the list of cubes we had into a list of cubes
                    acc.iter()
                        .flat_map(|c| c.non_overlapping_cubes(other_cube))
                        .collect_vec()
                });
                on_cubes.extend(remainder);
            } else {
                // remove this cube from each of the cubes
                on_cubes = on_cubes
                    .iter()
                    .flat_map(|c| c.non_overlapping_cubes(&cuboid))
                    .collect_vec();
            }
        }
        on_cubes
            .iter()
            // .inspect(|x| println!("cube {:?} has size {}", x, x.size()))
            .map(|c| c.size())
            .sum::<i64>() as Self::Output
    }
    fn part_two(&mut self) -> Self::Output {
        let mut on_cubes = vec![];
        for cuboid in &self.cube_instructions {
            if cuboid.turn_on {
                // we need to ensure we don't add overlapping parts as those are hard to calc with

                let remainder = on_cubes
                    .iter()
                    .fold(vec![cuboid.clone()], |acc, other_cube| {
                        // we want to transform the list of cubes we had into a list of cubes
                        acc.iter()
                            .flat_map(|c| c.non_overlapping_cubes(other_cube))
                            .collect_vec()
                    });
                on_cubes.extend(remainder);
            } else {
                // remove this cube from each of the cubes
                on_cubes = on_cubes
                    .iter()
                    .flat_map(|c| c.non_overlapping_cubes(&cuboid))
                    .collect_vec();
            }
        }
        on_cubes
            .iter()
            // .inspect(|x| println!("cube {:?} has size {}", x, x.size()))
            .map(|c| c.size())
            .sum::<i64>() as Self::Output
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let cube_instructions = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { cube_instructions })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_day_part_one, test_day_part_two};
    use super::*;

    #[test]
    fn elements_test() {
        let cube = Cube {
            x: (10, 12),
            y: (10, 12),
            z: (10, 12),
            turn_on: true,
        };
        let expected = [
            (10, 10, 10),
            (10, 10, 11),
            (10, 10, 12),
            (10, 11, 10),
            (10, 11, 11),
            (10, 11, 12),
            (10, 12, 10),
            (10, 12, 11),
            (10, 12, 12),
            (11, 10, 10),
            (11, 10, 11),
            (11, 10, 12),
            (11, 11, 10),
            (11, 11, 11),
            (11, 11, 12),
            (11, 12, 10),
            (11, 12, 11),
            (11, 12, 12),
            (12, 10, 10),
            (12, 10, 11),
            (12, 10, 12),
            (12, 11, 10),
            (12, 11, 11),
            (12, 11, 12),
            (12, 12, 10),
            (12, 12, 11),
            (12, 12, 12),
        ];
        assert_eq!(expected[..], cube.elements()[..]);
    }

    #[test]
    fn overlapping_elements() {
        let cube_on = Cube {
            x: (10, 12),
            y: (10, 12),
            z: (10, 12),
            turn_on: true,
        };
        let cube_off = Cube {
            x: (9, 11),
            y: (9, 11),
            z: (9, 11),
            turn_on: false,
        };
        let overlap = cube_on.overlapping_cube(&cube_off);
        let overlap_elements = overlap.expect("there should be overlap").elements();
        let expected = [
            (10, 10, 10),
            (10, 10, 11),
            (10, 11, 10),
            (10, 11, 11),
            (11, 10, 10),
            (11, 10, 11),
            (11, 11, 10),
            (11, 11, 11),
        ];
        assert_eq!(expected[..], overlap_elements[..]);
    }

    #[test]
    fn non_overlap_squares() {
        let cube_a = Cube {
            x: (0, 2),
            y: (0, 2),
            z: (0, 0),
            turn_on: true,
        };
        let cube_b = Cube {
            x: (1, 3),
            y: (1, 3),
            z: (0, 0),
            turn_on: true,
        };
        let overlap = Cube {
            x: (1, 2),
            y: (1, 2),
            z: (0, 0),
            turn_on: true,
        };
        assert_eq!(Some(overlap), cube_a.overlapping_cube(&cube_b));
        let mut expected = vec![(0, 0, 0), (1, 0, 0), (2, 0, 0), (0, 1, 0), (0, 2, 0)];
        expected.sort();
        let mut a_actual = cube_a
            .non_overlapping_cubes(&cube_b)
            .iter()
            .inspect(|c| println!("cube: {:?}", c))
            .flat_map(|x| x.elements())
            .collect_vec();
        a_actual.sort();
        assert_eq!(expected, a_actual);
    }

    #[test]
    fn non_overlap_test() {
        let cube = Cube {
            turn_on: true,
            x: (11, 13),
            y: (11, 13),
            z: (11, 13),
        };
        let other_cube = Cube {
            turn_on: true,
            x: (10, 12),
            y: (10, 12),
            z: (10, 12),
        };
        let non_overlap = cube.non_overlapping_cubes(&other_cube);
        let mut non_overlapping_elements =
            non_overlap.iter().flat_map(|c| c.elements()).collect_vec();
        non_overlapping_elements.sort();
        let mut unique = non_overlapping_elements.clone();
        unique.dedup();
        // assert_eq!(non_overlapping_elements, unique);
        let mut expected = vec![
            (11, 11, 13),
            (11, 12, 13),
            (11, 13, 11),
            (11, 13, 12),
            (11, 13, 13),
            (12, 11, 13),
            (12, 12, 13),
            (12, 13, 11),
            (12, 13, 12),
            (12, 13, 13),
            (13, 11, 11),
            (13, 11, 12),
            (13, 11, 13),
            (13, 12, 11),
            (13, 12, 12),
            (13, 12, 13),
            (13, 13, 11),
            (13, 13, 12),
            (13, 13, 13),
        ];
        expected.sort();
        assert_eq!(unique, expected);
    }

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
