use std::collections::HashSet;

use utils::{read_input, Purpose};

use crate::DayChallenge;

struct Rucksack {
    content: Vec<u8>,
}

impl Rucksack {
    fn first_compartment(&self) -> &[u8] {
        self.content[..self.content.len() / 2].as_ref()
    }

    fn second_compartment(&self) -> &[u8] {
        self.content[self.content.len() / 2..].as_ref()
    }
}

fn priority(val: u8) -> u32 {
    match &val {
        97..=122 => val as u32 - 96,
        _ => val as u32 - 38,
    }
}

pub struct Day03 {
    rucksacks: Vec<Rucksack>,
}

impl Day03 {
    pub fn init(purp: Purpose) -> Self {
        Day03 {
            rucksacks: parse_input(purp),
        }
    }
}

impl DayChallenge for Day03 {
    fn part_1(&self) -> String {
        let mut total: u32 = 0;
        self.rucksacks.iter().for_each(|rucksack| {
            let item_types_1: HashSet<u8> =
                HashSet::from_iter(rucksack.first_compartment().iter().copied());
            let item_types_2: HashSet<u8> =
                HashSet::from_iter(rucksack.second_compartment().iter().copied());
            let common_item = item_types_1
                .intersection(&item_types_2)
                .copied()
                .next()
                .expect("intersection must be long 1");
            total += priority(common_item)
        });
        return total.to_string();
    }

    fn part_2(&self) -> String {
        let mut total: u32 = 0;
        self.rucksacks.chunks(3).for_each(|group| {
            let intersection: HashSet<u8> = group.iter().skip(1).fold(
                HashSet::from_iter(group[0].content.iter().copied()),
                |acc, rs| {
                    acc.intersection(&HashSet::from_iter(rs.content.iter().copied()))
                        .cloned()
                        .collect()
                },
            );
            let common_item = intersection
                .iter()
                .copied()
                .next()
                .expect("intersection must be long 1");
            total += priority(common_item);
        });
        total.to_string()
    }
}

fn parse_input(purp: Purpose) -> Vec<Rucksack> {
    let input = read_input(3, purp);
    let mut res: Vec<Rucksack> = Vec::new();

    for line in input {
        res.push(Rucksack {
            content: line.as_bytes().clone().to_vec(),
        });
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day03 {
        Day03::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let day_03 = init_test();
        assert_eq!(day_03.part_1(), "157");
    }

    #[test]
    fn part_2_works() {
        let day_03 = init_test();
        assert_eq!(day_03.part_2(), "70");
    }
}
