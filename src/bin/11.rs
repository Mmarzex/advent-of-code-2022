use std::collections::VecDeque;



#[derive(Clone, Debug)]
struct Monkey<'a> {
    checks: usize,
    items: VecDeque<usize>,
    operation: &'a str,
    test: usize,
    test_true: usize,
    test_false: usize,
}

impl Monkey<'_> {
    fn new(input: &str) -> Monkey {
        let lines = input.trim_end().lines().collect::<Vec<&str>>();

        let items = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        let operation = lines[2].split(" = ").nth(1).unwrap();

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
            items: items,
            operation: operation,
            test: test,
            test_true: test_true,
            test_false: test_false,
        }
    }

    fn process(&mut self) -> Option<(usize, usize)> {
        if let Some(item) = self.items.pop_front() {
            self.checks += 1;
            let operation_parts = self.operation.split(' ').collect::<Vec<&str>>();
            let op_left = if operation_parts[0] == "old" {
                item.clone()
            } else {
                operation_parts[0].parse::<usize>().unwrap()
            };
            // let op = operation_parts[1];
            let op_right = if operation_parts[2] == "old" {
                item.clone()
            } else {
                operation_parts[2].parse::<usize>().unwrap()
            };

            let mut new_value = match operation_parts[1] {
                "+" => op_left + op_right,
                "*" => op_left * op_right,
                _ => panic!(),
            };

            new_value /= 3;

            if new_value % self.test == 0 {
                return Option::Some((new_value, self.test_true));
            } else {
                return Option::Some((new_value, self.test_false));
            }
        }

        None
    }

    fn process_p2(&mut self, m: usize) -> Option<(usize, usize)> {
        if let Some(item) = self.items.pop_front() {
            self.checks += 1;
            let operation_parts = self.operation.split(' ').collect::<Vec<&str>>();
            let op_left = if operation_parts[0] == "old" {
                item.clone()
            } else {
                operation_parts[0].parse::<usize>().unwrap()
            };
            // let op = operation_parts[1];
            let op_right = if operation_parts[2] == "old" {
                item.clone()
            } else {
                operation_parts[2].parse::<usize>().unwrap()
            };

            let mut new_value = match operation_parts[1] {
                "+" => op_left + op_right,
                "*" => op_left * op_right,
                _ => panic!(),
            };

            new_value %= m;
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
                if let Some((item, monkey_id)) = monkeys[id].process() {
                    monkeys[monkey_id].items.push_back(item);
                }
            }
        }
    }

    let mut monkey_business_values = monkeys.iter().map(|m| m.checks).collect::<Vec<usize>>();
    monkey_business_values.sort();
    let monkey_business: usize = monkey_business_values.iter().rev().take(2).product();
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
                if let Some((item, monkey_id)) = monkeys[id].process_p2(m) {
                    monkeys[monkey_id].items.push_back(item);
                }
            }
        }
    }

    let mut monkey_business_values = monkeys.iter().map(|m| m.checks).collect::<Vec<usize>>();
    monkey_business_values.sort();
    let monkey_business: usize = monkey_business_values.iter().rev().take(2).product();
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
