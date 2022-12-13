use std::collections::HashMap;

use crate::DayChallenge;
use nalgebra::DMatrix;
use priority_queue::PriorityQueue;
use utils::read_input;
use utils::Purpose;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Status {
    position: Position,
    step: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position(usize, usize);

impl Position {
    fn to_tuple(&self) -> (usize, usize) {
        return (self.0, self.1);
    }
}

#[derive(Debug)]
struct HeightMap {
    map: DMatrix<u32>,
    start: Position,
    end: Position,
    shortest_paths: HashMap<Position, u32>,
    priority_queue: PriorityQueue<Status, i32>,
}

impl HeightMap {
    fn visit(&mut self, pos: Position, actual_step: u32) {
        //println!("{:?}, {}", pos, actual_step);
        let ep = self.shortest_paths.get(&self.end);
        if ep.is_some() {
            //println!("found end! {:?}", ep);
            if *ep.unwrap() < actual_step {
                return;
            }
        }
        let sp = self.shortest_paths.get_mut(&pos);
        let mut proceed = false;
        match sp {
            None => {
                let _ = self.shortest_paths.insert(pos, actual_step);
                proceed = true;
            }
            Some(v) => {
                if *v > actual_step {
                    *v = actual_step;
                    proceed = true;
                }
            }
        };
        if proceed {
            let actual_height = self.map.get(pos.to_tuple()).unwrap().clone();
            //let x_towards_e = (self.end.0 as i32 - pos.0 as i32).signum();
            //let y_towards_e = (self.end.1 as i32 - pos.1 as i32).signum();
            for m in [(1, 0i32), (0, 1), (-1, 0), (0, -1)] {
                //for m in [(0, -1), (-1i32, 0i32), (0, 1), (1, 0)] {
                let x = pos.0 as i32 + m.0;
                let y = pos.1 as i32 + m.1;
                if x >= 0 && y >= 0 && x < self.map.nrows() as i32 && y < self.map.ncols() as i32 {
                    let potential_move = Position(x as usize, y as usize);
                    let new_height = self.map.get(potential_move.to_tuple()).unwrap().clone();
                    let delta = new_height as i32 - actual_height as i32;
                    //println!("Trying {:?} -> {:?}", pos, potential_move);
                    let skip = delta > 1
                        || (self.shortest_paths.contains_key(&potential_move)
                            && self.shortest_paths.get(&potential_move).unwrap()
                                < &(actual_step + 1));
                    if !skip {
                        //println!("{:?}", potential_move);
                        self.priority_queue.push(
                            Status {
                                position: potential_move,
                                step: actual_step + 1,
                            },
                            (new_height * 1_000_000 - (actual_step + 1)) as i32,
                        );
                        //self.visit(potential_move, &new_visited, actual_step + 1);
                    }
                }
            }
        }
    }

    fn find_shortest(&mut self, start: Position) -> Option<&u32> {
        self.priority_queue.clear();
        self.shortest_paths.clear();
        self.priority_queue.push(
            Status {
                position: start,
                step: 0,
            },
            1i32,
        );
        let mut prio: i32 = 0;
        loop {
            let new_pos = self.priority_queue.pop();
            match new_pos {
                None => break,
                Some(p) => {
                    if p.1 / 1_000_000 != prio {
                        prio = p.1 / 1_000_000;
                        /*println!(
                            "new step: {}, position: {:?}, priority: {}",
                            p.0.step, p.0.position, p.1
                        );*/
                    }
                    self.visit(p.0.position, p.0.step);
                }
            }
        }
        self.shortest_paths.get(&self.end)
    }
}

pub struct Day12 {
    height_map: HeightMap,
}

impl Day12 {
    pub fn init(purp: Purpose) -> Self {
        Day12 {
            height_map: parse_input(purp),
        }
    }
}

impl DayChallenge for Day12 {
    fn part_1(&mut self) -> String {
        //println!("{}", self.height_map.map);
        //self.height_map.visit(self.height_map.start, &vec![], 0);
        self.height_map
            .find_shortest(self.height_map.start)
            .unwrap()
            .to_string()
    }

    fn part_2(&mut self) -> String {
        let starts: Vec<Position> = self
            .height_map
            .map
            .iter()
            .enumerate()
            .filter(|&(i, &v)| v == 'a' as u32)
            .map(|(i, _)| {
                let start = self.height_map.map.vector_to_matrix_index(i);
                Position(start.0, start.1)
            })
            .collect();

        let mut shortest = 1_000_000u32;
        for s in starts {
            let this_shortest = self.height_map.find_shortest(s);
            if this_shortest.is_some() && this_shortest.unwrap() < &shortest {
                shortest = *this_shortest.unwrap();
            }
        }
        shortest.to_string()
    }
}

fn parse_input(purp: Purpose) -> HeightMap {
    let input = read_input(12, purp);
    let field: Vec<u32> = input
        .iter()
        .map(|l| l.trim().chars().map(|c| c as u32))
        .flatten()
        .collect();
    let mut map = DMatrix::from_vec(input[0].len(), input.len(), field);
    let (start_idx, _pos) = map
        .iter()
        .enumerate()
        .find(|(_i, &el)| el == 'S' as u32)
        .unwrap();
    let (end_idx, _pos) = map
        .iter()
        .enumerate()
        .find(|(_i, &el)| el == 'E' as u32)
        .unwrap();

    let start = map.vector_to_matrix_index(start_idx);
    let end = map.vector_to_matrix_index(end_idx);
    *map.get_mut(end).unwrap() = 'z' as u32;
    *map.get_mut(start).unwrap() = 'a' as u32;
    HeightMap {
        map,
        start: Position(start.0, start.1),
        end: Position(end.0, end.1),
        shortest_paths: HashMap::new(),
        priority_queue: PriorityQueue::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day12 {
        Day12::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_12 = init_test();
        assert_eq!(day_12.part_1(), "31");
    }

    #[test]
    fn part_2_works() {
        let mut day_12 = init_test();
        assert_eq!(day_12.part_2(), "29");
    }
}
