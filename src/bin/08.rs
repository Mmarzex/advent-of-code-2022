pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .trim_end()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let deltas = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut visible_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if y == 0 || y == grid.len() - 1 || x == 0 || x == grid[0].len() - 1 {
                visible_count += 1;
                continue;
            }
            for (dx, dy) in deltas.iter() {
                let mut xx = (x as i32) + dx;
                let mut yy = (y as i32) + dy;
                let v = grid[y][x];
                let mut is_visible = true;
                while xx >= 0 && yy >= 0 && xx < grid.len() as i32 && yy < grid[0].len() as i32 {
                    if grid[yy as usize][xx as usize] >= v {
                        is_visible = false;
                        break;
                    }

                    xx += dx;
                    yy += dy;
                }
                if is_visible {
                    visible_count += 1;
                    break;
                }
            }
        }
    }

    Option::Some(visible_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .trim_end()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let deltas = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut max_score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut score: u32 = 1;

            for (dx, dy) in deltas.iter() {
                let mut xx = (x as i32) + dx;
                let mut yy = (y as i32) + dy;
                let v = grid[y][x];
                let mut visible_count = 0;
                while xx >= 0 && yy >= 0 && xx < grid.len() as i32 && yy < grid[0].len() as i32 {
                    visible_count += 1;
                    if grid[yy as usize][xx as usize] >= v {
                        break;
                    }

                    xx += dx;
                    yy += dy;
                }
                score = score * visible_count;
            }

            if score > max_score {
                max_score = score;
            }
        }
    }

    Option::Some(max_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Option::Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Option::Some(8));
    }
}
