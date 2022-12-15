use crate::DayChallenge;
use regex::Regex;
use utils::read_input;
use utils::Purpose;

pub struct Day15 {
    sensors: Vec<Sensor>,
}

struct Position(i32, i32);

struct Sensor {
    position: Position,
    closest_beacon: Position,
}

fn dist(p1: &Position, p2: &Position) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

impl Sensor {
    fn dist_from_beacon(&self) -> u32 {
        dist(&self.position, &self.closest_beacon)
    }

    fn covered_positions(&self, line: i32) -> Option<(i32, i32)> {
        let dist_from_line = dist(&self.position, &Position(self.position.0, line));
        let dist_from_beacon = self.dist_from_beacon();

        if dist_from_beacon < dist_from_line {
            None
        } else {
            let residual = self.dist_from_beacon() - dist_from_line;
            Some((
                self.position.0 - residual as i32,
                self.position.0 + residual as i32,
            ))
        }
    }
}

impl Day15 {
    pub fn init(purp: Purpose) -> Self {
        Day15 {
            sensors: parse_input(purp),
        }
    }

    fn covered_at_line(&self, line: i32) -> Vec<(i32, i32)> {
        let mut ranges = self
            .sensors
            .iter()
            .map(|s| s.covered_positions(line))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect::<Vec<(i32, i32)>>();

        ranges.sort_by(|c1, c2| c1.0.cmp(&c2.0));
        let mut res: Vec<(i32, i32)> = vec![];
        let mut last: (i32, i32) = ranges.get_mut(0).unwrap().clone();
        for r in ranges.iter().skip(1) {
            if last.1 >= r.0 && r.1 > last.1 {
                last.1 = r.1;
            } else if last.1 < r.0 {
                res.push(last.clone());
                last = r.clone();
            }
        }
        res.push(last.clone());
        res
    }
}

impl DayChallenge for Day15 {
    fn part_1(&mut self) -> String {
        let res = self.covered_at_line(2000000);
        let n = res.iter().fold(0, |acc, v| acc + v.1 - v.0);
        n.to_string()
    }

    fn part_2(&mut self) -> String {
        for i in 0..=4000000 {
            let covered = self.covered_at_line(i);
            if covered.len() > 1 {
                let x = covered[0].1 as i64 + 1i64;
                println!("found ({}, {}), {:?}", x, i, covered);
                return (x * 4000000 + i as i64).to_string();
            } else if covered[0].0 > 0 {
                println!("found {}, {}, {:?}", 0, i, covered);
                return i.to_string();
            } else if covered[0].1 < 4000000 {
                return ((covered[0].1 + 1) * 4000000 + i).to_string();
            }
        }
        "".to_string()
    }
}

fn parse_input(purp: Purpose) -> Vec<Sensor> {
    let input = read_input(15, purp);
    let re = Regex::new(
        r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)",
    )
    .unwrap();
    input
        .iter()
        .map(|l| {
            if let Some(n) = re.captures(l) {
                let sx = n.name("sx").unwrap().as_str().parse::<i32>().unwrap();
                let sy = n.name("sy").unwrap().as_str().parse::<i32>().unwrap();
                let bx = n.name("bx").unwrap().as_str().parse::<i32>().unwrap();
                let by = n.name("by").unwrap().as_str().parse::<i32>().unwrap();
                Sensor {
                    position: Position(sx, sy),
                    closest_beacon: Position(bx, by),
                }
            } else {
                Sensor {
                    position: Position(0, 0),
                    closest_beacon: Position(0, 0),
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day15 {
        Day15::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_01 = init_test();
        assert_eq!(day_01.part_1(), "26");
    }

    #[test]
    fn part_2_works() {
        let mut day_01 = init_test();
        assert_eq!(day_01.part_2(), "56000011");
    }
}
