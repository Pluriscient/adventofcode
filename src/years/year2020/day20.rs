use itertools::__std_iter::FromIterator;
use itertools::Itertools;
use std::str::FromStr;
use std::fmt::{Debug, Formatter};
use std::fmt;

fn solve_part_one(tiles: Vec<Tile>) -> usize {
    // let's first find the edges that don't fit with the rest
    let mut eff_tiles = tiles.iter().map(EffTile::new).collect_vec();
    println!("flipping tile {:?}", &eff_tiles[0]);
    &eff_tiles[0].flip(FlipDirection::Horizontal);
    println!("flipped tile {:?}", &eff_tiles[0]);
    // now we find those tiles that have sides that don't fit with other 
    todo!()
}

fn solve_part_two(tiles: Vec<Tile>) -> usize {
    todo!()
}

struct Tile {
    id: u32,
    data: Vec<Vec<bool>>,
    top: Vec<bool>,
    bottom: Vec<bool>,
    left: Vec<bool>,
    right: Vec<bool>,
}

impl Debug for EffTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("Tile {}: [{:010b}, {:010b},{:010b},{:010b}]", self.id, self.corners[0], self.corners[1], self.corners[2], self.corners[3]))
    }
}

struct EffTile {
    id: u32,
    corners: [u32; 4],
}

enum FlipDirection {
    Horizontal,
    Vertical,
}

impl EffTile {
    fn boolvec_to_num(vec: Vec<bool>) -> u32 {
        assert!(vec.len() <= 32);
        let mut res = 0;
        for (i, b) in vec.iter().rev().enumerate() {
            if *b {
                res |= (1 << i)
            }
        }
        res
    }

    fn new(tile: &Tile) -> Self {
        let top = EffTile::boolvec_to_num(tile.top.clone());
        let bottom = EffTile::boolvec_to_num(tile.bottom.clone());
        let left = EffTile::boolvec_to_num(tile.left.clone());
        let right = EffTile::boolvec_to_num(tile.right.clone());
        Self {
            id: tile.id,
            corners: [top, right, bottom, left],
        }
    }

    fn rotate(&mut self, rotations: isize) {
        if rotations < 0 {
            self.corners.rotate_left(rotations.abs() as usize)
        } else {
            self.corners.rotate_right(rotations as usize)
        }
    }
    fn flip(&mut self, dir: FlipDirection) {
        // we use our knowledge that the bit length is 10, and 32-10==22
        match dir {
            FlipDirection::Horizontal => {
                self.corners[1] = self.corners[1].reverse_bits() >> 22;
                self.corners[3] = self.corners[3].reverse_bits() >> 22;
            }
            FlipDirection::Vertical => {
                self.corners[0] = self.corners[0].reverse_bits() >> 22;
                self.corners[2] = self.corners[2].reverse_bits() >> 22;
            }
        }
    }
}


impl Tile {
    fn new(id: u32, data: Vec<Vec<bool>>) -> Self {
        let (top, bottom) = (data[0].clone(), data[data.len() - 1].clone());
        let left = data.iter().map(|d| d[0]).collect_vec();
        let right = data.iter().map(|d| d[d.len() - 1]).collect_vec();
        Self {
            id,
            data,
            top,
            bottom,
            left,
            right,
        }
    }
}


impl<'a> FromIterator<&'a str> for Tile {
    fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();
        let tile_id: &str = iterator.next().unwrap();
        let id = &tile_id["Tile ".len()..tile_id.len() - 1];
        // println!("{} -> {}", tile_id, id);
        let data = iterator
            .map(|line: &str| line.trim().chars().map(|c| c == '#').collect_vec())
            .collect_vec();

        return Tile::new(u32::from_str(id).unwrap(), data);
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::error::Error;
    use super::*;

    fn parse_input() -> Result<Vec<Tile>, Box<dyn Error>> {
        let input = read_to_string("inputs/day20.txt")?;
        let mut lines = input.lines().peekable();

        // todo transform lines to our input somehow
        let mut res: Vec<Tile> = vec![];
        while let Some(line) = lines.peek() {
            let tile = lines.peeking_take_while(|l| !l.trim().is_empty()).collect();
            lines.next();
            res.push(tile);
        }


        Ok(res)
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let tiles = parse_input()?;
        let solution = solve_part_one(tiles);
        println!("[day20] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let tiles = parse_input()?;
        let solution = solve_part_two(tiles);
        println!("[day20] solution part 2: {}", solution);
        Ok(())
    }
}