use crate::DayChallenge;
use nalgebra::DMatrix;
use std::cmp::max;
use utils::read_input;
use utils::Purpose;

pub struct Day08 {
    field: DMatrix<u8>,
}

impl Day08 {
    pub fn init(purp: Purpose) -> Self {
        Day08 {
            field: parse_input(&purp),
        }
    }
}

impl DayChallenge for Day08 {
    fn part_1(&mut self) -> String {
        let (n_rows, n_cols) = self.field.shape();
        let mut n_visible = n_rows * 2 + n_cols * 2 - 4;
        for r in 1..n_rows - 1 {
            for c in 1..n_cols - 1 {
                let el = *self.field.get((c, r)).unwrap();
                let mut visible = self.field.slice((0, r), (c, 1)).max() < el;
                visible = visible || self.field.slice((c + 1, r), (n_cols - c - 1, 1)).max() < el;
                visible = visible || self.field.slice((c, 0), (1, r)).max() < el;
                visible = visible || self.field.slice((c, r + 1), (1, n_rows - r - 1)).max() < el;
                if visible {
                    n_visible += 1;
                }
            }
        }
        n_visible.to_string()
    }

    fn part_2(&mut self) -> String {
        let (n_rows, n_cols) = self.field.shape();
        let mut max_distance = 0;
        for r in 1..n_rows - 1 {
            for c in 1..n_cols - 1 {
                let el = *self.field.get((c, r)).unwrap();
                let mut left_distance = 0;
                for pos in (0..r).rev() {
                    left_distance += 1;
                    if *self.field.get((c, pos)).unwrap() >= el {
                        break;
                    }
                }

                let mut right_distance = 0;
                for pos in r + 1..n_rows {
                    right_distance += 1;
                    if *self.field.get((c, pos)).unwrap() >= el {
                        break;
                    }
                }

                let mut up_distance = 0;
                for pos in (0..c).rev() {
                    up_distance += 1;
                    if *self.field.get((pos, r)).unwrap() >= el {
                        break;
                    }
                }

                let mut down_distance = 0;
                for pos in c + 1..n_cols {
                    down_distance += 1;
                    if *self.field.get((pos, r)).unwrap() >= el {
                        break;
                    }
                }

                let distance = right_distance * left_distance * up_distance * down_distance;
                max_distance = max(distance, max_distance);
            }
        }
        max_distance.to_string()
    }
}

fn parse_input(purp: &Purpose) -> DMatrix<u8> {
    let input = read_input(8, purp);
    let field: Vec<u8> = input
        .iter()
        .map(|l| l.chars().map(|c| c.to_string().parse::<u8>().unwrap()))
        .flatten()
        .collect();
    DMatrix::from_vec(input.len(), input.len(), field)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day08 {
        Day08::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_08 = init_test();
        assert_eq!(day_08.part_1(), "21");
    }

    #[test]
    fn part_2_works() {
        let mut day_08 = init_test();
        assert_eq!(day_08.part_2(), "8");
    }
}
