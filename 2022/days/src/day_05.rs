use regex::Regex;
use utils::{read_input, Purpose};

use crate::DayChallenge;

pub struct Day05 {
    stack: Vec<Vec<u8>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}

impl Day05 {
    pub fn init(purp: Purpose) -> Self {
        let (stack, moves) = parse_input(&purp);
        Day05 { stack, moves }
    }
}

impl DayChallenge for Day05 {
    fn part_1(&mut self) -> String {
        let mut stack = self.stack.clone();
        self.moves.iter().for_each(|m| {
            for _ in 0..m.how_many {
                let to_move = stack[m.from - 1].pop().expect("must exist");
                stack[m.to - 1].push(to_move);
            }
        });

        let lasts: Vec<u8> = stack.iter().map(|s| s[s.len() - 1]).collect();
        String::from_utf8(lasts).expect("Found invalid UTF-8")
    }

    fn part_2(&mut self) -> String {
        let mut stack = self.stack.clone();
        self.moves.iter().for_each(|m| {
            let from_size = stack[m.from - 1].len();
            let to_move: Vec<u8> = stack[m.from - 1]
                .drain(from_size - m.how_many..from_size)
                .collect();
            stack[m.to - 1].extend(to_move);
        });

        let lasts: Vec<u8> = stack.iter().map(|s| s[s.len() - 1]).collect();
        String::from_utf8(lasts).expect("Found invalid UTF-8")
    }
}

fn parse_input(purp: &Purpose) -> (Vec<Vec<u8>>, Vec<Move>) {
    let input = read_input(5, purp);

    let stack_size: usize = input
        .iter()
        .find(|l| l.starts_with(" 1"))
        .expect("stack_size")
        .split_whitespace()
        .last()
        .expect("last does not exist")
        .parse()
        .expect("parse error");

    let mut res: Vec<Vec<u8>> = vec![Vec::new(); stack_size];
    let mut moves: Vec<Move> = vec![];
    let stacks_re = Regex::new(r"[\[ ]([A-Z ])[ \]][ ]?").unwrap();
    let moves_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut stacks = true;
    for l in input {
        if l.starts_with(" 1") || l.is_empty() {
            stacks = false;
            continue;
        }
        if stacks {
            let boh: Vec<u8> = stacks_re
                .captures_iter(&l)
                .map(|c| c.get(1).expect("must be something").as_str().as_bytes()[0])
                .collect();

            for (i, v) in boh.iter().cloned().enumerate() {
                if v != " ".as_bytes()[0] {
                    res[i].insert(0, v);
                }
            }
        } else {
            let move_parsed = moves_re.captures(&l).unwrap();
            moves.push(Move {
                how_many: move_parsed
                    .get(1)
                    .expect("what error")
                    .as_str()
                    .parse()
                    .expect("what parse error"),
                from: move_parsed
                    .get(2)
                    .expect("from error")
                    .as_str()
                    .parse()
                    .expect("from parse error"),
                to: move_parsed
                    .get(3)
                    .expect("to error")
                    .as_str()
                    .parse()
                    .expect("to parse error"),
            })
        }
    }
    (res.clone(), moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day05 {
        Day05::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_05 = init_test();
        assert_eq!(day_05.part_1(), "CMZ");
    }

    #[test]
    fn part_2_works() {
        let mut day_05 = init_test();
        assert_eq!(day_05.part_2(), "MCD");
    }
}
