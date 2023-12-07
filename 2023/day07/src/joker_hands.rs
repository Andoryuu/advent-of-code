use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug)]
pub struct JHand {
    pub htype: JHandType,
    pub cards: Vec<JCard>,
    pub bid: usize,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum JHandType {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum JCard {
    J,
    N(u32),
    T,
    Q,
    K,
    A,
}

impl Ord for JHand {
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

impl Eq for JHand {}

impl PartialOrd for JHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for JHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl From<Vec<JCard>> for JHandType {
    fn from(value: Vec<JCard>) -> Self {
        let groups = value
            .iter()
            .filter(|c| **c != JCard::J)
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .rev()
            .collect_vec();

        let jkr = value.iter().filter(|c| **c == JCard::J).count();

        match (groups.first().unwrap_or(&0) + jkr, groups.get(1)) {
            (5, _) => JHandType::Five,
            (4, _) => JHandType::Four,
            (3, Some(2)) => JHandType::Full,
            (3, _) => JHandType::Three,
            (2, Some(2)) => JHandType::TwoPair,
            (2, _) => JHandType::Pair,
            _ => JHandType::High,
        }
    }
}

impl From<char> for JCard {
    fn from(value: char) -> Self {
        match value {
            'A' => JCard::A,
            'K' => JCard::K,
            'Q' => JCard::Q,
            'J' => JCard::J,
            'T' => JCard::T,
            c if c.is_ascii_digit() => JCard::N(c.to_digit(10).unwrap()),
            _ => panic!(),
        }
    }
}
