use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    (p1(input), p2(input))
}

fn p1(input: &str) -> Solution {
    let answer = input
        .lines()
        .map(parse_line)
        .map(|s| compute_next_in_sequence(&s))
        .sum();

    Solution::Isize(answer)
}

fn p2(input: &str) -> Solution {
    let answer = input
        .lines()
        .map(parse_line)
        .map(|s| extrapolate_backwards(&s))
        .sum();

    Solution::Isize(answer)
}

fn parse_line(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .map(str::parse::<isize>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn compute_next_in_sequence(sequence: &[isize]) -> isize {
    let last_current = *sequence.last().unwrap();
    if all_same(sequence) {
        last_current
    } else {
        let diffs = sequence.windows(2).map(|window| window[1] - window[0]).collect::<Vec<_>>();
        last_current + compute_next_in_sequence(&diffs)
    }
}

fn extrapolate_backwards(sequence: &[isize]) -> isize {
    let first_current = *sequence.first().unwrap();
    if all_same(sequence) {
        first_current
    } else {
        let diffs = sequence.windows(2).map(|window| window[1] - window[0]).collect::<Vec<_>>();
        first_current - extrapolate_backwards(&diffs)
    }
}

fn all_same(diffs: &[isize]) -> bool {
    let first = diffs.first().unwrap();
    diffs.iter().all(|&x| x == *first)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day09/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Isize(114));
        assert_eq!(p2, Solution::Isize(2));
    }
}
