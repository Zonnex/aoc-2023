use crate::{Solution, SolutionPair};

pub mod camel_cards;

#[derive(Debug, Eq)]
struct Line(camel_cards::Hand, camel_cards::Bid);

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let mut lines = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let hand = camel_cards::Hand::parse(hand);
            let bid = camel_cards::Bid(bid.parse().unwrap());
            Line(hand, bid)
        })
        .collect::<Vec<_>>();

    lines.sort();

    (p1(&lines), p2(input))
}

fn p1(input: &[Line]) -> Solution {
    let winnings = input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, Line(_, camel_cards::Bid(bid)))| {
            let rank = i + 1;
            let bid = *bid as usize;
            rank * bid
        })
        .sum::<usize>();

    Solution::USize(winnings)
}

fn p2(input: &str) -> Solution {
    Solution::U64(0)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day07/test.txt");
        let (p1, _p2) = super::solve(input);
        assert_eq!(p1, Solution::USize(6440));
        // assert_eq!(p2, expected);
    }
}
