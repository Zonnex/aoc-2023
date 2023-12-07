use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Bid(pub u32);

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    N(u8),
}
impl Card {
    pub fn from_char(s: char) -> Option<Self> {
        match s {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '2'..='9' => s.to_digit(10).map(|n| Card::N(n as u8)),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::N(n) => *n,
        }
    }

    pub fn parse(s: &str) -> Vec<Self> {
        s.chars().filter_map(|c| Card::from_char(c)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    card_count: HashMap<Card, u8>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        let mut count = HashMap::new();
        for card in &cards {
            *count.entry(*card).or_insert(0) += 1;
        }

        Self {
            cards,
            card_count: count,
        }
    }

    pub fn rank(&self) -> u8 {
        if self.five_of_a_kind() {
            return 6;
        }
        if self.four_of_a_kind() {
            return 5;
        }
        if self.full_house() {
            return 4;
        }
        if self.three_of_a_kind() {
            return 3;
        }
        if self.two_pair() {
            return 2;
        }
        if self.one_pair() {
            return 1;
        }
        0
    }

    pub fn five_of_a_kind(&self) -> bool {
        let count = &self.card_count;
        count.values().any(|&v| v == 5)
    }

    pub fn four_of_a_kind(&self) -> bool {
        let count = &self.card_count;
        count.values().any(|&v| v == 4)
    }

    pub fn full_house(&self) -> bool {
        let count = &self.card_count;
        count.values().any(|&v| v == 3) && count.values().any(|&v| v == 2)
    }

    pub fn three_of_a_kind(&self) -> bool {
        let count = &self.card_count;
        count.values().any(|&v| v == 3) && count.values().filter(|&v| *v == 1).count() == 2
    }

    pub fn two_pair(&self) -> bool {
        let count = &self.card_count;
        count.values().filter(|&v| *v == 2).count() == 2
    }

    pub fn one_pair(&self) -> bool {
        let count = &self.card_count;
        count.values().any(|&v| v == 2) && count.values().filter(|&v| *v == 1).count() == 3
    }

    pub(crate) fn parse(hand: &str) -> Hand {
        let cards = Card::parse(hand);
        Hand::new(cards)
    }
}

#[cfg(test)]
mod hand_tests {
    use super::*;

    #[test]
    fn test_five_of_a_kind() {
        let hand = Hand::new(vec![Card::A, Card::A, Card::A, Card::A, Card::A]);
        assert!(hand.five_of_a_kind());
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = Hand::new(vec![Card::A, Card::A, Card::A, Card::A, Card::K]);
        assert!(hand.four_of_a_kind());
    }

    #[test]
    fn test_full_house() {
        let hand = Hand::new(vec![Card::A, Card::A, Card::A, Card::N(2), Card::N(2)]);
        assert!(hand.full_house());
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = Hand::new(vec![Card::A, Card::A, Card::A, Card::N(2), Card::N(3)]);
        assert!(hand.three_of_a_kind());
    }

    #[test]
    fn test_two_pair() {
        let hand = Hand::new(vec![Card::A, Card::A, Card::N(2), Card::N(2), Card::N(3)]);
        assert!(hand.two_pair());
    }

    #[test]
    fn test_one_pair() {
        let hand = Hand::new(vec![Card::A, Card::A, Card::N(2), Card::N(3), Card::N(4)]);
        assert!(hand.one_pair());
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn test_card_sorting() {
        let mut cards = Card::parse("987A645KQJT32");

        assert_eq!(
            cards,
            vec![
                Card::A,
                Card::K,
                Card::Q,
                Card::J,
                Card::T,
                Card::N(9),
                Card::N(8),
                Card::N(7),
                Card::N(6),
                Card::N(5),
                Card::N(4),
                Card::N(3),
                Card::N(2),
            ]
        );
    }
}
