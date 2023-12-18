use std::collections::HashMap;

use crate::{Solution, SolutionPair};

type Memo = HashMap<(Vec<u8>, Vec<usize>), usize>;

#[derive(Clone, Debug)]
struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn parse(input: &str, repeats: usize) -> Self {
        let lines = input
            .lines()
            .map(|line| {
                let (springs, pattern) = line.split_once(' ').unwrap();
                let springs = std::iter::repeat(springs)
                    .take(repeats)
                    .collect::<Vec<_>>()
                    .join("?")
                    .bytes()
                    .collect::<Vec<_>>();

                let pattern = pattern
                    .split(',')
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<_>>()
                    .repeat(repeats);

                Line { springs, pattern }
            })
            .collect();
        Self { lines }
    }
}

#[derive(Clone, Debug)]
struct Line {
    springs: Vec<u8>,
    pattern: Vec<usize>,
}

struct Rules<'a> {
    pattern: &'a [u8],
    solution: &'a [usize],
}

impl<'a> Rules<'a> {
    fn pattern_is_empty_but_solution_is_not(&self) -> bool {
        self.pattern.is_empty() && !self.solution.is_empty()
    }
    fn solution_is_empty_but_pattern_contains_hash(&self) -> bool {
        self.solution.is_empty() && self.pattern.iter().any(|&b| b == b'#')
    }
    fn pattern_does_not_have_enough_candidates(&self) -> bool {
        let candidates = self
            .pattern
            .iter()
            .filter(|&&b| b == b'#' || b == b'?')
            .count();

        candidates < self.solution.iter().sum::<usize>()
    }

    fn pattern_too_short(&self) -> bool {
        if self.solution.len() > 1 {
            let min_length = self.solution.iter().sum::<usize>() + self.solution.len() - 1;
            self.pattern.len() < min_length
        } else {
            false
        }
    }

    fn invalid(&self) -> bool {
        if self.pattern_too_short() {
            println!(
                "[PATTERN TOO SHORT] pattern: {:?}, solution: {:?}",
                pattern_collect(self.pattern),
                self.solution
            );
            return true;
        }
        if self.pattern_is_empty_but_solution_is_not() {
            println!(
                "[NOT COMPLETE] pattern: {:?}, solution: {:?}",
                pattern_collect(self.pattern),
                self.solution
            );
            return true;
        }
        if self.solution_is_empty_but_pattern_contains_hash() {
            println!(
                "[PARTS REMAIN] pattern: {:?}, solution: {:?}",
                pattern_collect(self.pattern),
                self.solution
            );
            return true;
        }
        if self.pattern_does_not_have_enough_candidates() {
            println!(
                "[TOO FEW CANDIDATES] pattern: {:?}, solution: {:?}",
                pattern_collect(self.pattern),
                self.solution
            );
            return true;
        }

        false
    }
}

pub fn solve(input: &str) -> SolutionPair {
    (p1(input), p2(input))
}

fn p1(input: &str) -> Solution {
    let mut input = Input::parse(input, 1);
    let total_arrangements = input
        .lines
        .iter_mut()
        .map(|l| {
            let mut memo = Memo::new();
            count_arrangements(&mut l.springs, &mut l.pattern, &mut memo)
        })
        .sum::<usize>();

    Solution::Usize(total_arrangements)
}

fn p2(input: &str) -> Solution {
    let mut input = Input::parse(input, 5);

    let total_arrangements = input
        .lines
        .iter_mut()
        .inspect(|l| print!("pattern: {:?} -> ", pattern_collect(&l.springs)))
        .map(|l| {
            let mut memo = Memo::new();
            count_arrangements(&mut l.springs, &mut l.pattern, &mut memo)
        })
        .inspect(|l| {
            println!("{}", l);
        })
        .sum::<usize>();

    Solution::Usize(total_arrangements)
}

