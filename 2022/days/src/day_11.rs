use crate::DayChallenge;
use primes::factors;
use utils::read_input;
use utils::Purpose;

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    if_true: usize,
    if_false: usize,
    activity: u64,
}

struct ThrowTo {
    to_monkey: usize,
    item: u64,
}

impl Monkey {
    fn throw_items(&mut self, divide: bool) -> Vec<ThrowTo> {
        let mut res: Vec<ThrowTo> = vec![];
        while !self.items.is_empty() {
            let initial_item = self.items.pop().unwrap();
            let after_inspect = (self.operation)(initial_item);
            let item = if divide {
                after_inspect / 3
            } else {
                after_inspect
            };
            let test = item % self.test == 0;
            let to_monkey = if test { self.if_true } else { self.if_false };
            self.activity += 1;
            res.push(ThrowTo { to_monkey, item });
        }
        res
    }
}

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

impl Day11 {
    pub fn init(purp: Purpose) -> Self {
        Day11 {
            monkeys: parse_input(purp),
        }
    }

    fn worry_level_after(&mut self, n_turns: usize, divide: bool) -> u64 {
        for _ in 0..n_turns {
            for monkey_idx in 0..self.monkeys.len() {
                let throws = self.monkeys[monkey_idx].throw_items(divide);
                throws.iter().for_each(|t| {
                    self.monkeys
                        .get_mut(t.to_monkey)
                        .unwrap()
                        .items
                        .push(t.item);
                });
            }
        }
        self.monkeys
            .sort_by(|m1, m2| m1.activity.partial_cmp(&m2.activity).unwrap());
        self.monkeys[self.monkeys.len() - 1].activity
            * self.monkeys[self.monkeys.len() - 2].activity
    }
}

impl DayChallenge for Day11 {
    fn part_1(&mut self) -> String {
        self.worry_level_after(20, true).to_string()
    }

    fn part_2(&mut self) -> String {
        self.worry_level_after(1000, false).to_string()
    }
}

fn parse_input(purp: Purpose) -> Vec<Monkey> {
    let input = read_input(11, purp);
    let monkeys_str = input.chunks(7);
    monkeys_str
        .into_iter()
        .map(|ms| {
            //let _idx: usize = ms[0].split_whitespace().last().unwrap().parse().unwrap();
            let items: Vec<u64> = ms[1]
                .split("items: ")
                .last()
                .unwrap()
                .split(",")
                .map(|n_str| n_str.trim().parse().unwrap())
                .collect();
            let operation = Box::new(parse_expression(ms[2].split("= ").last().unwrap()));
            let test: u64 = ms[3].split_whitespace().last().unwrap().parse().unwrap();
            let if_true: usize = ms[4].split_whitespace().last().unwrap().parse().unwrap();
            let if_false: usize = ms[5].split_whitespace().last().unwrap().parse().unwrap();
            let activity: u64 = 0;

            Monkey {
                items,
                operation,
                test,
                if_true,
                if_false,
                activity,
            }
        })
        .collect()
}

fn parse_expression(s: &str) -> Box<dyn Fn(u64) -> u64> {
    let splitted: Vec<&str> = s.split_whitespace().collect();
    if splitted[2] == "old" {
        match splitted[1] {
            "+" => Box::new(|n| 2 * n),
            "*" => Box::new(|n| n * n),
            &_ => todo!(),
        }
    } else {
        let val = splitted[2].parse::<u64>().unwrap();
        match splitted[1] {
            "+" => Box::new(move |n| n + val),
            "*" => Box::new(move |n| n * val),
            &_ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day11 {
        Day11::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_01 = init_test();
        assert_eq!(day_01.part_1(), "10605");
    }

    #[test]
    fn part_2_works() {
        let mut day_01 = init_test();
        assert_eq!(day_01.part_2(), "2713310158");
    }
}
