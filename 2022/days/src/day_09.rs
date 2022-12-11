use std::collections::HashSet;


use crate::DayChallenge;
use nalgebra::SimdPartialOrd;
use utils::read_input;
use utils::Purpose;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Move {
    d: Direction,
    l: usize,
}

#[derive(PartialEq, Hash, Clone)]
struct Position {
    x:i16,
    y:i16
}

impl Position {
    fn is_near(&self, other: &Self) -> bool {
        other.x>=self.x-1 && other.x<= self.x+1 && other.y>=self.y-1 && other.y<=self.y+1
    }

    fn follow(&mut self, head: &Self) {
        if self.is_near(head) {return;}
        let x_dist = head.x - self.x;
        let y_dist = head.y - self.y;
        if x_dist.abs() > 1 {
            self.y = if y_dist == 0 {self.y} else {self.y + y_dist.signum()};
            self.x = self.x + x_dist.signum();
        } else {
            self.x = if x_dist == 0 {self.x} else {self.x + x_dist.signum()};
            self.y = self.y + y_dist.signum();
        }
    }
}

impl Eq for Position {

}


pub struct Day09 {
    moves:Vec<Move>,
}

impl Day09 {
    pub fn init(purp: Purpose) -> Self {
        Day09 {
            moves: parse_input(purp)
        }
    }
}

impl DayChallenge for Day09 {
    fn part_1(&mut self) -> String {
        let mut visited :HashSet<Position> = HashSet::new();
        let mut head: Position = Position { x: 0, y: 0 };
        let mut tail: Position = Position { x:0, y: 0 };
        visited.insert(tail.clone());
        for m in &self.moves {
            for _ in 0..m.l {
                match m.d {
                    Direction::Right => {
                        head = Position{x:head.x+1,y:head.y};
                    },
                    Direction::Left  => {
                        head = Position{x:head.x-1,y:head.y};
                    },
                    Direction::Up => {
                        head = Position{x:head.x,y:head.y+1};
                    },
                    Direction::Down  => {
                        head = Position{x:head.x,y:head.y-1};
                    },
                }
                if !head.is_near(&tail) {
                    tail.follow(&head);
                    visited.insert(tail.clone());
                }
            }
        }
        visited.len().to_string()
    }

    fn part_2(&mut self) -> String {
        let mut visited :HashSet<Position> = HashSet::new();
        let mut knots: Vec<Position> = vec![Position { x:0, y: 0 };10];
        visited.insert(knots[9].clone());
        for m in &self.moves {
            for _ in 0..m.l {
                match m.d {
                    Direction::Right => {
                        knots[0] = Position{x:knots[0].x+1,y:knots[0].y};
                    },
                    Direction::Left  => {
                        knots[0] = Position{x:knots[0].x-1,y:knots[0].y};
                    },
                    Direction::Up => {
                        knots[0] = Position{x:knots[0].x,y:knots[0].y+1};
                    },
                    Direction::Down  => {
                        knots[0] = Position{x:knots[0].x,y:knots[0].y-1};
                    },
                }
                for k in 1..10 {
                    let is_near = knots[k].is_near(&knots[k-1]);
                    if !is_near {
                        let prev_knot = knots[k-1].clone();
                        knots[k].follow(&prev_knot);
                    }
                }
                visited.insert(knots[9].clone());
            }
        }
        visited.len().to_string()
    }
}

fn parse_input(purp: Purpose) -> Vec<Move> {
    let input = read_input(9, purp);
    input.iter().map(|l| {
        let move_str = l.split_whitespace().collect::<Vec<&str>>();
        let sz:usize = move_str[1].parse().unwrap();
        match move_str[0] {
            "R" => Move{d:Direction::Right,l:sz},
            "U" => Move{d:Direction::Up,l:sz},
            "L" => Move{d:Direction::Left,l:sz},
            "D" => Move{d:Direction::Down,l:sz},
            &_ => todo!()
        }}).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day09 {
        Day09::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_09 = init_test();
        assert_eq!(day_09.part_1(), "13");
    }

    #[test]
    fn part_2_works() {
        let mut day_09 = init_test();
        assert_eq!(day_09.part_2(), "36");
    }
}
