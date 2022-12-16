use std::ops::RangeInclusive;

use rustc_hash::FxHashMap;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Sensor {
    position: Pos,
    closest_beacon: Pos,
    distance: i32,
}

impl Pos {
    fn parse(line: &str) -> Pos {
        // x=9, y=16
        let l = line.split(", ").map(|ll| {
            let (_v, i) = ll.split_once("=").unwrap();
            i.parse::<i32>().unwrap()
        }).collect::<Vec<i32>>();

        Pos { x: l[0], y: l[1] }
    }
}

impl Sensor {
    fn parse(line: &str) -> Sensor {
        let l = line
            .replace("Sensor at ", "")
            .split(": closest beacon is at")
            .map(|l| Pos::parse(l))
            .collect::<Vec<Pos>>();

        let p = l[0];
        let b = l[1];
        let distance = (p.x.abs_diff(b.x) + p.y.abs_diff(b.y)) as i32;

        Sensor { position: p, closest_beacon: b, distance }
    }

    fn beacon_check(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let yy = self.position.y.abs_diff(y) as i32;

        if yy > self.distance {
            return None
        }

        let d = self.distance - yy;

        Some((self.position.x - d)..=(self.position.x + d))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let sensors = input
        .trim_end()
        .lines()
        .map(|l| Sensor::parse(l))
        .collect::<Vec<Sensor>>();

    let y = if sensors.len() <= 14 { 10 } else { 2000000 };

    let mut scanned = FxHashMap::<i32, u8>::default();

    for sensor in sensors.iter() {
        if let Some(xr) = sensor.beacon_check(y) {
            for x in xr {
                scanned.entry(x).or_insert(b'#');
            }
        }

        if sensor.closest_beacon.y == y {
            scanned.insert(sensor.closest_beacon.x, b'B');
        }
    }

    Option::Some(scanned.iter().filter(|(_k, v)| **v == b'#').count())
}

pub fn part_two(input: &str) -> Option<i64> {
    let sensors = input
        .trim_end()
        .lines()
        .map(|l| Sensor::parse(l))
        .collect::<Vec<Sensor>>();

    let max_y = if sensors.len() <= 14 { 20 } else { 4000000 };

    let found = (0..=max_y).into_par_iter().find_map_any(|y| {
        let xs = sensors.iter().flat_map(|s| s.beacon_check(y)).collect::<Vec<_>>();

        let mut x = 0;
        while x <= max_y {
            if let Some(xr) = xs.iter().find(|xx| xx.contains(&x)) {
                x = xr.end() + 1;
                continue;
            }
            return Some((x, y))
        }
        None
    });

    let (x, y) = found.unwrap();

    Option::Some(((x as i64) * 4000000) + y as i64)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Option::Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Option::Some(56000011));
    }
}
