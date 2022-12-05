pub fn part_one(input: &str) -> Option<String> {
    let (_stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = vec![
        vec!['F', 'C', 'J', 'P', 'H', 'T', 'W'],
        vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
        vec!['H', 'P', 'T', 'R'],
        vec!['Z', 'S', 'N', 'P', 'H', 'T'],
        vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
        vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
        vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
        vec!['N', 'D', 'S'],
        vec!['D', 'Z', 'S', 'F', 'M']
    ];

    for instruction in instructions.lines() {
        let x = instruction.split(' ').collect::<Vec<&str>>();
        let n = x[1].parse::<usize>().unwrap();
        let from = x[3].parse::<usize>().unwrap() - 1;
        let to = x[5].parse::<usize>().unwrap() - 1;

        let t = (0..n).map(|_| stacks[from].pop().unwrap()).collect::<Vec<char>>();

        for elem in t.iter() {
            stacks[to].push(*elem);
        }
    }

    let phrase: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    Option::Some(phrase)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_stacks, instructions) = input.split_once("\n\n").unwrap();

    // let mut stacks = vec![
    //     vec!['Z', 'N'],
    //     vec!['M', 'C', 'D'],
    //     vec!['P']
    // ];

    let mut stacks = vec![
        vec!['F', 'C', 'J', 'P', 'H', 'T', 'W'],
        vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
        vec!['H', 'P', 'T', 'R'],
        vec!['Z', 'S', 'N', 'P', 'H', 'T'],
        vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
        vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
        vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
        vec!['N', 'D', 'S'],
        vec!['D', 'Z', 'S', 'F', 'M']
    ];

    for instruction in instructions.lines() {
        let x = instruction.split(' ').collect::<Vec<&str>>();
        let n = x[1].parse::<usize>().unwrap();
        let from = x[3].parse::<usize>().unwrap() - 1;
        let to = x[5].parse::<usize>().unwrap() - 1;

        let t = (0..n).map(|_| stacks[from].pop().unwrap()).collect::<Vec<char>>();

        for elem in t.iter().rev() {
            stacks[to].push(*elem);
        }
    }

    let phrase: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    Option::Some(phrase)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
    }
}
