use std::collections::HashMap;

use crate::{utils::vector_2d::Vector2, Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    let map = Map::parse(input);
    (p1(&map), p2(&map))
}

fn p1(map: &Map) -> Solution {
    compute_distances(map, 2)
}

fn p2(map: &Map) -> Solution {
    compute_distances(map, 1_000_000)
}

fn compute_distances(map: &Map, size: usize) -> Solution {
    let pairs = pairs(&map.galaxies);
    let sum = pairs
        .iter()
        .map(|(a, b)| {
            let distance = a.distance_to(*b);
            distance + map.extra_distance_between(*a, *b, size)
        })
        .sum();

    Solution::Usize(sum)
}

fn pairs(galaxies: &[Vector2]) -> Vec<(Vector2, Vector2)> {
    let mut pairs = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            pairs.push((galaxies[i], galaxies[j]));
        }
    }
    pairs
}

#[derive(Debug)]
struct Map {
    galaxies: Vec<Vector2>,
}
impl Map {
    fn parse(input: &str) -> Self {
        let mut map = HashMap::new();
        let mut galaxies = Vec::new();

        for (y, line) in input.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vector2::new_usize(x, y);
                let val = match c {
                    '.' => b'.',
                    '#' => {
                        galaxies.push(pos);
                        b'#'
                    }
                    _ => panic!("Invalid input"),
                };
                map.insert(pos, val);
            }
        }
        Map { galaxies }
    }

    fn extra_distance_between(&self, a: Vector2, b: Vector2, size: usize) -> usize {
        let (min_y, max_y) = (a.y.min(b.y), a.y.max(b.y));
        let (min_x, max_x) = (a.x.min(b.x), a.x.max(b.x));

        let mut extra_y = 0;
        for row in min_y + 1..max_y {
            if self.galaxies.iter().any(|g| g.y == row) {
                continue;
            } else {
                extra_y += size - 1;
            }
        }

        let mut extra_x = 0;
        for column in min_x + 1..max_x {
            if self.galaxies.iter().any(|g| g.x == column) {
                continue;
            } else {
                extra_x += size - 1;
            }
        }

        extra_x + extra_y
    }
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day11/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(374));
        assert_eq!(p2, Solution::Usize(82000210));
    }
}
