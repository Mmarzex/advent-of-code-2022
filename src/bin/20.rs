use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<i32> {
    let grove_seq = input
        .trim_end()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<VecDeque<i32>>();

    let mut grove_seq_mixed = grove_seq
        .iter()
        .enumerate()
        .collect::<VecDeque<(usize, &i32)>>();

    for (idx, num) in grove_seq.iter().enumerate() {
        let current_idx = grove_seq_mixed
            .iter()
            .position(|x| x.0 == idx && x.1 == num)
            .unwrap();
        grove_seq_mixed.remove(current_idx);
        let new_idx =
            ((current_idx as i32 + *num).rem_euclid(grove_seq_mixed.len() as i32)) as usize;
        grove_seq_mixed.insert(new_idx, (idx, num));
    }


    let offset = grove_seq_mixed.iter().position(|p| *p.1 == 0).unwrap();

    let a = grove_seq_mixed.iter().cycle().nth(1000 + offset).unwrap().1;
    let b = grove_seq_mixed.iter().cycle().nth(2000 + offset).unwrap().1;
    let c = grove_seq_mixed.iter().cycle().nth(3000 + offset).unwrap().1;

    Option::Some(a + b + c)
    // None
}

pub fn part_two(input: &str) -> Option<i64> {
    let grove_seq = input
        .trim_end()
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * 811589153)
        .collect::<VecDeque<i64>>();

    let mut grove_seq_mixed = grove_seq
        .iter()
        .enumerate()
        .collect::<VecDeque<(usize, &i64)>>();

    for _ in 0..10 {
        for (idx, num) in grove_seq.iter().enumerate() {
            let current_idx = grove_seq_mixed
                .iter()
                .position(|x| x.0 == idx && x.1 == num)
                .unwrap();
            grove_seq_mixed.remove(current_idx);

            let new_idx = ((current_idx as i64 + *num).rem_euclid(grove_seq_mixed.len() as i64)) as usize;

            grove_seq_mixed.insert(new_idx, (idx, num));
        }
    }

    let offset = grove_seq_mixed.iter().position(|p| *p.1 == 0).unwrap();

    let a = grove_seq_mixed.iter().cycle().nth(1000 + offset).unwrap().1;
    let b = grove_seq_mixed.iter().cycle().nth(2000 + offset).unwrap().1;
    let c = grove_seq_mixed.iter().cycle().nth(3000 + offset).unwrap().1;

    Option::Some(a + b + c)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Option::Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Option::Some(1623178306));
    }
}
