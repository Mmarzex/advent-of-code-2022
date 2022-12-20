use std::collections::VecDeque;

use nom::bytes::complete::tag;
use nom::IResult;
use nom::{
    character::complete::i32,
    combinator::{all_consuming, map},
    sequence::tuple,
};
use rustc_hash::FxHashSet;
use rayon::prelude::*;


#[derive(Clone, Copy, Debug)]
struct BluePrint {
    id: i32,
    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: (i32, i32),
    geode_robot: (i32, i32),
    max_ore_cost: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    ore: i32,
    clay: i32,
    obisidian: i32,
    geode: i32,
    ore_robot: i32,
    clay_robot: i32,
    obisidian_robot: i32,
    geode_robot: i32,
    tick: i32,
}

impl State {
    pub fn new() -> State {
        State { ore: 0, clay: 0, obisidian: 0, geode: 0, ore_robot: 1, clay_robot: 0, obisidian_robot: 0, geode_robot: 0, tick: 0 }
    }

    pub fn update(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obisidian += self.obisidian_robot;
        self.geode += self.geode_robot;
        self.tick += 1;
    }
}

fn parse(line: &str) -> IResult<&str, BluePrint> {
    map(
        all_consuming(tuple((
            tag("Blueprint "),
            i32,
            tag(": Each ore robot costs "),
            i32,
            tag(" ore. Each clay robot costs "),
            i32,
            tag(" ore. Each obsidian robot costs "),
            i32,
            tag(" ore and "),
            i32,
            tag(" clay. Each geode robot costs "),
            i32,
            tag(" ore and "),
            i32,
            tag(" obsidian."),
        ))),
        |(
            _,
            id,
            _,
            ore,
            _,
            clay,
            _,
            obsidian_ore,
            _,
            obsidian_clay,
            _,
            geode_ore,
            _,
            geode_obsidian,
            _,
        )| {
            BluePrint {
                id,
                ore_robot: ore,
                clay_robot: clay,
                obsidian_robot: (obsidian_ore, obsidian_clay),
                geode_robot: (geode_ore, geode_obsidian),
                max_ore_cost: *[ore, clay, obsidian_ore, geode_ore].iter().max().unwrap()
            }
        },
    )(line)
}

fn bfs(blueprint: &BluePrint, tick: i32) -> i32 {
    let mut visited = FxHashSet::<State>::default();

    let mut q = VecDeque::new();

    let starting_state = State::new();
    let mut best: i32= 0;

    q.push_back(starting_state);

    while let Some(mut state) = q.pop_front() {
        best = best.max(state.geode);

        let mut cache_state = state;
        cache_state.tick = 0;

        if state.geode < best - 1 || state.tick == tick || visited.contains(&cache_state) {
            continue;
        }

        visited.insert(cache_state);


        if state.ore >= blueprint.geode_robot.0 && state.obisidian >= blueprint.geode_robot.1 {
            let mut next_state = state;
            next_state.ore -= blueprint.geode_robot.0;
            next_state.obisidian -= blueprint.geode_robot.1;
            next_state.update();
            next_state.geode_robot += 1;
            q.push_back(next_state);
        } else {
            if state.ore >= blueprint.ore_robot && state.ore_robot < blueprint.max_ore_cost {
                let mut next_state = state;
                next_state.ore -= blueprint.ore_robot;
                next_state.update();
                next_state.ore_robot += 1;
                q.push_back(next_state);
            }

            if state.ore >= blueprint.clay_robot && state.clay_robot < blueprint.obsidian_robot.1 {
                let mut next_state = state;
                next_state.ore -= blueprint.clay_robot;
                next_state.update();
                next_state.clay_robot += 1;
                q.push_back(next_state);
            }

            if state.ore >= blueprint.obsidian_robot.0 && state.clay >= blueprint.obsidian_robot.1 {
                let mut next_state = state;
                next_state.ore -= blueprint.obsidian_robot.0;
                next_state.clay -= blueprint.obsidian_robot.1;
                next_state.update();
                next_state.obisidian_robot += 1;
                q.push_back(next_state);
            }

            state.update();
            q.push_back(state);
        }

    }

    best
}

pub fn part_one(input: &str) -> Option<i32> {
    let blueprints = input
        .trim_end()
        .lines()
        .map(|line| parse(line).unwrap().1)
        .collect::<Vec<BluePrint>>();

    let res: i32 = blueprints.par_iter().map(|b| (b.id as i32) * bfs(b, 24)).sum();


    Option::Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let blueprints = input
        .trim_end()
        .lines()
        .map(|line| parse(line).unwrap().1)
        .take(3)
        .collect::<Vec<BluePrint>>();

    let res: i32 = blueprints.par_iter().map(|b| bfs(b, 32)).product();

    Option::Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Option::Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Option::Some(2852));
    }
}
