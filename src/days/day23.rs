use std::collections::{BinaryHeap, HashMap};

use crate::{utils::vector_2d::*, Solution, SolutionPair};

struct Map {
    tiles: HashMap<Vector2, u8>,
    start: Vector2,
    end: Vector2,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut tiles = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);

        for (y, line) in input.lines().rev().enumerate() {
            for (x, c) in line.bytes().enumerate() {
                let pos = Vector2::new_usize(x, y);
                tiles.insert(pos, c);
                max_y = max_y.max(y);
                max_x = max_x.max(x);
            }
        }

        let (start, _) = tiles
            .iter()
            .find(|(p, c)| p.y == max_y as isize && **c == b'.')
            .unwrap();

        let (end, _) = tiles.iter().find(|(p, c)| p.y == 0 && **c == b'.').unwrap();

        Self {
            start: *start,
            end: *end,
            tiles,
        }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let _map = Map::parse(input);

    (Solution::None, Solution::None)

    // (p1(&map), p2(&map))
}

fn p1(map: &Map) -> Solution {
    Solution::Usize(dfs(map))
}

fn p2(map: &Map) -> Solution {
    let mut max_distance = 0;
    let mut heap = BinaryHeap::new();
    heap.push((0, map.start, im::HashSet::new()));

    while let Some((steps, current, path)) = heap.pop() {
        if current == map.end {
            max_distance = max_distance.max(steps);
        }

        if path.contains(&current) {
            continue;
        }

        for next in current.adjacent_points() {
            if let Some(&c) = map.tiles.get(&next) {
                if c != b'#' {
                    let new_steps = steps + 1;
                    heap.push((new_steps, next, path.update(current)));
                }
            }
        }
    }

    Solution::Usize(max_distance)
}

fn dfs(map: &Map) -> usize {
    let mut max_distance = 0;
    let mut heap = BinaryHeap::new();
    heap.push((0_usize, map.start, im::HashSet::new()));

    while let Some((steps, current, path)) = heap.pop() {
        if current == map.end {
            max_distance = max_distance.max(steps);
        }

        if path.contains(&current) {
            continue;
        }

        let path = path.update(current);

        match map.tiles.get(&current) {
            None => continue,
            Some(&c) => match c {
                b'#' => unreachable!(),
                b'>' => heap.push((steps + 1, current + E, path.clone())),
                b'<' => heap.push((steps + 1, current + W, path.clone())),
                b'^' => heap.push((steps + 1, current + N, path.clone())),
                b'v' => heap.push((steps + 1, current + S, path.clone())),
                b'.' => {
                    let neighbours = vec![
                        (current + E, b'<'),
                        (current + W, b'>'),
                        (current + S, b'^'),
                        (current + N, b'v'),
                    ];

                    for (next, slope) in neighbours {
                        if let Some(&c) = map.tiles.get(&next) {
                            if c != b'#' && c != slope {
                                heap.push((steps + 1, next, path.clone()));
                            }
                        }
                    }
                }
                _ => unreachable!(),
            },
        }
    }

    max_distance
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day23/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(94));
        assert_eq!(p2, Solution::Usize(154));
    }
}
