use std::collections::{HashMap, HashSet, VecDeque};

use crate::{utils::vector_2d::Vector2, Solution, SolutionPair};

struct Map {
    grid: HashMap<Vector2, u8>,
    width: usize,
    height: usize,
}
impl Map {
    fn parse(input: &str) -> Self {
        let (mut width, mut height) = (0, 0);
        let mut grid = HashMap::new();
        for (y, line) in input.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.insert(Vector2::new_usize(x, y), c as u8);
                width = width.max(x);
                height = height.max(y);
            }
        }
        Self {
            grid,
            width: width + 1,
            height: height + 1,
        }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let map = Map::parse(input);
    let (start, _) = map.grid.iter().find(|(_, &v)| v == b'S').unwrap();
    let mut queue = VecDeque::<(usize, Vector2)>::new();
    let mut visited = HashMap::new();
    queue.push_back((0, *start));

    while let Some((dist, current)) = queue.pop_front() {
        if visited.contains_key(&current) {
            continue;
        }

        visited.insert(current, dist);

        for next in current.adjacent_points() {
            if let Some(c) = map.grid.get(&next) {
                if *c != b'#' {
                    queue.push_back((dist + 1, next));
                }
            }
        }
    }

    let p1 = visited
        .values()
        .filter(|v| **v <= 64 && **v % 2 == 0)
        .count();

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();

    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let n = ((26501365 - (map.width / 2)) / map.height) as usize;
    assert_eq!(n, 202300);

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let p2 = odd * visited.values().filter(|v| **v % 2 == 1).count()
        + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);

    (Solution::Usize(p1), Solution::Usize(p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day21/test.txt");
        let map = Map::parse(input);
    }
}
