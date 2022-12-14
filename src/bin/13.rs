use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, i64};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use serde_json::{Number, Value};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
enum Packet {
    Number(i64),
    List(Vec<Packet>),
}

impl std::cmp::Eq for Packet {}

impl std::cmp::PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.partial_cmp(b),
            (Packet::Number(_), Packet::List(b)) => {
                Packet::List(vec![self.clone()]).partial_cmp(&other)
            }
            (Packet::List(a), Packet::Number(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))
            }
            (Packet::List(a), Packet::List(b)) => {
                for (aa, bb) in a.iter().zip(b) {
                    if let Some(result) = aa.partial_cmp(bb) {
                        if result != Ordering::Equal {
                            return Option::from(result);
                        }
                    }
                }

                a.len().partial_cmp(&b.len())
            }
        }
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse(line: &str) -> IResult<&str, Packet> {
    alt((
        map(i64, |v| Packet::Number(v)),
        map(
            delimited(tag("["), separated_list0(tag(","), parse), tag("]")),
            |l| Packet::List(l),
        ),
    ))(line)
}

pub fn part_one(input: &str) -> Option<usize> {
    let packets = input
        .trim_end()
        .split("\n\n")
        .map(|lines| lines.lines().map(|l| parse(l).unwrap().1).collect())
        .collect::<Vec<Vec<Packet>>>();

    let mut res = 0;
    for (idx, p) in packets.iter().enumerate() {
        if p[0] < p[1] {
            res += idx + 1;
        }
    }

    Option::Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut packets = input
        .trim_end()
        .split("\n\n")
        .flat_map(|l| l.lines())
        // .lines()
        // .filter(|l| !l.is_empty())
        .map(|l| parse(l).unwrap().1)
        .collect::<Vec<Packet>>();

    let d1 = parse("[[2]]").unwrap().1;
    let d2 = parse("[[6]]").unwrap().1;

    packets.push(d1.clone());
    packets.push(d2.clone());

    packets.sort();

    let d1 = packets.iter().position(|p| p == &d1).unwrap();
    let d2 = packets.iter().position(|p| p == &d2).unwrap();

    Option::Some((d1 + 1) * (d2 + 1))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Option::Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Option::Some(140));
    }
}
