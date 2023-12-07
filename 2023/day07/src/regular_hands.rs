use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug)]
pub struct Hand {
    pub htype: HandType,
    pub cards: Vec<Card>,
    pub bid: usize,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum HandType {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum Card {
    N(u32),
    T,
    J,
    Q,
    K,
    A,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.htype != other.htype {
            return self.htype.cmp(&other.htype);
        }

        self.cards
            .iter()
            .zip(other.cards.iter())
            .find(|(s, o)| *s != *o)
            .map(|(s, o)| s.cmp(o))
            .unwrap_or(Ordering::Equal)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl From<Vec<Card>> for HandType {
    fn from(value: Vec<Card>) -> Self {
        let groups = value
            .iter()
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .rev()
            .collect_vec();

        match (groups.first().unwrap(), groups.get(1)) {
            (5, _) => HandType::Five,
            (4, _) => HandType::Four,
            (3, Some(2)) => HandType::Full,
            (3, _) => HandType::Three,
            (2, Some(2)) => HandType::TwoPair,
            (2, _) => HandType::Pair,
            _ => HandType::High,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            c if c.is_ascii_digit() => Card::N(c.to_digit(10).unwrap()),
            _ => panic!(),
        }
    }
}
