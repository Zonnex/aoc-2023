use std::collections::HashMap;

use crate::{Solution, SolutionPair};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Map {
    tiles: Vec<Vec<u8>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .rev()
            .map(|line| line.bytes().collect())
            .collect();

        Self { tiles }
    }

    fn tilt_up(&mut self) {
        let width = self.tiles[0].len();
        let height = self.tiles.len();

        for col in 0..width {
            let mut spaces = 0;
            for row in (0..height).rev() {
                match self.tiles[row][col] {
                    b'O' => {
                        if spaces > 0 {
                            self.tiles[row][col] = b'.';
                            self.tiles[row + spaces][col] = b'O';
                        }
                    }
                    b'#' => {
                        spaces = 0;
                    }
                    b'.' => {
                        spaces += 1;
                    }
                    _ => panic!("Invalid character in map"),
                }
            }
        }
    }

    // https://www.geeksforgeeks.org/inplace-rotate-square-matrix-by-90-degrees/
    fn rotate_clockwise(&mut self) {
        let n = self.tiles.len();
        for layer in 0..n / 2 {
            for index in layer..n - layer - 1 {
                // Swap in a cycle: bottom_left -> bottom_right -> top_right -> top_left
                let temp = self.tiles[layer][index];
                self.tiles[layer][index] = self.tiles[index][n - 1 - layer];
                self.tiles[index][n - 1 - layer] = self.tiles[n - 1 - layer][n - 1 - index];
                self.tiles[n - 1 - layer][n - 1 - index] = self.tiles[n - 1 - index][layer];
                self.tiles[n - 1 - index][layer] = temp;
            }
        }
    }

    fn cycle(&mut self) -> usize {
        for _ in 0..4 {
            self.tilt_up();
            self.rotate_clockwise();
        }
        self.count()
    }

    fn count(&self) -> usize {
        self.tiles.iter().enumerate().fold(0, |acc, (row, line)| {
            acc + (line.iter().filter(|c| c == &&b'O').count() * (row + 1))
        })
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let map = Map::parse(input);
    (p1(map.clone()), p2(map))
}

fn p1(mut map: Map) -> Solution {
    map.tilt_up();

    Solution::Usize(map.count())
}

fn p2(mut map: Map) -> Solution {
    let cycles = 1_000_000_000;
    let mut cache = HashMap::new();
    for cycle in 1..cycles {
        let result = map.cycle();
        if let Some(last_seen) = cache.insert(map.clone(), cycle) {
            let remaining = cycles - cycle;
            let cycle_length = cycle - last_seen;
            if remaining % cycle_length == 0 {
                return Solution::Usize(result);
            }
        }
    }
    unreachable!("No solution found")
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day14/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(136));
        assert_eq!(p2, Solution::Usize(64));
    }
}
