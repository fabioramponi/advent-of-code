use utils::read_input;
use utils::Purpose;
use crate::DayChallenge;

pub struct Day02 {
}

impl Day02 {
    pub fn init(purp: Purpose) -> Self {
        Day02 {
        }
    }
}

impl DayChallenge for Day02 {
    fn part_1(&self) -> String {
        String::from("Ciao 1")
    }

    fn part_2(&self) -> String {
        String::from("Ciao 2")
    }
}

fn parse_input(purp: Purpose) -> Vec<String> {
    read_input(2, purp)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day02 {
        Day02::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let day_02 = init_test();
        assert_eq!(day_02.part_1(), "24000");
    }

    #[test]
    fn part_2_works() {
        let day_02 = init_test();
        assert_eq!(day_02.part_2(), "45000");
    }
}