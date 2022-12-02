use core::panic;

pub fn part_one(input: &str) -> Option<u32> {
    let x: i32 = input
        .trim_end()
        .lines()
        .map(|l| l.split(' ').collect())
        .map(|v: Vec<&str>| {
            match v[1] {
                "X" => match v[0] {
                    "A" => 4,
                    "B" => 1,
                    "C" => 7,
                    _ => panic!("not possible")
                },
                "Y" => match v[0] {
                    "A" => 8,
                    "B" => 5,
                    "C" => 2,
                    _ => panic!("not possible")
                },
                "Z" => match v[0] {
                    "A" => 3,
                    "B" => 9,
                    "C" => 6,
                    _ => panic!("not possible")
                },
                _ => panic!("not possible")
            }
        }).sum();

    Option::Some(x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let x: i32 = input
        .trim_end()
        .lines()
        .map(|l| l.split(' ').collect())
        .map(|v: Vec<&str>| {
            match v[0] {
                "A" => match v[1] {
                    "X" => 3,
                    "Y" => 4,
                    "Z" => 8,
                    _ => panic!()
                },
                "B" => match v[1] {
                    "X" => 1,
                    "Y" => 5,
                    "Z" => 9,
                    _ => panic!()
                },
                "C" => match v[1] {
                    "X" => 2,
                    "Y" => 6,
                    "Z" => 7,
                    _ => panic!()
                }
                _ => panic!("not possible")
            }
        }).sum();
    Option::Some(x as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Option::Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Option::Some(12));
    }
}
