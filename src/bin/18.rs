use std::collections::VecDeque;

use rustc_hash::FxHashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let coords = input.trim_end().lines().map(|l| {
        l.split(",")
            .map(|ll| ll.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
    }).map(|l| (l[0], l[1], l[2])).collect::<Vec<(isize, isize, isize)>>();

    let coords_set = FxHashSet::from_iter(coords.iter());

    let d = vec![(0isize, 0isize, 1isize), (0, 1, 0), (1, 0, 0), (0, 0, -1), (0, -1, 0), (-1, 0, 0)];

    let mut open_sides = 0;
    for (x, y, z) in coords.iter() {
        for (dx, dy, dz) in d.iter() {
            if !coords_set.contains(&(x + dx, y + dy, z + dz)) {
                open_sides += 1;
            }
        }
    }

    Option::Some(open_sides as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let coords = input.trim_end().lines().map(|l| {
        l.split(",")
            .map(|ll| ll.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
    }).map(|l| (l[0], l[1], l[2])).collect::<Vec<(isize, isize, isize)>>();

    let mut coords_set = FxHashSet::from_iter(coords.iter().map(|l| *l));

    let mut open_coords: FxHashSet<(isize, isize, isize)> = FxHashSet::default();

    let d = vec![(0isize, 0isize, 1isize), (0, 1, 0), (1, 0, 0), (0, 0, -1), (0, -1, 0), (-1, 0, 0)];

    let mut open_sides = 0;
    for (x, y, z) in coords.iter() {
        for (dx, dy, dz) in d.iter() {
            if !coords_set.contains(&(x + dx, y + dy, z + dz)) {
                open_sides += 1;
                open_coords.insert((x + dx, y + dy, z + dz));
            }
        }
    }

    let min_x = coords.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_x = coords.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = coords.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_y = coords.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_z = coords.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap().2;
    let max_z = coords.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap().2;

    for coord in open_coords.iter() {
        let mut q = VecDeque::<(isize, isize, isize)>::new();
        let mut visited = FxHashSet::<(isize, isize, isize)>::default();
        visited.insert(*coord);
        let mut interior = true;
        q.push_back(*coord);
        'fillloop: while !q.is_empty() {
            let (x, y, z) = q.pop_front().unwrap();
            for (dx, dy, dz) in d.iter() {
                let p = (x + dx, y + dy, z + dz);
                if p.0 < min_x || p.0 > max_x || p.1 < min_y || p.1 > max_y || p.2 < min_z || p.2 > max_z {

                    interior = false;
                    break 'fillloop;
                }

                if coords_set.contains(&p) {
                    continue;
                }

                if !visited.contains(&p) {
                    visited.insert(p);
                    q.push_back(p);
                }
            }
        }

        if interior {
            coords_set = coords_set.union(&visited).map(|l| *l).collect::<FxHashSet<_>>();
        }
    }

    let mut sides = 0;

    for (x, y, z) in coords.iter() {
        for (dx, dy, dz) in d.iter() {
            if !coords_set.contains(&(x + dx, y + dy, z + dz)) {
                sides += 1;
            }
        }
    }
    Option::Some(sides as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Option::Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Option::Some(58));
    }
}
