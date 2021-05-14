use std::collections::HashSet;
use itertools::Itertools;

fn solve_part_one(mut grid: Grid) -> usize {
    let cycles = 6;
    // println!("Before any cycles: {:?}", grid.active_points);
    for _ in 0..cycles {
        grid.run_cyle();
        // println!("active nodes: {:?}", grid.active_points.iter().sorted_by_key(|p| p.z));
    }

    grid.active_points.len()
}

fn solve_part_two(grid: Grid) -> usize {
    todo!()
}

#[derive(Debug)]
struct Grid {
    // tocuhed_points: Vec<Point>,
    active_points: HashSet<Point>,
    is_fourdim: bool,
}


#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Grid {
    fn get_neighbours(point: &Point, fourdim: bool) -> Vec<Point> {
        let mut res = Vec::with_capacity(26);
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if fourdim {
                        for w in -1..=1 {
                            if !(x == 0 && [x, y, z, w].iter().all_equal()) {
                                res.push(Point { x: point.x + x, y: point.y + y, z: point.z + z, w: point.w + w });
                            }
                        }
                    } else {
                        if !(x == 0 && [x, y, z].iter().all_equal()) {
                            res.push(Point { x: point.x + x, y: point.y + y, z: point.z + z, w:0 });
                        }
                    }
                }
            }
        }
        res
    }

    fn run_cyle(&mut self) {
        let mut inactive: HashSet<Point> = HashSet::with_capacity(self.active_points.len() * 26);
        let mut next_state: HashSet<_> = self.active_points.clone();
        for active_point in &self.active_points {
            let (active, passive): (HashSet<Point>, HashSet<_>) = Self::get_neighbours(&active_point, self.is_fourdim).iter()
                .partition(|p| self.active_points.contains(p));
            // println!("Point {:?} has active neighbours \n{:?}\npassive:\n{:?}", active_point, active, passive);

            inactive.extend(passive);
            if !matches!(active.len(), 2|3) {
                next_state.remove(&active_point);
                // println!("Turned off point");
            }
        }
        for passive_point in inactive {
            if Self::get_neighbours(&passive_point, self.is_fourdim).iter()
                .filter(|p| self.active_points.contains(p))
                .count() == 3 {
                next_state.insert(passive_point);
            }
        }
        self.active_points = next_state;
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::error::Error;
    use super::*;
    use itertools::Itertools;

    fn parse_input() -> Result<Grid, Box<dyn Error>> {
        let input = read_to_string("inputs/day17.txt")?;
        let lines = input.lines();
        let points = lines.enumerate()
            .flat_map(|(y, l)| l.char_indices()
                .filter_map(move |(x, c)|
                    if c == '#' {
                        return Some(Point {
                            x: x as isize,
                            y: y as isize,
                            z: 0,
                            w: 0,
                        });
                    } else {
                        None
                    }
                )).collect();
        Ok(Grid {
            active_points: points,
            is_fourdim: false,
        })
    }

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let grid = parse_input()?;
        let solution = solve_part_one(grid);
        println!("[day17] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let mut grid = parse_input()?;
        grid.is_fourdim = true;
        let solution = solve_part_one(grid);
        println!("[day17] solution part 2: {}", solution);
        Ok(())
    }
}