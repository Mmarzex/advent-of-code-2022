use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = input.trim_end().lines().collect::<Vec<&str>>();

    let mut cycle = 1;
    let mut strength = 0;
    let mut x = 1;

    let checks = vec![20, 60, 100, 140, 180, 220];

    let mut strengths: Vec<i32> = vec![];

    for instruction in instructions.iter() {
        if *instruction == "noop" {
            cycle += 1;
            if checks.contains(&cycle) {
                strengths.push(cycle * x);
                strength += cycle * x;
            }
        } else {
            let v = instruction
                .split_once(' ')
                .unwrap()
                .1
                .parse::<i32>()
                .unwrap();

            if checks.contains(&(cycle + 1)) {
                let new_cycle = cycle + 1;
                strengths.push(new_cycle * x);
                strength += new_cycle * x;
            } else if checks.contains(&(cycle + 2)) {
                let new_x = x + v;
                let new_cycle = cycle + 2;
                strengths.push(new_cycle * new_x);
                strength += new_cycle * new_x;
            }
            x += v;
            cycle += 2;
        }
    }

    Option::Some(strength as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = input.trim_end().lines().collect::<Vec<&str>>();

    let mut cycle: usize = 0;
    let mut x: isize = 1;

    let mut rows: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];

    let mut current_row = 0;

    for instruction in instructions.iter() {
        if *instruction == "noop" {
            let p= cycle % 40;
            cycle += 1;

            if p.abs_diff(x as usize) <= 1 {
                rows[current_row][p] = '#';
            }

            if p == 39 {
                current_row += 1;
            }
        } else {
            let v = instruction
                .split_once(' ')
                .unwrap()
                .1
                .parse::<isize>()
                .unwrap();

            for _ in 0..2 {
                let p = cycle % 40;
                cycle += 1;

                if p.abs_diff(x as usize) <= 1 {
                    rows[current_row][p] = '#';
                }

                if p == 39 {
                    current_row += 1;
                }
            }
            x += v;
        }
    }

    Option::Some(rows.iter().map(|r| r.iter().join("")).join("\n"))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Option::Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Option::Some("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....".to_string()));
    }
}
