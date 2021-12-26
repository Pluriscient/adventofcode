use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type Day = Day23;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
    Empty,
}
impl FromStr for Amphipod {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        match s {
            "A" => Ok(Amphipod::Amber),
            "B" => Ok(Amphipod::Bronze),
            "C" => Ok(Amphipod::Copper),
            "D" => Ok(Amphipod::Desert),
            "." => Ok(Amphipod::Empty),
            _ => Err(From::from(format!("Unknown Amphipod: {}", s))),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cave {
    hallway: Vec<Amphipod>,
    a_cave: Vec<Amphipod>,
    b_cave: Vec<Amphipod>,
    c_cave: Vec<Amphipod>,
    d_cave: Vec<Amphipod>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Day23 {
    start: Cave,
}
#[allow(unused)]
impl Day {
    /// Returns a list of valid moves from the current state
    /// in the form of the next state and the cost of the move
    fn possible_moves(state: &Cave) -> Vec<(Cave, usize)> {
        let mut moves = Vec::new();
        // consider a perfect situation, only one amphipod still has to move from the hallway into the cave it belongs to.
        if let Some((index, amp)) = state
            .hallway
            .iter()
            .find_position(|amp| !matches!(amp, Amphipod::Empty))
        {
            let mut new_state = state.clone();
            new_state.hallway[index] = Amphipod::Empty;
            match amp {
                Amphipod::Amber => {
                    new_state.a_cave[index] = *amp;
                }
                Amphipod::Bronze => {
                    new_state.b_cave[index] = *amp;
                }
                Amphipod::Copper => {
                    new_state.c_cave[index] = *amp;
                }
                Amphipod::Desert => {
                    new_state.d_cave[index] = *amp;
                }
                _ => unreachable!(),
            }
            moves.push((new_state, 1));
        }

        // for i in 0..state.len() {}
        moves
    }

    fn solve(current_cost: usize, current_state: Cave, target: &Cave) -> usize {
        if current_state == *target {
            return current_cost;
        }
        let mut best_cost = std::usize::MAX;
        // for (i, (a, b)) in current_state.iter().enumerate() {
        //     let mut new_state = current_state.clone();
        //     new_state[i] = *b;
        //     new_state[i + 1] = *a;
        //     let new_cost = Day23::solve(current_cost + 1, new_state, target);
        //     if new_cost < best_cost {
        //         best_cost = new_cost;
        //     }
        // }
        best_cost
    }
}

impl AOCDay for Day {
    const DAY: usize = 23;
    type Output = isize;

    fn part_one(&mut self) -> Self::Output {
        let target = Cave {
            hallway: vec![Amphipod::Empty; self.start.hallway.len()],
            a_cave: vec![Amphipod::Amber; self.start.a_cave.len()],
            b_cave: vec![Amphipod::Bronze; self.start.b_cave.len()],
            c_cave: vec![Amphipod::Copper; self.start.c_cave.len()],
            d_cave: vec![Amphipod::Desert; self.start.d_cave.len()],
        };
        let res = Self::solve(0, self.start.clone(), &target);
        res as Self::Output
    }
    fn part_two(&mut self) -> Self::Output {
        todo!()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        // we store a configuration as follows: [HALLWAY, A_ROOM, B_ROOM, C_ROOM, D_ROOM]
        let lines = s
            .lines()
            .skip(2)
            .take(2)
            .map(|line| line.trim().trim_matches('#'))
            .collect_vec(); // assume hallway is empty
        let (a, b, c, d) = lines[0].split("#").collect_tuple().unwrap();
        let (e, f, g, h) = lines[1].split("#").collect_tuple().unwrap();
        let start = vec![
            Amphipod::from_str(a).unwrap(),
            Amphipod::from_str(e).unwrap(),
            Amphipod::from_str(b).unwrap(),
            Amphipod::from_str(f).unwrap(),
            Amphipod::from_str(c).unwrap(),
            Amphipod::from_str(g).unwrap(),
            Amphipod::from_str(d).unwrap(),
            Amphipod::from_str(h).unwrap(),
        ];
        todo!()
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
