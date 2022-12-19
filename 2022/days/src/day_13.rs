use std::cmp::Ordering;

use crate::DayChallenge;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use utils::read_input;
use utils::Purpose;

#[derive(Debug, Clone)]
enum Node {
    Number(u32),
    List(Vec<Node>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        match (self, other) {
            (&Node::Number(n1), &Node::Number(n2)) => {
                if n1 < n2 {
                    Ordering::Less
                } else if n1 > n2 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (Node::List(n1), Node::List(n2)) => {
                for i in 0..n1.len() {
                    if i < n2.len() {
                        let is_ordered = n1[i].cmp(&n2[i]);
                        if is_ordered != Ordering::Equal {
                            return is_ordered;
                        }
                    }
                }
                if n1.len() < n2.len() {
                    return Ordering::Less;
                } else if n1.len() > n2.len() {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            }
            (Node::Number(n1), Node::List(_)) => {
                let n1_list = Node::List(vec![Node::Number(*n1)]);
                n1_list.cmp(other)
            }
            (Node::List(_), Node::Number(n2)) => {
                let n2_list = Node::List(vec![Node::Number(*n2)]);
                self.cmp(&n2_list)
            }
        }
    }
}

fn digit(input: &str) -> IResult<&str, char> {
    one_of("0123456789")(input)
}

fn uint(input: &str) -> IResult<&str, &str> {
    recognize(many1(digit))(input)
}

fn number(input: &str) -> nom::IResult<&str, Node> {
    let parser = recognize(uint);
    map(parser, |s| {
        // FIXME: unwrap() may panic if the value is out of range
        let n = s.parse::<u32>().unwrap();
        Node::Number(n)
    })(input)
}

fn array(input: &str) -> IResult<&str, Node> {
    let parser = delimited(tag("["), separated_list0(tag(","), value), tag("]"));
    map(parser, |v| Node::List(v))(input)
}

fn value(input: &str) -> IResult<&str, Node> {
    alt((array, number))(input)
}

type Pair = (Node, Node);

pub struct Day13 {
    list_pairs: Vec<Pair>,
}

impl Day13 {
    pub fn init(purp: Purpose) -> Self {
        Day13 {
            list_pairs: parse_input(&purp),
        }
    }
}

impl DayChallenge for Day13 {
    fn part_1(&mut self) -> String {
        self.list_pairs
            .iter()
            .map(|(n1, n2)| n1.cmp(n2))
            .enumerate()
            .filter(|(_, order)| *order == Ordering::Less)
            .fold(0, |acc, v| acc + v.0 + 1)
            .to_string()
    }

    fn part_2(&mut self) -> String {
        let mut pairs = self.list_pairs.clone();

        let d1 = Node::List(vec![Node::List(vec![Node::Number(2)])]);
        let d2 = Node::List(vec![Node::List(vec![Node::Number(6)])]);

        pairs.push((d1.clone(), d2.clone()));
        let mut all: Vec<Node> = Vec::new();
        pairs.iter().for_each(|(n1, n2)| {
            all.push(n1.clone());
            all.push(n2.clone());
        });
        all.sort();
        all.iter()
            .enumerate()
            .filter(|(_, v)| (*v).cmp(&d1) == Ordering::Equal || (*v).cmp(&d2) == Ordering::Equal)
            .fold(1, |acc, (i, _)| acc * (i + 1))
            .to_string()
    }
}

fn parse_input(purp: &Purpose) -> Vec<Pair> {
    let input = read_input(13, purp);
    let nodes: Vec<Node> = input
        .into_iter()
        .filter(|s| s.len() > 0)
        .map(|s| {
            let (_, result) = all_consuming(array)(&s)
                .map_err(|nom_err| match nom_err {
                    nom::Err::Incomplete(_) => unreachable!(),
                    nom::Err::Error(e) => e,
                    nom::Err::Failure(e) => e,
                })
                .unwrap();
            result
        })
        .collect();
    nodes
        .chunks(2)
        .map(|c| (c[0].clone(), c[1].clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Day13 {
        Day13::init(Purpose::Test)
    }

    #[test]
    fn part_1_works() {
        let mut day_13 = init_test();
        assert_eq!(day_13.part_1(), "13");
    }

    #[test]
    fn part_2_works() {
        let mut day_13 = init_test();
        assert_eq!(day_13.part_2(), "140");
    }
}
