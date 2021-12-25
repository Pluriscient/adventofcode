use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

type Day = Day21;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Score {
    player_score: usize,
    player_pos: usize,
    turns: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day21 {
    player_pos: usize,
    opp_pos: usize,
}

#[allow(unused)]
impl Day {
    fn solve(&self, start_score: usize) -> HashMap<usize, usize> {
        const MOVEMENT_OCCURRENCES: [(usize, usize); 7] =
            [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let mut scores = HashMap::new();
        scores.insert(
            Score {
                player_pos: start_score,
                player_score: 0,
                turns: 0,
            },
            1,
        );
        let mut finished = HashMap::new();
        loop {
            let mut new_scores = HashMap::new();

            for (score, count) in scores.iter() {
                if score.player_score >= 21 {
                    *finished.entry(score.turns).or_insert(0) += *count;
                    continue;
                }
                for &(player_move, occurrences) in &MOVEMENT_OCCURRENCES {
                    let p_pos = (score.player_pos + player_move - 1) % 10 + 1;
                    let new_score = Score {
                        player_pos: p_pos,
                        player_score: score.player_score + p_pos,
                        turns: score.turns + 1,
                    };
                    *new_scores.entry(new_score).or_insert(0) += (count * occurrences);
                }
            }
            if new_scores.is_empty() {
                break;
            }
            scores = new_scores;
        }
        finished
    }
}

struct Die {
    max: usize,
    last_roll: usize,
    rolls: usize,
}

impl Die {
    fn new(max: usize) -> Self {
        Self {
            max,
            last_roll: 0,
            rolls: 0,
        }
    }
    fn roll(&mut self) -> usize {
        self.rolls += 1;
        self.last_roll = (self.last_roll) % self.max + 1;
        // println!("Rolled {}", self.last_roll);
        self.last_roll
    }
}

impl AOCDay for Day {
    const DAY: usize = 21;
    type Output = usize;

    fn part_one(&mut self) -> Self::Output {
        let mut die = Die::new(100);
        let mut player_score = 0;

        let mut opp_score = 0;
        for turn in 1.. {
            // first player one rolls thrice
            let rolls = [die.roll(), die.roll(), die.roll()];
            let sum = rolls.iter().sum::<usize>();
            self.player_pos = (self.player_pos + sum - 1) % 10 + 1;
            player_score += self.player_pos;
            println!("Player rolls {:?} and moves to {}", rolls, self.player_pos);
            if player_score >= 1000 {
                println!(
                    "Player wins after {} turns ({} rolls) with a score of {} (opponent: {})",
                    turn, die.rolls, player_score, opp_score
                );
                return opp_score * die.rolls;
            }
            let rolls = [die.roll(), die.roll(), die.roll()];
            let sum = rolls.iter().sum::<usize>();
            self.opp_pos = (self.opp_pos + sum - 1) % 10 + 1;
            opp_score += self.opp_pos;
            if opp_score >= 1000 {
                println!(
                    "Opponent wins after {} turns ({} rolls) with a score of {} (opponent: {})",
                    turn, die.rolls, opp_score, player_score
                );
                return player_score * die.rolls;
            }
        }
        unreachable!()
    }
    fn part_two(&mut self) -> Self::Output {
        // we're trying to find the number of universes in which one player wins
        // score to reach is 21
        // we're probably going to have to use memoization
        // we want to known the number of universes in which one player wins
        // there are 27 outcomes for each player turn
        // so 27 * 27 = 729 universes per turn
        // so 729 ^ X = number of universes we need to consider
        // highest number of turns is a such a score that we get a 1 every time (technically we can only have 9 score, rotating every 10 we get a 1)
        let solve_a = self.solve(self.player_pos);
        let solve_b = self.solve(self.opp_pos);
        // so there are
        println!("{:?}", solve_a);
        println!("a will have won {} times ", solve_a.values().sum::<usize>());
        println!("{:?}", solve_b);
        println!("b will have won {} times ", solve_b.values().sum::<usize>());
        // let mut mem = HashMap::with_capacity(10000);
        // expecting: 444356092776315
        // got:       196218569
        
        solve_a.values().sum::<usize>()
    }
}

impl FromStr for Day {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> AResult<Self> {
        let players = s
            .lines()
            .map(|p| {
                p.split(":").collect_vec()[1]
                    .trim()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect_vec();
        Ok(Self {
            player_pos: players[0],
            opp_pos: players[1],
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
