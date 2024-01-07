use crate::{Solution, SolutionPair};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Bid(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand([usize; 5]);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Part {
    One,
    Two,
}

const A: usize = 14;
const K: usize = 13;
const Q: usize = 12;
const J: usize = 11;
const T: usize = 10;

impl Hand {
    fn parse(s: &str) -> Self {
        let mut cards = [0; 5];
        let mut i = 0;
        for card in s.as_bytes() {
            cards[i] = match card {
                b'A' => A,
                b'K' => K,
                b'Q' => Q,
                b'J' => J,
                b'T' => T,
                b'2'..=b'9' => (card - b'0') as usize,
                _ => unreachable!(),
            };
            i += 1;
        }

        Self(cards)
    }

    fn rank(self, part: Part) -> usize {
        let Hand(cards) = self;

        let cards = match part {
            Part::One => cards,
            Part::Two => {
                let mut cards = cards;
                cards.iter_mut().for_each(|c| {
                    if *c == J {
                        *c = 1;
                    }
                });
                cards
            }
        };

        let mut counts = [0; 15];
        for card in cards.iter() {
            let index = *card;
            counts[index] += 1;
        }

        let jokers = counts[1];
        counts[1] = 0;
        counts.sort_unstable();
        counts.reverse();
        counts[0] += jokers;


        let mut key = 0;

        for f in counts.iter().take(5) {
            key = (key << 4) | f;
        }
        for r in cards {
            key = (key << 4) | r;
        }

        key
    }
}

#[derive(Debug, Clone, Copy)]
struct Line(Hand, Bid);

pub fn solve(input: &str) -> SolutionPair {
    let lines = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = Hand::parse(hand);
            let bid = Bid(bid.parse().unwrap());
            Line(hand, bid)
        })
        .collect::<Vec<_>>();

    (p1(&lines), p2(&lines))
}

fn p1(lines: &[Line]) -> Solution {
    let result = sort(lines, Part::One);

    Solution::Usize(result)
}

fn p2(input: &[Line]) -> Solution {
    let result = sort(input, Part::Two);

    Solution::Usize(result)
}

fn sort(lines: &[Line], part: Part) -> usize {
    let mut lines = lines
        .iter()
        .map(|Line(hand, Bid(bid))| (hand.rank(part), *bid))
        .collect::<Vec<_>>();

    lines.sort_unstable();

    lines
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| ((index + 1) * *bid))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day07/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(6440));
        assert_eq!(p2, Solution::Usize(5905));
    }
}
