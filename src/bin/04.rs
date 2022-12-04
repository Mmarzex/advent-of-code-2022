pub fn part_one(input: &str) -> Option<u32> {
    let x = input.trim_end().lines().map(|line| {
        let pairs = line.split(',').collect::<Vec<&str>>();
        let left = pairs[0].split('-').map(|p| p.parse().unwrap()).collect::<Vec<u32>>();
        let right = pairs[1].split('-').map(|p| p.parse().unwrap()).collect::<Vec<u32>>();

        if is_in_range(&left, &right) || is_in_range(&right, &left) {
            return Option::Some(1)
        }
        None
    }).filter(|o| o.is_some())
    .count();

    Option::Some(x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {

    let x = input.trim_end().lines().map(|line| {
        let pairs = line.split(',').collect::<Vec<&str>>();
        let left = pairs[0].split('-').map(|p| p.parse().unwrap()).collect::<Vec<u32>>();
        let right = pairs[1].split('-').map(|p| p.parse().unwrap()).collect::<Vec<u32>>();

        // let one = left[0]..(left[1] + 1);
        // let two = right[0]..(right[1] + 1);

        overlaps(&left, &right)
    }).filter(|l| *l).count();

    Option::Some(x as u32)
}

fn is_in_range(one: &Vec<u32>, two: &Vec<u32>) -> bool {
    one[0] <= two[0] && one[1] >= two[1]
}

fn overlaps(one: &Vec<u32>, two: &Vec<u32>) -> bool {
    let r1 = one[0]..(one[1] + 1);
    let r2 = two[0]..(two[1] + 1);

    r1.contains(&two[0]) || r1.contains(&two[1]) || r2.contains(&one[0]) || r2.contains(&one[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Option::Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Option::Some(4));
    }
}
