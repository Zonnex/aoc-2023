use std::cmp::Ordering;

use crate::{Solution, SolutionPair};

pub mod camel_cards;

#[derive(Debug, Clone)]
struct Line(camel_cards::Hand, camel_cards::Bid);


struct p1_hand_ranker(pub Line);

impl p1_hand_ranker {
    pub fn rank(&self) -> u8 {
        let p1_hand_ranker(Line(hand, _)) = self;
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
}

pub fn solve(input: &str) -> SolutionPair {
    let lines = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let hand = camel_cards::Hand::parse(hand);
            let bid = camel_cards::Bid(bid.parse().unwrap());
            Line(hand, bid)
        })
        .collect::<Vec<_>>();

    (p1(p1_sorter(lines.clone())), p2(input))
}

fn p1_sorter(mut lines: Vec<Line>) -> Vec<Line> {
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
        let rank_compare = left.rank().cmp(&right.rank());
        if rank_compare == Ordering::Equal {
            return compare_cards(&left.cards, &right.cards);
        } else {
            return rank_compare;
        }
    }

    lines.sort_by(compare_hands);
    lines
}

fn p2_sorter(lines: Vec<Line>) -> Vec<Line> {
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
