use utils::{read_input, Purpose};

use crate::DayChallenge;

pub struct Day06 {
    signal: Vec<u8>,
}

impl Day06 {
    pub fn init(purp: Purpose) -> Self {
        let v = read_input(6, purp);
        //let (stack, moves) = parse_input(purp);
        Day06 {
            signal: v.iter().next().unwrap().as_bytes().to_vec(),
        }
    }
}

fn find_start(signal: &mut Vec<u8>, l: usize) -> usize {
    let i = signal
        .windows(l)
        .enumerate()
        .find(|(_, w)| {
            let mut thisw = w.to_vec();
            thisw.sort();
            !thisw.windows(2).any(|w| w[0] == w[1])
        })
        .unwrap()
        .0;
    return i + l;
}

impl DayChallenge for Day06 {
    fn part_1(&mut self) -> String {
        return find_start(&mut self.signal, 4).to_string();
    }

    fn part_2(&mut self) -> String {
        return find_start(&mut self.signal, 14).to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day06 {
        Day06::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_06 = init_test();
        assert_eq!(day_06.part_1(), "10");
    }

    #[test]
    fn part_2_works() {
        let mut day_06 = init_test();
        assert_eq!(day_06.part_2(), "");
    }
}
