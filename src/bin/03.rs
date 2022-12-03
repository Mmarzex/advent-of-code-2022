use std::collections::HashSet;

use itertools::Itertools;

// use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let x: u32 = input
        .trim_end()
        .lines()
        .map(|l| {
            let (one, two) = l.split_at(l.len() / 2);

            let one_s: HashSet<char> = one.chars().collect();

            let two_s: HashSet<char> = two.chars().collect();

            let z: Vec<&char> = one_s.intersection(&two_s).into_iter().collect();

            let found = (*z[0]) as u32;

            let v = if found > 96 { found - 96 } else { found - 38 };

            v
        })
        .sum();

    Option::Some(x)
}

pub fn part_two(input: &str) -> Option<u32> {
    let x: Vec<&str> = input.trim_end().lines().collect();

    let mut sum: u32 = 0;

    for chunk in &x.into_iter().chunks(3) {
        let sets: Vec<HashSet<char>> = chunk.map(|l| l.chars().collect()).collect();
        let one = &sets[0];
        let two = &sets[1];
        let three = &sets[2];

        let z: HashSet<char> = one
            .intersection(&two)
            .map(|c| *c)
            .collect::<HashSet<char>>();
        let zz: Vec<&char> = z.intersection(three).collect();

        let found = (*zz[0]) as u32;

        let v = if found > 96 { found - 96 } else { found - 38 };

        sum += v;
    }

    Option::Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Option::Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Option::Some(70));
    }
}
