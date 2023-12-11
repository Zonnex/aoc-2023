use std::cmp::Ordering;

use crate::{Solution, SolutionPair};

use self::camel_cards::{Hand};

pub mod camel_cards;

#[derive(Debug, Clone)]
struct Line(camel_cards::Hand, camel_cards::Bid);

struct P1Ranker<'a>(&'a Hand);

impl<'a> P1Ranker<'a> {
    pub fn five_of_a_kind(&self) -> bool {
        let count = &self.0.card_count;
        count.values().any(|&v| v == 5)
    }

    pub fn four_of_a_kind(&self) -> bool {
        let count = &self.0.card_count;
        count.values().any(|&v| v == 4)
    }

    pub fn full_house(&self) -> bool {
        let count = &self.0.card_count;
        count.values().any(|&v| v == 3) && count.values().any(|&v| v == 2)
    }

    pub fn three_of_a_kind(&self) -> bool {
        let count = &self.0.card_count;
        count.values().any(|&v| v == 3) && count.values().filter(|&v| *v == 1).count() == 2
    }

    pub fn two_pair(&self) -> bool {
        let count = &self.0.card_count;
        count.values().filter(|&v| *v == 2).count() == 2
    }

    pub fn one_pair(&self) -> bool {
        let count = &self.0.card_count;
        count.values().any(|&v| v == 2) && count.values().filter(|&v| *v == 1).count() == 3
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let lines = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = camel_cards::Hand::parse(hand);
            let bid = camel_cards::Bid(bid.parse().unwrap());
            Line(hand, bid)
        })
        .collect::<Vec<_>>();

    (p1(p1_sorter(lines.clone())), p2(input))
}

fn p1_sorter(mut lines: Vec<Line>) -> Vec<Line> {
    fn rank(hand: &Hand) -> u8 {
        let hand = P1Ranker(hand);
        if hand.five_of_a_kind() {
            return 6;
        }
        if hand.four_of_a_kind() {
            return 5;
        }
        if hand.full_house() {
            return 4;
        }
        if hand.three_of_a_kind() {
            return 3;
        }
        if hand.two_pair() {
            return 2;
        }
        if hand.one_pair() {
            return 1;
        }
        0
    }

    fn compare_cards(left: &[camel_cards::Card], right: &[camel_cards::Card]) -> Ordering {
        for (l, r) in left.iter().zip(right.iter()) {
            let card_compare = l.value().cmp(&r.value());
            if card_compare != Ordering::Equal {
                return card_compare;
            }
        }
        Ordering::Equal
    }

    fn compare_hands(Line(left, _): &Line, Line(right, _): &Line) -> Ordering {
        let rank_compare = rank(left).cmp(&rank(right));
        if rank_compare == Ordering::Equal {
            compare_cards(&left.cards, &right.cards)
        } else {
            rank_compare
        }
    }

    lines.sort_by(compare_hands);
    lines
}

fn p2_sorter(_lines: Vec<Line>) -> Vec<Line> {
    todo!()
}

fn p1(input: Vec<Line>) -> Solution {
    let winnings = input
        .iter()
        .enumerate()
        .map(|(i, Line(_, camel_cards::Bid(bid)))| {
            let rank = i + 1;
            let bid = *bid as usize;
            rank * bid
        })
        .sum::<usize>();

    Solution::Usize(winnings)
}

fn p2(_input: &str) -> Solution {
    Solution::U64(0)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day07/test.txt");
        let (p1, _p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(6440));
        // assert_eq!(p2, expected);
    }
}
