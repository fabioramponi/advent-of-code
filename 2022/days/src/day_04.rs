use regex::Regex;
use utils::{read_input, Purpose};

use crate::DayChallenge;

#[derive(Debug)]
struct Assignment {
    from: u32,
    to: u32,
}

#[derive(Debug)]
struct Pair {
    elf_1: Assignment,
    elf_2: Assignment,
}

impl Pair {
    fn fully_includes(&self) -> u8 {
        if self.elf_1.from <= self.elf_2.from && self.elf_1.to >= self.elf_2.to {
            return 1;
        }
        if self.elf_1.from >= self.elf_2.from && self.elf_1.to <= self.elf_2.to {
            return 2;
        }
        return 0;
    }

    fn overlap(&self) -> bool {
        self.fully_includes() > 0
            || (self.elf_1.from >= self.elf_2.from && self.elf_1.from <= self.elf_2.to)
            || (self.elf_1.to >= self.elf_2.from && self.elf_1.to <= self.elf_2.to)
    }
}

pub struct Day04 {
    pairs: Vec<Pair>,
}

impl Day04 {
    pub fn init(purp: Purpose) -> Self {
        Day04 {
            pairs: parse_input(purp),
        }
    }
}

impl DayChallenge for Day04 {
    fn part_1(&mut self) -> String {
        self.pairs
            .iter()
            .filter(|p| p.fully_includes() > 0)
            .count()
            .to_string()
    }

    fn part_2(&mut self) -> String {
        self.pairs
            .iter()
            .filter(|p| p.overlap())
            .count()
            .to_string()
    }
}

fn parse_input(purp: Purpose) -> Vec<Pair> {
    let input = read_input(4, purp);
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    input
        .iter()
        .map(|l| {
            let captures = re.captures(l.as_str()).expect("not captured");
            let assignment_1 = Assignment {
                from: captures
                    .get(1)
                    .expect("not found")
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
                to: captures
                    .get(2)
                    .expect("not found")
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
            };
            let assignment_2 = Assignment {
                from: captures
                    .get(3)
                    .expect("not found")
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
                to: captures
                    .get(4)
                    .expect("not found")
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
            };
            Pair {
                elf_1: assignment_1,
                elf_2: assignment_2,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day04 {
        Day04::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_04 = init_test();
        assert_eq!(day_04.part_1(), "2");
    }

    #[test]
    fn part_2_works() {
        let mut day_04 = init_test();
        assert_eq!(day_04.part_2(), "4");
    }
}
