use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{fmt::Error, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Operand {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl FromStr for Operand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operand::Addition),
            "-" => Ok(Operand::Subtraction),
            "*" => Ok(Operand::Multiplication),
            "/" => Ok(Operand::Division),
            _ => panic!(),
        }
    }
}

impl Operand {
    pub fn eval(&self, left: f64, right: f64) -> f64 {
        match &self {
            Self::Addition => left + right,
            Self::Subtraction => left - right,
            Self::Multiplication => left * right,
            Self::Division => left / right,
        }
    }
}

#[derive(Debug, Clone)]
enum Equation {
    Operation(String, Operand, String),
    Number(f64),
}

fn parse(line: &str) -> (String, Equation) {
    let (key, v) = line.split_once(": ").unwrap();

    match v.parse::<f64>() {
        Ok(vv) => (key.to_string(), Equation::Number(vv)),
        Err(_) => {
            let (a, o, b) = v.split_whitespace().take(3).collect_tuple().unwrap();
            (
                key.to_string(),
                Equation::Operation(a.to_string(), Operand::from_str(o).unwrap(), b.to_string()),
            )
        }
    }
}

fn solve(from: &str, list: &HashMap<String, Equation>) -> f64 {
    match list.get(from).unwrap().clone() {
        Equation::Number(v) => v,
        Equation::Operation(l, o, r) => o.eval(solve(&l, list), solve(&r, list)),
    }
}

pub fn part_one(input: &str) -> Option<f64> {
    let equations = input
        .trim_end()
        .lines()
        .map(parse)
        .collect::<HashMap<String, Equation>>();

    println!("{:?}", equations);

    let res = solve("root", &equations);

    Option::Some(res)
}

pub fn part_two(input: &str) -> Option<f64> {
    let mut equations = input
        .trim_end()
        .lines()
        .map(parse)
        .collect::<HashMap<String, Equation>>();

    let (rleft, rright) = match equations.get("root").unwrap().clone() {
        Equation::Number(_) => panic!(),
        Equation::Operation(l, _, r) => (l, r),
    };

    let left = solve(&rleft, &equations);
    let right = solve(&rright, &equations);

    if let Equation::Number(v) = equations.get_mut("humn").unwrap() {
        *v -= 1.0;
    }

    let on_right = left == solve(&rleft, &equations);

    let goal = if on_right { left } else { right };
    let from = if on_right { rright } else { rleft };

    println!("on_right {:?} goal: {:?}, from: {:?}", on_right, goal, from);

    let mut low = 0f64;

    let mut high = 10_000_000_000_000f64;

    loop {
        let human_val = ((low + high) / 2.0f64).floor();
        if human_val < high {
            println!("human_val: {:?}", human_val);
        }

        if let Equation::Number(v) = equations.get_mut("humn").unwrap() {
            *v = human_val;
        } else {
            panic!()
        }

        let res = solve(&from, &equations);
        let cmp_ord = res.total_cmp(&goal);

        println!("res: {:?} cmp_ord: {:?}", res, cmp_ord);

        match cmp_ord {
            Ordering::Greater => low = human_val + 1.0,
            Ordering::Less => high = human_val - 1.0,
            Ordering::Equal => return Option::Some(human_val),
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Option::Some(152.0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Option::Some(301.0));
    }
}
