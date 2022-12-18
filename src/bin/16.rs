use rustc_hash::FxHashMap;

type Room = (String, u32, Vec<String>);

fn parse(line: &str) -> Room {
    let (a, b) = line.split_once("; ").unwrap();
    let aa = a.replace("Valve ", "");
    let (valve, a2) = aa.split_once(" has flow rate=").unwrap();
    let flow_rate = a2.parse::<u32>().unwrap();

    let connected = if b.contains("tunnels") {
        b.replace("tunnels lead to valves ", "")
            .split(", ")
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
    } else {
        b.replace("tunnel leads to valve ", "")
            .split(", ")
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
    };

    (valve.to_string(), flow_rate, connected)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rooms: Vec<Room> = input.trim_end().lines().map(|l| parse(l)).collect();

    rooms.sort_by(|a, b| b.1.cmp(&a.1));

    let idx_lookup = rooms
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0.clone(), i))
        .collect::<FxHashMap<String, usize>>();

    let none_zero_flow = rooms.iter().filter(|v| v.1 > 0).count();
    let n = rooms.len();

    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u32; n];

    for v in rooms.iter() {
        let idx = idx_lookup[&v.0];
        flow[idx] = v.1;
        for valve in v.2.iter() {
            adj[idx].push(idx_lookup[valve]);
        }
    }

    let aa = idx_lookup["AA"];

    let m = 1 << none_zero_flow;

    let mut opt = vec![vec![vec![0; m]; n]; 30];

    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..m {
                let mut v = opt[t][i][x];
                if ii & x != 0 && t >= 2 {
                    v = v.max(opt[t - 1][i][x - ii] + flow[i] * t as u32);
                }

                for &j in adj[i].iter() {
                    v = v.max(opt[t - 1][j][x]);
                }

                opt[t][i][x] = v;
            }
        }
    }
    Option::Some(opt[29][aa][m - 1])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rooms: Vec<Room> = input.trim_end().lines().map(|l| parse(l)).collect();

    rooms.sort_by(|a, b| b.1.cmp(&a.1));

    let idx_lookup = rooms
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0.clone(), i))
        .collect::<FxHashMap<String, usize>>();

    let none_zero_flow = rooms.iter().filter(|v| v.1 > 0).count();
    let n = rooms.len();

    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u32; n];

    for v in rooms.iter() {
        let idx = idx_lookup[&v.0];
        flow[idx] = v.1;
        for valve in v.2.iter() {
            adj[idx].push(idx_lookup[valve]);
        }
    }

    let aa = idx_lookup["AA"];

    let m = 1 << none_zero_flow;

    let mut opt = vec![vec![vec![0; m]; n]; 30];

    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..m {
                let mut v = opt[t][i][x];
                if ii & x != 0 && t >= 2 {
                    v = v.max(opt[t - 1][i][x - ii] + flow[i] * t as u32);
                }

                for &j in adj[i].iter() {
                    v = v.max(opt[t - 1][j][x]);
                }

                opt[t][i][x] = v;
            }
        }
    }

    let mut max: u32 = 0;
    for x in 0..m {
        for y in 0..x {
            let z = x & y;
            if z == 0 {
                max = max.max(opt[25][aa][x] + opt[25][aa][y]);
            }
        }
    }

    Option::Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Option::Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Option::Some(1707));
    }
}