fn count_arrangements(pattern: &mut [u8], solution: &mut [usize], memo: &mut Memo) -> usize {
    let key = (pattern.to_vec(), solution.to_vec());
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let rules = Rules { pattern, solution };

    if rules.invalid() {
        return 0;
    }
    if solution.is_empty() && pattern.iter().filter(|&&b| b != b'#').count() == 0 {
        return 1;
    }

    fn dot(pattern: &mut [u8], solution: &mut [usize], memo: &mut Memo) -> usize {
        count_arrangements(&mut pattern[1..], solution, memo)
    }

    fn pound(pattern: &mut [u8], solution: &mut [usize], memo: &mut Memo) -> usize {
        match solution.first() {
            Some(&size) => {
                if pattern.len() < size || pattern.iter().take(size).any(|&b| b == b'.') {
                    return 0;
                }
                if pattern.len() == size {
                    if solution.len() == 1 {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                if let Some(&b'?') | Some(&b'.') = pattern.get(size) {
                    count_arrangements(&mut pattern[size + 1..], &mut solution[1..], memo)
                } else {
                    0
                }
            }
            None => 0,
        }
    }

    match pattern.first() {
        Some(&b'#') => {
            let res = pound(pattern, solution, memo);
            memo.insert(key, res);
            res
        }
        Some(&b'.') => {
            let res = dot(pattern, solution, memo);
            memo.insert(key, res);
            res
        }
        Some(&b'?') => {
            let dot_solutions = dot(pattern, solution, memo);
            let pound_solutions = pound(pattern, solution, memo);
            dot_solutions + pound_solutions
        }
        _ => unreachable!("Invalid input"),
    }
}

fn pattern_collect(pattern: &[u8]) -> String {
    pattern
        .iter()
        .map(|&b| match b {
            b'#' => '#',
            b'.' => '.',
            b'?' => '?',
            _ => unreachable!("Invalid input"),
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::etc::Solution;
    type Memo = HashMap<(Vec<u8>, Vec<usize>), usize>;

    #[test]
    fn test_pattern_1() {
        let mut pattern = std::iter::repeat("???.###").take(5).join("?").bytes().collect::<Vec<_>>();
        let mut solution = [1, 1, 3].repeat(5);
        let mut memo = Memo::new();
        assert_eq!(
            super::count_arrangements(&mut pattern, &mut solution, &mut memo),
            1
        );
    }

    #[test]
    fn test_pattern_2() {
        let mut pattern = std::iter::repeat("???.###").take(5).join("?").bytes().collect::<Vec<_>>();
        let mut solution = [1, 1, 3].repeat(5);
        let mut memo = Memo::new();
        assert_eq!(
            super::count_arrangements(&mut pattern, &mut solution, &mut memo),
            16384
        );
    }

    #[test]
    fn test_pattern_3() {
        let mut pattern = std::iter::repeat("?#?#?#?#?#?#?#?").take(5).join("?").bytes().collect::<Vec<_>>();
        let mut solution = [1, 3, 1, 6].repeat(5);
        let mut memo = Memo::new();
        assert_eq!(
            super::count_arrangements(&mut pattern, &mut solution, &mut memo),
            1
        );
    }

    #[test]
    fn test_pattern_4() {
        let mut pattern = std::iter::repeat("????.#...#...").take(5).join("?").bytes().collect::<Vec<_>>();
        let mut solution = [4, 1, 1].repeat(5);
        let mut memo = Memo::new();
        assert_eq!(
            super::count_arrangements(&mut pattern, &mut solution, &mut memo),
            16
        );
    }

    #[test]
    fn test_pattern_5() {
        let mut pattern = std::iter::repeat("????.######..#####.").take(5).join("?").bytes().collect::<Vec<_>>();
        let mut solution = [1, 6, 5].repeat(5);
        let mut memo = Memo::new();
        assert_eq!(
            super::count_arrangements(&mut pattern, &mut solution, &mut memo),
            2500
        );
    }

    #[test]
    fn test_pattern_6() {
        let mut pattern = std::iter::repeat("?###????????").take(5).join("?").bytes().collect::<Vec<_>>();
        let mut solution = [3, 2, 1].repeat(5);
        let mut memo = Memo::new();
        assert_eq!(
            super::count_arrangements(&mut pattern, &mut solution, &mut memo),
            506250
        );
    }

    #[test]
    fn test_pattern_7() {
        let mut pattern = std::iter::repeat("???#??????????????").take(5).join("?").bytes().collect::<Vec<_>>();
        let mut solution = [1,2,1,4,1].repeat(5);
        let mut memo = Memo::new();
        let res = super::count_arrangements(&mut pattern, &mut solution, &mut memo);
        println!("{}", res);
    }

    #[test]
    fn test_pattern_8() {
        let mut pattern = std::iter::repeat("???#???##.#?").take(5).join("?").bytes().collect::<Vec<_>>();
        let mut solution = [5,2,2].repeat(5);
        let mut memo = Memo::new();
        let res = super::count_arrangements(&mut pattern, &mut solution, &mut memo);
        println!("{}", res);
    }

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day12/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(21));
        assert_eq!(p2, Solution::Usize(525152));
    }
}
