use std::collections::VecDeque;

use itertools::Itertools;

fn bfs(grid: &Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut q = VecDeque::new();
    q.push_back((start, 0 as usize));

    loop {
        let ((x, y), l) = q.pop_front()?;

        if (x, y) == goal {
            return Some(l);
        }

        for (dx, dy) in [(-1 as isize, 0), (0, -1 as isize), (1, 0), (0, 1)] {
            let (xx, yy) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            let Some(&el) = grid.get(xx).and_then(|f| f.get(yy)) else { continue };

            if grid[x][y] + 1 >= el && !visited[xx][yy] {
                visited[xx][yy] = true;
                q.push_back(((xx, yy), l + 1));
            }
        }

    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = input.trim_end()
        .lines()
        .map(|l| l.as_bytes().iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'S')
        .unwrap();
    let goal= (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'E')
        .unwrap();
    grid[start.0][start.1] = b'a';
    grid[goal.0][goal.1] = b'z';

    bfs(&grid, start, goal)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = input.trim_end()
        .lines()
        .map(|l| l.as_bytes().iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'S')
        .unwrap();
    let goal= (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'E')
        .unwrap();
    grid[start.0][start.1] = b'a';
    grid[goal.0][goal.1] = b'z';

    let mut res: usize = 999999;
    for (x, row) in grid.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == b'a' {
                if let Some(path) = bfs(&grid, (x, y), goal) {
                    res = res.min(path);
                }
            }
        }
    }

    Option::Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Option::Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Option::Some(29));
    }
}
