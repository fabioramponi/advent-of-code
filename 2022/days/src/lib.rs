use utils::Purpose;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_10;

pub trait DayChallenge {
    fn part_1(&mut self) -> String;
    fn part_2(&mut self) -> String;
}

pub fn get_day(day: u8, purp: Purpose) -> Option<Box<dyn DayChallenge>> {
    match day {
        1 => Some(Box::new(day_01::Day01::init(purp))),
        2 => Some(Box::new(day_02::Day02::init(purp))),
        3 => Some(Box::new(day_03::Day03::init(purp))),
        4 => Some(Box::new(day_04::Day04::init(purp))),
        5 => Some(Box::new(day_05::Day05::init(purp))),
        6 => Some(Box::new(day_06::Day06::init(purp))),
        7 => Some(Box::new(day_07::Day07::init(purp))),
        8 => Some(Box::new(day_08::Day08::init(purp))),
        10 => Some(Box::new(day_10::Day10::init(purp))),
        _ => None,
    }
}
