use std::collections::{HashMap, HashSet, VecDeque};

use crate::{utils::vector_2d::*, Solution, SolutionPair};

type Map = HashMap<Vector2, char>;

pub fn solve(input: &str) -> SolutionPair {
    let map = parse_map(input);

    (p1(&map), p2(&map))
}

fn parse_map(input: &str) -> HashMap<Vector2, char> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Vector2::new_usize(x, y), c);
        }
    }
    map
}

fn p1(map: &Map) -> Solution {
    let start = map
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(pos, _)| *pos)
        .unwrap();

    let starts = get_start_exits(map, start);
    let mut todo = VecDeque::from(starts);
    let mut visits = HashSet::new();
    visits.insert(start);

    while let Some(pos) = todo.pop_front() {
        let c = match map.get(&pos) {
            Some(&c) => c,
            None => continue,
        };

        if !visits.insert(pos) {
            continue;
        }
        
        exits(c)
            .into_iter()
            .map(|d| pos + d)
            .filter(|x| !visits.contains(x))
            .for_each(|x| todo.push_back(x));
    }

    Solution::from(visits.len().div_ceil(2))
}

fn get_start_exits(map: &HashMap<Vector2, char>, start: Vector2) -> [Vector2; 2] {
    const NEWS: [Vector2; 4] = [N, E, W, S];
    let mut exits = [Vector2::new(0, 0); 2];
    for dir in NEWS {
        let next = start + dir;
        if let Some(c) = map.get(&next) {
            if can_visit(dir, c) {
                exits[0] = next;
                exits.rotate_left(1);
            }
        }
    }
    exits
}

fn can_visit(dir: Vector2, next: &char) -> bool {
    match (dir, next) {
        (N, '|') => true,
        (N, 'F') => true,
        (N, '7') => true,
        (E, '-') => true,
        (E, '7') => true,
        (E, 'J') => true,
        (S, '|') => true,
        (S, 'J') => true,
        (S, 'L') => true,
        (W, '-') => true,
        (W, 'F') => true,
        (W, 'L') => true,
        _ => false,
    }
}

fn exits(this: char) -> [Vector2; 2] {
    match this {
        '|' => [N, S],
        '-' => [W, E],
        'L' => [N, E],
        'F' => [S, E],
        '7' => [W, S],
        'J' => [W, N],
        _ => unreachable!("Invalid exit char: {}", this),
    }
}

fn p2(_map: &Map) -> Solution {
    Solution::from(0)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day10/test.txt");
        let (p1, _p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(4));
        // assert_eq!(p2, expected);
    }
}
