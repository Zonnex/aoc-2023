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
    pub(super) card_count: HashMap<Card, u8>,
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

    pub(crate) fn parse(hand: &str) -> Hand {
        let cards = Card::parse(hand);
        Hand::new(cards)
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
