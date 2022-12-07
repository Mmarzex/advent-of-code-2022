use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut current_dir: Vec<&str> = vec![];

    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let mut dirs: Vec<String> = vec![];

    for line in input.trim_end().lines() {
        if line.starts_with("$") {
            let command_parts: Vec<&str> = line.split(' ').collect();
            if command_parts.len() > 2 {
                if command_parts[2] == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(command_parts[2]);
                    let p = current_dir.join("/").replacen("/", "", 1);
                    dir_sizes.insert(p.clone(), 0);
                    dirs.push(p);
                }
            }
        } else {
            let ls_parts: Vec<&str> = line.split(' ').collect();
            if ls_parts[0] != "dir" {
                let file_size = ls_parts[0].parse::<u32>().unwrap();
                let key = current_dir.join("/").replacen("/", "", 1);
                let current_size = dir_sizes[&key];
                dir_sizes.insert(key, current_size + file_size);
            }
        }
    }

    let mut result: u32 = 0;

    for dir in dirs.iter() {
        let total_dir_size: u32 = dirs
            .iter()
            .filter_map(|d| match d.starts_with(dir) {
                true => Some(dir_sizes[d]),
                false => None,
            })
            .sum();
        if total_dir_size <= 100000 {
            result += total_dir_size;
        }
    }
    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut current_dir: Vec<&str> = vec![];

    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let mut dirs: Vec<String> = vec![];

    for line in input.trim_end().lines() {
        if line.starts_with("$") {
            let command_parts: Vec<&str> = line.split(' ').collect();
            if command_parts.len() > 2 {
                if command_parts[2] == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(command_parts[2]);
                    let p = current_dir.join("/").replacen("/", "", 1);
                    dir_sizes.insert(p.clone(), 0);
                    dirs.push(p);
                }
            }
        } else {
            let ls_parts: Vec<&str> = line.split(' ').collect();
            if ls_parts[0] != "dir" {
                let file_size = ls_parts[0].parse::<u32>().unwrap();
                let key = current_dir.join("/").replacen("/", "", 1);
                let current_size = dir_sizes[&key];
                dir_sizes.insert(key, current_size + file_size);
            }
        }
    }

    let root_size: u32 = dir_sizes.values().sum();

    let free_space: u32 = 70000000 - root_size;

    let needed_space: u32 = 30000000 - free_space;

    let mut sizes: Vec<u32> = vec![];

    for dir in dirs.iter() {
        let total_dir_size: u32 = dirs
            .iter()
            .filter_map(|d| match d.starts_with(dir) {
                true => Some(dir_sizes[d]),
                false => None,
            })
            .sum();
        if total_dir_size >= needed_space {
            sizes.push(total_dir_size);
        }
    }
    Option::Some(*sizes.iter().min().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Option::Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Option::Some(24933642));
    }
}
