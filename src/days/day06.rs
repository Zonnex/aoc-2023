use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    (p1(input), p2(input))
}

fn p1(input: &str) -> Solution {
    parse_races(input)
        .into_iter()
        .map(|(time, record)| calculate_margin_of_error(time, record))
        .product::<usize>()
        .into()
}

fn p2(input: &str) -> Solution {
    let values = input
        .replace(" ", "")
        .lines()
        .filter_map(|line| {
            let (_, value) = line.split_once(":").unwrap();
            value.parse::<usize>().ok()
        })
        .collect::<Vec<_>>();

    let time = values[0];
    let record = values[1];

    calculate_margin_of_error(time, record).into()
}

fn parse_races(input: &str) -> Vec<(usize, usize)> {
    fn parse_line(line: &str) -> Vec<usize> {
        let (_, values) = line.split_once(":").unwrap();
        values
            .split_whitespace()
            .into_iter()
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect()
    }
    let times = parse_line(input.lines().next().unwrap());
    let records = parse_line(input.lines().last().unwrap());

    times.into_iter().zip(records.into_iter()).collect()
}

fn calculate_margin_of_error(time: usize, record: usize) -> usize {
    (1..time)
        .into_iter()
        .filter_map(|charge| {
            let remaining = time - charge;
            let distance = charge * remaining;
            if distance > record {
                Some(charge)
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day06/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::USize(288));
        assert_eq!(p2, Solution::USize(71503));
    }
}
