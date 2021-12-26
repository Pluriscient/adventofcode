use super::util::AResult;
use super::AOCDay;
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
#[cfg(test)]
use std::time::{SystemTime, UNIX_EPOCH};

type Day = Day21;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Score {
    player_score: usize,
    player_pos: usize,
    opp_score: usize,
    opp_pos: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Day21 {
    player_pos: usize,
    opp_pos: usize,
}

impl Day {
    fn solve(&self) -> usize {
        const MOVEMENT_OCCURRENCES: [(usize, usize); 7] =
            [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let mut scores = HashMap::new();
        scores.insert(
            Score {
                player_pos: self.player_pos,
                player_score: 0,
                opp_pos: self.opp_pos,
                opp_score: 0,
            },
            1,
        );
        let mut player_won = 0;
        let mut opp_won = 0;
        #[cfg(test)]
        let mut sizes = Vec::with_capacity(20);
        'out: loop {
            for (won_score, is_player) in [(&mut player_won, true), (&mut opp_won, false)] {
                let mut new_scores = HashMap::new();
                for (score, count) in scores.iter() {
                    for &(player_move, occurrences) in &MOVEMENT_OCCURRENCES {
                        let old_pos = if is_player {
                            score.player_pos
                        } else {
                            score.opp_pos
                        };
                        let old_score = if is_player {
                            score.player_score
                        } else {
                            score.opp_score
                        };
                        let new_pos = (old_pos + player_move - 1) % 10 + 1;
                        let new_score = old_score + new_pos;

                        if new_score >= 21 {
                            *won_score += count * occurrences;
                            continue;
                        }
                        let new_score = if is_player {
                            Score {
                                player_pos: new_pos,
                                player_score: new_score,
                                ..*score
                            }
                        } else {
                            Score {
                                opp_pos: new_pos,
                                opp_score: new_score,
                                ..*score
                            }
                        };
                        *new_scores.entry(new_score).or_insert(0) += count * occurrences;
                    }
                }
                if new_scores.is_empty() {
                    break 'out;
                }
                #[cfg(test)]
                sizes.push(new_scores.len());
                scores = new_scores;
            }
        }
        #[cfg(test)]
        std::io::Write::write_all(
            &mut std::fs::File::create(format!(
                "day21-sizes-{}.txt",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ))
            .unwrap(),
            sizes.iter().map(|x| x.to_string()).join("\n").as_bytes(),
        )
        .unwrap();

        player_won.max(opp_won)
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
        self.solve()
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
