use std::collections::HashMap;

use crate::DayChallenge;
use enum_map::{enum_map, Enum, EnumMap};
use utils::read_input;
use utils::Purpose;

#[derive(Debug, Enum, PartialEq, Eq, Clone, Copy, Hash)]
enum Output {
    ROCK = 1,
    PAPER = 2,
    SCISSOR = 3,
}

#[derive(Clone)]
struct Strategy {
    opponent: String,
    you: String,
}

#[derive(Clone, Copy)]
struct Turn {
    opponent: Output,
    you: Output,
}

impl Turn {
    fn points(self, wins: EnumMap<Output, Output>) -> u32 {
        let choice_pts = self.you as u32;

        if self.opponent == self.you {
            return choice_pts + 3;
        };
        if wins[self.you] == self.opponent {
            return choice_pts + 6;
        };
        return choice_pts;
    }
}

pub struct Day02 {
    strategy_guide: Vec<Strategy>,
    wins: EnumMap<Output, Output>,
}

impl Day02 {
    pub fn init(purp: Purpose) -> Self {
        Day02 {
            strategy_guide: parse_input(purp),
            wins: enum_map! {
            Output::PAPER => Output::ROCK,
            Output::SCISSOR => Output::PAPER,
            Output::ROCK => Output::SCISSOR,},
        }
    }
}

impl DayChallenge for Day02 {
    fn part_1(&mut self) -> String {
        let mapping: HashMap<String, Output> = HashMap::from([
            (String::from("A"), Output::ROCK),
            (String::from("B"), Output::PAPER),
            (String::from("C"), Output::SCISSOR),
            (String::from("X"), Output::ROCK),
            (String::from("Y"), Output::PAPER),
            (String::from("Z"), Output::SCISSOR),
        ]);

        let turns: Vec<Turn> = self
            .strategy_guide
            .iter()
            .map(|t| Turn {
                opponent: mapping[&t.opponent],
                you: mapping[&t.you],
            })
            .collect();

        turns
            .iter()
            .map(|t| t.points(self.wins))
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&mut self) -> String {
        let mapping: HashMap<String, Output> = HashMap::from([
            (String::from("A"), Output::ROCK),
            (String::from("B"), Output::PAPER),
            (String::from("C"), Output::SCISSOR),
        ]);
        let mut res: u32 = 0;
        for s in &self.strategy_guide {
            let opponent_move = mapping[&s.opponent];
            let you_move = match s.you.as_str() {
                "Y" => opponent_move,
                "X" => self.wins[opponent_move],
                "Z" => {
                    self.wins
                        .iter()
                        .filter(|(_k, &v)| v == opponent_move)
                        .next()
                        .expect("")
                        .0
                }
                &_ => todo!(),
            };
            res += Turn {
                opponent: opponent_move,
                you: you_move,
            }
            .points(self.wins)
        }
        res.to_string()
    }
}

fn parse_input(purp: Purpose) -> Vec<Strategy> {
    let input = read_input(2, purp);
    let turns_str: Vec<Vec<String>> = input
        .iter()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    turns_str
        .iter()
        .map(|t| Strategy {
            opponent: t[0].clone(),
            you: t[1].clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day02 {
        Day02::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_02 = init_test();
        assert_eq!(day_02.part_1(), "15");
    }

    #[test]
    fn part_2_works() {
        let mut day_02 = init_test();
        assert_eq!(day_02.part_2(), "12");
    }
}
