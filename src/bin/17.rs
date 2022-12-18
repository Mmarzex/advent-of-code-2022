use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug)]
enum Rock {
    HorizontalLine,
    Plus,
    Angle,
    VerticalLine,
    Square
}

impl Rock {
    fn as_vec() -> Vec<Rock> {
        vec![Rock::HorizontalLine, Rock::Plus, Rock::Angle, Rock::VerticalLine, Rock::Square]
    }

    fn coords(self) -> Vec<(isize, isize)> {
        match self {
            Rock::HorizontalLine => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Rock::Angle => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::VerticalLine => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Board {
    columns: [FxHashMap<isize, bool>; 7]
}

impl Board {
    fn max_height(&self) -> isize {
        self.columns.iter().map(|c| *c.keys().max().unwrap_or(&-1isize)).max().unwrap_or(-1)
    }

    fn hit_bounds(&self, coord: (isize, isize)) -> bool {
        coord.0 < 0 || coord.0 > 6
    }

    fn hit_bottom(&self, coord: (isize, isize)) -> bool {
        coord.1 < 0 || self.columns[coord.0 as usize].contains_key(&coord.1)
    }

    fn rock_hit_check(&self, rock: Rock, coord: (isize, isize)) -> bool {
        for (rx, ry) in rock.coords().iter() {
            if self.hit_bounds((coord.0 + rx, coord.1 + ry)) || self.hit_bottom((coord.0 + rx, coord.1 + ry)) {
                return true
            }
        }
        false
    }

    fn set_rock(&mut self, rock: Rock, coord: (isize, isize)) {
        for (rx, ry) in rock.coords().iter() {
            self.columns[(coord.0 + rx) as usize].insert(coord.1 + ry, true);
        }
    }

    fn column_heights(&self) -> [isize; 7] {
        let max_height = self.max_height();
        let mut res = [0isize; 7];

        for (i, column) in res.iter_mut().enumerate() {
            let column_height = self.columns[i].keys().max().unwrap_or(&-1isize);
            *column = max_height - column_height;
        }

        res
    }

    fn drop(&mut self, rock: Rock, jet_patterns: &mut impl Iterator<Item = (usize, char)>) {
        let mut rc = (2, self.max_height() + 4);

        loop {
            let jet_offset = match jet_patterns.next() {
                Some((_, '<')) => -1,
                Some((_, '>')) => 1,
                _ => panic!()
            };

            let rc_jet = (rc.0 + jet_offset, rc.1);
            if !self.rock_hit_check(rock, rc_jet) {
                rc = rc_jet;
            }

            let rc_fall = (rc.0, rc.1 - 1);
            if self.rock_hit_check(rock, rc_fall) {
                self.set_rock(rock, rc);
                break;
            }

            rc = rc_fall;
        }
    }
}



pub fn part_one(input: &str) -> Option<isize> {
    let mut jet_patterns = input.trim_end().chars().enumerate().cycle().peekable();
    let rocks = vec![Rock::HorizontalLine, Rock::Plus, Rock::Angle, Rock::VerticalLine, Rock::Square];
    let mut rock_iter = rocks.iter().enumerate().cycle().peekable();

    let max_rocks = 2022;

    let mut board = Board::default();

    for ri in 0..max_rocks {
        println!("{:?}", ri);
        let rock = rock_iter.next().unwrap();
        board.drop(*rock.1, &mut jet_patterns);
    }

    Option::Some(board.max_height() + 1)
}

type CacheKey = (usize, usize, [isize; 7]);

type CacheValue = (usize, isize);

pub fn part_two(input: &str) -> Option<usize> {
    let mut jet_patterns = input.trim_end().chars().enumerate().cycle().peekable();
    let rocks = vec![Rock::HorizontalLine, Rock::Plus, Rock::Angle, Rock::VerticalLine, Rock::Square];
    let mut rock_iter = rocks.iter().enumerate().cycle().peekable();

    let max_rocks = 1000000000000i64;

    let mut board = Board::default();

    let mut cache = FxHashMap::<CacheKey, CacheValue>::default();

    let mut ri = 0i64;

    while ri < max_rocks {
        ri += 1;
        println!("{:?}", ri);
        let rock = rock_iter.next().unwrap();
        board.drop(*rock.1, &mut jet_patterns);

        let key: CacheKey = (jet_patterns.peek().unwrap().0, rock_iter.peek().unwrap().0, board.column_heights());
        let value: CacheValue = (ri as usize, board.max_height());
        if let Some(old_value) = cache.insert(key, value) {
            let l = ri - old_value.0 as i64;
            let cycles_left = (max_rocks - ri) / l;

            ri += cycles_left * l;

            let cycle_height = cycles_left * (value.1 as i64 - old_value.1 as i64);

            for _ in ri..max_rocks {
                board.drop(*rock_iter.next().unwrap().1, &mut jet_patterns);
            }

            return Option::Some((board.max_height() + 1 + cycle_height as isize) as usize);
        }
    }

    Option::Some(board.max_height() as usize + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Option::Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Option::Some(1514285714288));
    }
}
