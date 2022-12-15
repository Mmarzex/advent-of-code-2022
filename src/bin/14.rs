use rustc_hash::FxHashMap;
use std::collections::{HashMap, HashSet};

struct Grid {
    grid: Vec<Vec<u8>>,
    lowest_y: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let coords = input
        .trim_end()
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    let parts = c.split(",").collect::<Vec<&str>>();
                    (
                        parts[0].parse::<usize>().unwrap(),
                        parts[1].parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>();

    // let mut grid: HashMap<(usize, usize), u8> =
    let mut grid: FxHashMap<(usize, usize), u8> = FxHashMap::default();
    let mut lowest_y: usize = 0;

    for line in coords.iter() {
        for i in 0..line.len() - 1 {
            let p1 = line[i];
            let p2 = line[i + 1];
            for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
                for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                    grid.insert((x, y), b'#');
                }
            }

            if p1.1 > lowest_y {
                lowest_y = p1.1;
            }
            if p2.1 > lowest_y {
                lowest_y = p2.1;
            }
        }
    }

    let sand_starting_point: (usize, usize) = (500, 0);
    let mut finished = false;

    let mut sand_count: u32 = 0;
    while !finished {
        let mut x = sand_starting_point.0.clone();
        let mut y = sand_starting_point.1.clone();

        loop {
            if y > lowest_y {
                finished = true;
                break;
            }

            if !grid.contains_key(&(x, y + 1)) {
                // grid.insert((x, y + 1), b'#');
                y += 1;
            } else if !grid.contains_key(&(x - 1, y + 1)) {
                // grid.insert((x - 1, y + 1), b'#');
                x -= 1;
                y += 1;
            } else if !grid.contains_key(&(x + 1, y + 1)) {
                // grid.insert((x + 1, y + 1), b'#');
                x += 1;
                y += 1;
            } else {
                grid.insert((x, y), b'o');
                sand_count += 1;
                break;
            }
        }
    }

    // let sand_count = grid.iter().map(|(_p, v)| *v == b'o').count();
    Option::Some(sand_count as u32)
}

pub fn part_oned(input: &str) -> Option<u32> {
    let coords = input
        .trim_end()
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    let parts = c.split(",").collect::<Vec<&str>>();
                    (
                        parts[0].parse::<usize>().unwrap(),
                        parts[1].parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>();

    let mut grid = vec![vec![b'.'; 600]; 600];

    let mut checked_lowest_point: usize = 0;
    for line in coords.iter() {
        for i in 0..line.len() - 1 {
            let p1 = line[i];
            let p2 = line[i + 1];
            for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
                for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                    grid[x][y] = b'#';
                }
            }

            if p1.1 > checked_lowest_point {
                checked_lowest_point = p1.1;
            }

            if p2.1 > checked_lowest_point {
                checked_lowest_point = p2.1;
            }
        }
    }

    let sand_starting_point: (usize, usize) = (500, 0);

    let mut finished = false;
    let mut sand_count = 0;

    while !finished {
        let mut x = sand_starting_point.0.clone();
        let mut y = sand_starting_point.1.clone();

        loop {
            if y > checked_lowest_point {
                finished = true;
                break;
            }
            if grid[x][y + 1] == b'.' {
                y += 1;
                continue;
            } else if grid[x - 1][y + 1] == b'.' {
                x -= 1;
                y += 1;
                continue;
            } else if grid[x + 1][y + 1] == b'.' {
                x += 1;
                y += 1;
                continue;
            } else {
                sand_count += 1;
                grid[x][y] = b'o';
                break;
            }
        }
    }

    Option::Some(sand_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let coords = input
        .trim_end()
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    let parts = c.split(",").collect::<Vec<&str>>();
                    (
                        parts[0].parse::<usize>().unwrap(),
                        parts[1].parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>();

    let mut grid = vec![vec![b'.'; 800]; 800];

    let mut checked_lowest_point: usize = 0;
    for line in coords.iter() {
        for i in 0..line.len() - 1 {
            let p1 = line[i];
            let p2 = line[i + 1];
            for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
                for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                    grid[x][y] = b'#';
                }
            }

            if p1.1 > checked_lowest_point {
                checked_lowest_point = p1.1;
            }

            if p2.1 > checked_lowest_point {
                checked_lowest_point = p2.1;
            }
        }
    }

    let sand_starting_point: (usize, usize) = (500, 0);

    let mut finished = false;
    let mut sand_count = 0;

    for x in 0..800 {
        grid[x][checked_lowest_point + 2] = b'#';
    }

    while !finished {
        let mut x = sand_starting_point.0.clone();
        let mut y = sand_starting_point.1.clone();

        loop {
            if grid[x][y + 1] == b'.' {
                y += 1;
                continue;
            } else if grid[x - 1][y + 1] == b'.' {
                x -= 1;
                y += 1;
                continue;
            } else if grid[x + 1][y + 1] == b'.' {
                x += 1;
                y += 1;
                continue;
            } else {
                sand_count += 1;
                grid[x][y] = b'o';
                break;
            }
        }

        if grid[sand_starting_point.0][sand_starting_point.1] == b'o' {
            break;
        }
    }

    Option::Some(sand_count as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Option::Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Option::Some(93));
    }
}
