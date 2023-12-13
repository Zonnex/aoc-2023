use std::ops::RangeInclusive;

use crate::{Solution, SolutionPair};

struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|line| {
                let (springs, pattern) = line.split_once(' ').unwrap();
                let springs = springs.to_owned();
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

struct Line {
    springs: String,
    pattern: Vec<usize>,
}

pub fn solve(input: &str) -> SolutionPair {
    (p1(input), p2(input))
}

fn p1(input: &str) -> Solution {
    let input = Input::parse(input);
    let total_arrangements = input
        .lines
        .iter()
        .map(|l| count_arrangements(&l.springs, &l.pattern))
        .sum::<usize>();

    Solution::Usize(total_arrangements)
}

fn count_arrangements(pattern: &str, solution: &[usize]) -> usize {
    let _current = 0;
    let _ranges = initiate_ranges(pattern, solution);
    for _size in solution {}
    // if solution length is 1, this is the last window, count all valid positions to the right
    // if solution length is greater than 1, recurse with smaller solution and substring of remaining pattern
    // after recurse, move to the right until valid, then recurse again
    // make inclusive range of size equal to first usize in solution
    // find first spot in pattern for that range so that all elements matches ? or #
    // surrounding elements must be ., ?, or out of bounds

    /*
    ??##.?????#?? 3,1,2
    [??#]#[.]?[??]??#??
    ?[?##].[?]????#??
    ?[?##].[?]?[??]?#??

    ?[?##].[?]???[?#]?? = 1
    ?[?##].[?]????[#?]? = 1
    ?[?##].?[?]??[?#]?? = 1
    ?[?##].?[?]???[#?]? = 1
    ?[?##].??[?]?[?#]?? = 1
    ?[?##].??[?]??[#?]? = 1
    ?[?##].???[?]?[#?]? = 1

    // all groups satisfied, no # outside groups
    */
    0
}

fn initiate_ranges(pattern: &str, solution: &[usize]) -> Vec<RangeInclusive<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0;
    for size in solution {
        let end = start + size - 1;
        let mut valid_range = false;

        while !valid_range {
            valid_range = pattern[start..=end].chars().all(|c| c == '?' || c == '#')
                && pattern.chars().nth(start - 1) != Some('#')
                && pattern.chars().nth(end + 1) != Some('#');

            start += 1;
        }

        ranges.push(start..=end);
        start = end + 1;
    }
    ranges
}

fn p2(_input: &str) -> Solution {
    Solution::Usize(0)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day12/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(21));
        assert_eq!(p2, Solution::Usize(0));
    }
}
