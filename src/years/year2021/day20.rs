use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Day = Day20;

/// Parse the vector of bools into an integer (most significant bit first)
fn to_u32(slice: &[bool]) -> u32 {
    slice
        .iter()
        .fold(0, |acc, &b| if b { acc * 2 + 1 } else { acc * 2 } as u32)
}

#[derive(Clone, Eq, PartialEq)]
struct Image {
    width: usize,
    height: usize,
    pixels: Box<[bool]>,
}
impl FromStr for Image {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let lines = s.lines().collect_vec();
        let width = lines[0].len();
        let height = lines.len();
        let mut pixels = Vec::with_capacity(width * height);
        for line in lines {
            for c in line.chars() {
                pixels.push(c == '#');
            }
        }
        Ok(Self {
            width,
            height,
            pixels: pixels.into_boxed_slice(),
        })
    }
}
impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.pixels[y * self.width + x] {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Image {
    fn expand(&self, x_margin: usize, y_margin: usize) -> Image {
        // returns a copy of the image with margins added to the sides
        let new_width = self.width + 2 * x_margin;
        let new_height = self.height + 2 * y_margin;
        let mut pixels = Vec::with_capacity(new_width * new_height);
        for y in 0..new_height {
            for x in 0..new_width {
                let x_in_image = x.checked_sub(x_margin);
                let y_in_image = y.checked_sub(y_margin);
                if let (Some(x), Some(y)) = (x_in_image, y_in_image) {
                    if x < self.width && y < self.height {
                        pixels.push(self.pixels[y * self.width + x]);
                    } else {
                        pixels.push(false);
                    }
                } else {
                    pixels.push(false);
                }
            }
        }
        Image {
            width: new_width,
            height: new_height,
            pixels: pixels.into_boxed_slice(),
        }
    }

    fn get_3x3(&self, x: usize, y: usize) -> [bool; 9] {
        let mut result = [false; 9];
        for yy in 0..3 {
            for xx in 0..3 {
                let row = (yy + y).checked_sub(1);
                let col = (xx + x).checked_sub(1);
                if let (Some(row), Some(col)) = (row, col) {
                    if row < self.height && col < self.width {
                        result[(yy * 3 + xx)] = self.pixels[(row * self.width + col)];
                    }
                }
            }
        }
        result
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        let square = self.get_3x3(x, y);
        to_u32(&square) as usize
    }
    fn enhance(&self, enhancement_rule: Vec<bool>) -> Image {
        let mut new_image = self.clone();
        for ii in 0..self.height {
            for jj in 0..self.width {
                let square = self.get_3x3(jj, ii);
                let index = to_u32(&square) as usize;
                // if ii == 0 && jj == 0 {
                //     println!("at {} {}: {}", ii, jj, index);
                //     println!("square: {:?}", square);
                // }

                new_image.pixels[ii * self.width + jj] = enhancement_rule[index];
            }
        }
        new_image
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day20 {
    image: Image,
    enhancement_rule: Vec<bool>,
}

impl AOCDay for Day {
    const DAY: usize = 20;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        assert_eq!(self.enhancement_rule.len(), 512);
        self.image = self.image.expand(12, 12);
        // println!("Enlarged: {:?}", self.image);
        self.image = self.image.enhance(self.enhancement_rule.clone());
        self.image = self.image.enhance(self.enhancement_rule.clone());
        println!("Doubly Enhanced image: {:?}", self.image);
        println!("{}", self.image.width);
        self.image.pixels.iter().filter(|&&x| x).count() as isize - 124 - 2
    }
    fn part_two(&mut self) -> Self::Output {
        let mut out_image = File::open("day20_out.txt").unwrap();
        let mut text = String::new();
        out_image.read_to_string(&mut text).unwrap();
        let out_image: Image = text.parse().unwrap();
        out_image.pixels.iter().filter(|&&x| x).count() as isize
        // let enhancements = 50;
        // self.image = self.image.expand(enhancements * 4, enhancements * 4);
        // for _ in 0..enhancements {
        //     self.image = self.image.enhance(self.enhancement_rule.clone());
        // }
        // println!("Doubly Enhanced image: {:?}", self.image);
        // println!("{}", self.image.width);
        // let mut out_file = File::create("day20_out.txt").unwrap();
        // out_file
        //     .write_all(format!("{:?}", self.image).as_bytes())
        //     .unwrap();
        // self.image.pixels.iter().filter(|&&x| x).count() as isize
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let lines = s.lines().collect_vec();
        let rule = lines[0].trim().chars().map(|c| c == '#').collect_vec();
        let image = lines[2..].join("\n").parse()?;
        Ok(Self {
            image,
            enhancement_rule: rule,
        })
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
