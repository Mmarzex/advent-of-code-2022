use std::collections::VecDeque;

#[derive(Clone, Debug)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Clone, Debug)]
enum OperationSide {
    Number(usize),
    Old,
}

#[derive(Clone, Debug)]
struct Equation {
    operation: Operation,
    left: OperationSide,
    right: OperationSide,
}

#[derive(Clone, Debug)]
struct Monkey {
    checks: usize,
    items: VecDeque<usize>,
    equation: Equation,
    test: usize,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        let lines = input.trim_end().lines().collect::<Vec<&str>>();

        let items = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        let operation_parts = lines[2]
            .split(" = ")
            .nth(1)
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>();
        let op_left = if operation_parts[0] == "old" {
            OperationSide::Old
        } else {
            OperationSide::Number(operation_parts[0].parse::<usize>().unwrap())
        };
        let op_right = if operation_parts[2] == "old" {
            OperationSide::Old
        } else {
            OperationSide::Number(operation_parts[2].parse::<usize>().unwrap())
        };

        let equation = Equation {
            left: op_left,
            right: op_right,
            operation: match operation_parts[1] {
                "+" => Operation::Addition,
                "*" => Operation::Multiplication,
                &_ => panic!(),
            },
        };

        let test = lines[3]
            .split("by ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let test_true = lines[4]
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let test_false = lines[5]
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            checks: 0,
            items,
            equation,
            test,
            test_true,
            test_false,
        }
    }

    fn process(&mut self, worry_reducer: impl Fn(usize) -> usize) -> Option<(usize, usize)> {
        if let Some(item) = self.items.pop_front() {
            self.checks += 1;
            let left = match self.equation.left {
                OperationSide::Number(n) => n,
                OperationSide::Old => item,
            };
            let right = match self.equation.right {
                OperationSide::Number(n) => n,
                OperationSide::Old => item,
            };
            let mut new_value = match self.equation.operation {
                Operation::Addition => left + right,
                Operation::Multiplication => left * right,
            };

            // new_value /= 3;

            new_value = worry_reducer(new_value);

            if new_value % self.test == 0 {
                return Option::Some((new_value, self.test_true));
            } else {
                return Option::Some((new_value, self.test_false));
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = input
        .trim_end()
        .split("\n\n")
        .map(|m| Monkey::new(m))
        .collect::<Vec<Monkey>>();

    for _ in 0..20 {
        for id in 0..monkeys.len() {
            while monkeys[id].items.len() > 0 {
                if let Some((item, monkey_id)) = monkeys[id].process(|i| i / 3) {
                    monkeys[monkey_id].items.push_back(item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| a.checks.cmp(&b.checks));

    let monkey_business: usize = monkeys.iter().rev().take(2).map(|m| m.checks).product();
    Option::Some(monkey_business)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = input
        .trim_end()
        .split("\n\n")
        .map(|m| Monkey::new(m))
        .collect::<Vec<Monkey>>();

    let m: usize = monkeys.iter().map(|m| m.test).product();

    for _ in 0..10000 {
        for id in 0..monkeys.len() {
            while monkeys[id].items.len() > 0 {
                if let Some((item, monkey_id)) = monkeys[id].process(|i| i % m) {
                    monkeys[monkey_id].items.push_back(item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| a.checks.cmp(&b.checks));

    let monkey_business: usize = monkeys.iter().rev().take(2).map(|m| m.checks).product();
    Option::Some(monkey_business)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Option::Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Option::Some(2713310158));
    }
}
