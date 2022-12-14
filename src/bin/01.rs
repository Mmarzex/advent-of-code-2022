pub fn part_one(input: &str) -> Option<u32> {
    let x: Vec<u32> = input
        .trim_end()
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|z| z.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .collect();

    Option::Some(*x.iter().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut x: Vec<u32> = input
        .trim_end()
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|z| z.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .collect();

    x.sort();

    Option::Some(x.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Option::Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Option::Some(45000));
    }
}
