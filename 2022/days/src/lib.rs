use utils::Purpose;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

pub trait DayChallenge {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

pub fn get_day(day: u8, purp: Purpose) -> Option<Box<dyn DayChallenge>> {
    match day {
        1 => Some(Box::new(day_01::Day01::init(purp))),
        2 => Some(Box::new(day_02::Day02::init(purp))),
        3 => Some(Box::new(day_03::Day03::init(purp))),
        4 => Some(Box::new(day_04::Day04::init(purp))),
        _ => None,
    }
}
