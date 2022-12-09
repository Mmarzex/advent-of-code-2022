use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let instructions: Vec<(&str, u32)> = input
        .trim_end()
        .lines()
        .map(|l| {
            let (d, s) = l.split_once(' ').unwrap();
            (d, s.parse::<u32>().unwrap())
        })
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut h: (i32, i32) = (0, 0);
    let mut t: (i32, i32) = (0, 0);

    for (direction, steps) in instructions.iter() {
        let dh: (i32, i32) = match *direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("unreachable"),
        };

        for step in 0..*steps {
            h.0 += dh.0;
            h.1 += dh.1;

            let adj = h.0.abs_diff(t.0) <= 1 && h.1.abs_diff(t.1) <= 1;

            if !adj {
                match t.0.cmp(&h.0) {
                    std::cmp::Ordering::Less => t.0 += 1,
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => t.0 -= 1,
                }

                match t.1.cmp(&h.1) {
                    std::cmp::Ordering::Less => t.1 += 1,
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => t.1 -= 1,
                }

                visited.insert(t.clone());
            }
        }
    }

    Option::Some(visited.len() as u32)
}

fn step(h: (i32, i32), t: &mut (i32, i32)) {
    let adj = h.0.abs_diff(t.0) <= 1 && h.1.abs_diff(t.1) <= 1;

    if !adj {
        match t.0.cmp(&h.0) {
            std::cmp::Ordering::Less => t.0 += 1,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => t.0 -= 1,
        }

        match t.1.cmp(&h.1) {
            std::cmp::Ordering::Less => t.1 += 1,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => t.1 -= 1,
        }

    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions: Vec<(&str, u32)> = input
        .trim_end()
        .lines()
        .map(|l| {
            let (d, s) = l.split_once(' ').unwrap();
            (d, s.parse::<u32>().unwrap())
        })
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut tails = [(0, 0); 9];

    let mut h: (i32, i32) = (0, 0);

    for (direction, steps) in instructions.iter() {
        let dh: (i32, i32) = match *direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("unreachable"),
        };

        for _ in 0..*steps {
            h.0 += dh.0;
            h.1 += dh.1;

            step(h, &mut tails[0]);


            for i in 0..tails.len() - 1 {
                step(tails[i], &mut tails[i + 1]);
            }


            visited.insert(tails[8]);

        }
    }

    Option::Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Option::Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Option::Some(1));
    }
}
