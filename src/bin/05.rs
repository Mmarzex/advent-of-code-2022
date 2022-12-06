use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let lines = stacks.lines().collect::<Vec<&str>>();

    let (stack_indexes_lines, stack_lines) = lines.split_last().unwrap();

    let mut crate_stacks: Vec<Vec<char>> = vec![];

    let stack_indexes = stack_indexes_lines.split("   ").count();

    for _ in 0..stack_indexes {
        crate_stacks.push(vec![]);
    }

    for line in stack_lines.iter() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let y = chunk.join("");


            let yt = y.trim();

            if !yt.is_empty() {
                crate_stacks[idx].insert(0, yt.chars().nth(1).unwrap());
            }
        }
    }

    for instruction in instructions.lines() {
        let x = instruction.split(' ').collect::<Vec<&str>>();
        let n = x[1].parse::<usize>().unwrap();
        let from = x[3].parse::<usize>().unwrap() - 1;
        let to = x[5].parse::<usize>().unwrap() - 1;

        let t = (0..n)
            .map(|_| crate_stacks[from].pop().unwrap())
            .collect::<Vec<char>>();

        for elem in t.iter() {
            crate_stacks[to].push(*elem);
        }
    }

    let phrase: String = crate_stacks.iter().map(|s| s.last().unwrap()).collect();

    Option::Some(phrase)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let lines = stacks.lines().collect::<Vec<&str>>();

    let (stack_indexes_lines, stack_lines) = lines.split_last().unwrap();

    let mut crate_stacks: Vec<Vec<char>> = vec![];

    let stack_indexes = stack_indexes_lines.split("   ").count();

    for _ in 0..stack_indexes {
        crate_stacks.push(vec![]);
    }

    for line in stack_lines.iter() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let y = chunk.join("");


            let yt = y.trim();

            if !yt.is_empty() {
                crate_stacks[idx].insert(0, yt.chars().nth(1).unwrap());
            }
        }
    }

    for instruction in instructions.lines() {
        let x = instruction.split(' ').collect::<Vec<&str>>();
        let n = x[1].parse::<usize>().unwrap();
        let from = x[3].parse::<usize>().unwrap() - 1;
        let to = x[5].parse::<usize>().unwrap() - 1;

        let t = (0..n)
            .map(|_| crate_stacks[from].pop().unwrap())
            .collect::<Vec<char>>();

        for elem in t.iter().rev() {
            crate_stacks[to].push(*elem);
        }
    }

    let phrase: String = crate_stacks.iter().map(|s| s.last().unwrap()).collect();

    Option::Some(phrase)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Option::Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Option::Some("MCD".to_string()));
    }
}
