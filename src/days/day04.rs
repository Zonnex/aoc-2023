use std::collections::HashSet;

use crate::{Solution, SolutionPair};

type Scratchcard = (HashSet<u8>, Vec<u8>);

pub fn solve(input: &str) -> SolutionPair {
    let scratchcards = parse_input(input);

    (p1(&scratchcards), p2(&scratchcards))
}

fn parse_input(input: &str) -> Vec<Scratchcard> {
    input
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(": ").unwrap();
            let (winning_numbers, actual) = card.split_once(" | ").unwrap();

            let winning_numbers = winning_numbers
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect::<HashSet<u8>>();

            let actual = actual
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<u8>>();

            (winning_numbers, actual)
        })
        .collect::<Vec<_>>()
}

fn p1(cards: &[Scratchcard]) -> Solution {
    let answer = cards
        .iter()
        .map(check_scratchcard)
        .map(|wins| if wins > 0 { 1 << (wins - 1) } else { 0 })
        .sum();

    Solution::Usize(answer)
}

fn check_scratchcard(card: &Scratchcard) -> usize {
    let (winning_numbers, actual) = card;

    actual
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count()
}

fn p2(input: &Vec<Scratchcard>) -> Solution {
    let winning_numbers_count = input[0].0.len();
    let mut repeats = vec![1; winning_numbers_count];
    let mut total = 0;

    for card in input {
        let repeat = repeats[0];
        repeats[0] = 1;
        repeats.rotate_left(1);

        let wins = check_scratchcard(card);
        total += repeat;
        for i in 0..wins {
            repeats[i] += repeat;
        }
    }

    Solution::Usize(total)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day04/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(13));
        assert_eq!(p2, Solution::Usize(30));
    }
}
