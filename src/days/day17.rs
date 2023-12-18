use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::{utils::vector_2d::*, Solution, SolutionPair};

struct Map {
    grid: HashMap<Vector2, usize>,
    size: Vector2,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut grid = HashMap::new();
        let (mut x_size, mut y_size) = (0, 0);
        for (y, line) in input.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.insert(Vector2::new_usize(x, y), c.to_digit(10).unwrap() as usize);
                x_size = x_size.max(x);
            }
            y_size = y_size.max(y);
        }
        Self {
            grid,
            size: Vector2::new_usize(x_size, y_size),
        }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let map = Map::parse(input);
    (p1(&map), p2(&map))
}

fn p1(map: &Map) -> Solution {
    let start = Vector2::new(0, map.size.y);
    let end = Vector2::new(map.size.x, 0);
    let heat = bfs(map, start, end, 1, 3);
    Solution::Usize(heat)
}

fn p2(map: &Map) -> Solution {
    let start = Vector2::new(0, map.size.y);
    let end = Vector2::new(map.size.x, 0);
    let heat = bfs(map, start, end, 4, 10);
    Solution::Usize(heat)
}
fn bfs(map: &Map, start: Vector2, end: Vector2, min_steps: usize, max_steps: usize) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(Reverse((0_usize, start, 1, E)));

    while let Some(Reverse((heat_acc, current, forward_steps, dir))) = queue.pop() {
        if current == end && forward_steps >= min_steps {
            return heat_acc;
        }

        if !visited.insert((current, dir, forward_steps)) {
            continue;
        }

        if forward_steps < max_steps {
            let next = current + dir;
            if let Some(heat) = map.grid.get(&next) {
                queue.push(Reverse((heat_acc + heat, next, forward_steps + 1, dir)));
            }
        }

        if forward_steps >= min_steps {
            let left = current.left(dir);
            if let Some(&heat) = map.grid.get(&left) {
                let dir = left - current;
                queue.push(Reverse((heat_acc + heat, left, 1, dir)));
            }

            let right = current.right(dir);
            if let Some(&heat) = map.grid.get(&right) {
                let dir = right - current;
                queue.push(Reverse((heat_acc + heat, right, 1, dir)));
            }
        }
    }
    unreachable!("No path found")
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day17/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(102));
        assert_eq!(p2, Solution::Usize(94));
    }
}
