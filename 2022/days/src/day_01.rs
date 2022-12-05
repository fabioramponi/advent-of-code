use crate::DayChallenge;
use utils::read_input;
use utils::Purpose;

pub struct Day01 {
    energies: Vec<i32>,
}

impl Day01 {
    pub fn init(purp: Purpose) -> Self {
        Day01 {
            energies: (parse_input(purp)),
        }
    }
}

impl DayChallenge for Day01 {
    fn part_1(&mut self) -> String {
        self.energies
            .iter()
            .max()
            .expect("Unexpected error")
            .clone()
            .to_string()
    }

    fn part_2(&mut self) -> String {
        let mut energies = self.energies.clone();
        energies.sort_by(|a, b| b.cmp(a));
        let s: i32 = energies.iter().take(3).sum();
        s.to_string()
    }
}

fn parse_input(purp: Purpose) -> Vec<i32> {
    let input = read_input(1, purp);
    let mut energies: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    input.iter().for_each(|x| match x.as_ref() {
        "" => {
            energies.push(total);
            total = 0;
        }
        _ => {
            total += x.parse::<i32>().unwrap();
        }
    });
    energies.push(total);
    energies.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day01 {
        Day01::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_01 = init_test();
        assert_eq!(day_01.part_1(), "24000");
    }

    #[test]
    fn part_2_works() {
        let mut day_01 = init_test();
        assert_eq!(day_01.part_2(), "45000");
    }
}
