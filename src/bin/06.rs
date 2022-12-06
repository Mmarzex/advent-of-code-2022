use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 14)
}

fn solve(input: &str, n: usize) -> Option<u32> {
    let mut seq: VecDeque<char> = VecDeque::new();

    for (idx, w) in input.chars().enumerate() {
        if seq.len() == n {
            seq.pop_front();
        }

        seq.push_back(w);

        if seq.len() == n {
            let check_set: HashSet<char> = seq.clone().into_iter().collect();
            if check_set.len() == n {
                return Option::Some((idx + 1) as u32);
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Option::Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Option::Some(19));
    }
}
