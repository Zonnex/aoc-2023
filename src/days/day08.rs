use std::collections::{HashMap, VecDeque};

use crate::{Solution, SolutionPair};

pub(super) struct Input<'a> {
    instructions: Vec<u8>,
    network_map: HashMap<&'a str, (&'a str, &'a str)>,
}

pub fn solve(input: &str) -> SolutionPair {
    let input = parse_input(input);

    (p1(&input), p2(&input))
}

pub(super) fn p1(input: &Input) -> Solution {
    let mut current = "AAA";
    let mut steps = 0;
    let iterator = input.instructions.iter().cycle();
    for dir in iterator {
        current = move_to(&input.network_map, current, *dir);
        steps += 1;
        if current == "ZZZ" {
            break;
        }
    }
    Solution::Usize(steps)
}

pub(super) fn p2(input: &Input) -> Solution {
    let mut locations = input
        .network_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| (*k, 1usize))
        .collect::<VecDeque<_>>();

    let mut iterator = input.instructions.iter().cycle();
    let mut path_lengths = Vec::new();
    while !locations.is_empty() {
        let dir = iterator.next().unwrap();
        for _ in 0..locations.len() {
            let (location, steps) = locations.pop_front().unwrap();
            let new = move_to(&input.network_map, location, *dir);
            if new.ends_with('Z') {
                path_lengths.push(steps)
            } else {
                locations.push_back((new, steps + 1));
            }
        }
    }

    Solution::Usize(lcm(path_lengths))
}

fn lcm(mut numbers: Vec<usize>) -> usize {
    numbers.sort();
    let mut lcm = numbers[0];
    for &n in &numbers[1..] {
        lcm = lcm * n / gcd(lcm, n);
    }
    lcm
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn move_to<'a>(map: &HashMap<&'a str, (&'a str, &'a str)>, current: &'a str, dir: u8) -> &'a str {
    let (left, right) = map.get(current).unwrap();
    match dir {
        b'L' => left,
        b'R' => right,
        _ => unreachable!(),
    }
}

pub(super) fn parse_input(input: &str) -> Input {
    let (instructions, map) = input.split_once("\n\n").unwrap();

    let instructions = instructions.chars().map(|c| c as u8).collect::<Vec<_>>();

    let network_map = map
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" = (").unwrap();
            let (left, right) = to.split_once(", ").unwrap();
            let right = right.trim_end_matches(')');
            (from, (left, right))
        })
        .collect::<HashMap<_, _>>();

    Input {
        instructions,
        network_map,
    }
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input_p1 = include_str!("../../input/day08/test_p1.txt");
        let input_p2 = include_str!("../../input/day08/test_p2.txt");
        let input_p1 = super::parse_input(input_p1);
        let input_p2 = super::parse_input(input_p2);
        let (p1, p2) = (super::p1(&input_p1), super::p2(&input_p2));
        assert_eq!(p1, Solution::Usize(6));
        assert_eq!(p2, Solution::Usize(6));
    }
}
