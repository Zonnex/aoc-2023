use std::collections::HashMap;

use crate::{Solution, SolutionPair};


type Memo = HashMap<(Vec<u8>, Vec<usize>), usize>;

#[derive(Clone, Debug)]
struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|line| {
                let (springs, pattern) = line.split_once(' ').unwrap();
                let springs = springs.bytes().collect::<Vec<_>>();
                let pattern = pattern
                    .split(',')
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<_>>();
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
        self.pattern
            .iter()
            .filter(|&&b| b == b'#' || b == b'?')
            .count()
            < self.solution.iter().sum::<usize>()
    }

    fn invalid(&self) -> bool {
        if self.pattern_is_empty_but_solution_is_not() {
            return true;
        }
        if self.solution_is_empty_but_pattern_contains_hash() {
            return true;
        }
        if self.pattern_does_not_have_enough_candidates() {
            return true;
        }

        return false;
    }
}


pub fn solve(input: &str) -> SolutionPair {
    let input = Input::parse(input);
    (p1(input.clone()), p2(input))
}

fn p1(mut input: Input) -> Solution {
    let mut memo = Memo::new();
    let total_arrangements = input
        .lines
        .iter_mut()
        .map(|l| count_arrangements(&mut l.springs, &mut l.pattern, &mut memo))
        .sum::<usize>();

    Solution::Usize(total_arrangements)
}
fn p2(mut input: Input) -> Solution {
    input.lines.iter_mut().for_each(|l| {
        l.springs = l.springs.repeat(5);
        l.pattern = l.pattern.repeat(5);
    });

    let mut memo = Memo::new();

    let total_arrangements = input
        .lines
        .iter_mut()
        .map(|l| count_arrangements(&mut l.springs, &mut l.pattern, &mut memo))
        .inspect(|f| println!("{}", f))
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
        memo.insert(key, 0);
        return 0;
    }
    if solution.is_empty() && pattern.iter().filter(|&&b| b != b'#').count() == 0 {
        memo.insert(key, 1);
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
        Some(&b'#') => pound(pattern, solution, memo),
        Some(&b'.') => dot(pattern, solution, memo),
        Some(&b'?') => dot(pattern, solution, memo) + pound(pattern, solution, memo),
        _ => unreachable!("Invalid input"),
    }
}



#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day12/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(21));
        assert_eq!(p2, Solution::Usize(525152));
    }
}
