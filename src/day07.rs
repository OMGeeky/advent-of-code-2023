use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use crate::*;

pub struct Day07;
impl Day for Day07 {
    const DAY_NUM: u8 = 7;
    type Input = Game;
    type Output = usize;

    fn get_test_data() -> Self::Input {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string()
            .into()
    }

    fn get_test_result() -> Self::Output {
        6440
    }

    fn run(data: Self::Input) -> Self::Output {
        dbg!(&data);
        let mut ranks = data.hands;
        ranks.sort();
        let ranks: Vec<(usize, Hand)> = ranks.into_iter().enumerate().collect();
        ranks
            .iter()
            .map(|(r, h)|(r+1, h))
            .inspect(|(r, hand)| {
                println!(
                    "{:>4}*{:>3}=>{:>7} \t{:?} \t{:?}",
                    r ,
                    hand.bid,
                    r * hand.bid,
                    hand.combo,
                    hand.cards
                )
            })
            .map(|(r, h)| r * h.bid)
            .sum()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Game {
    hands: Vec<Hand>,
}
impl From<String> for Game {
    fn from(value: String) -> Self {
        Self {
            hands: value.lines().map_into().collect(),
        }
    }
}
pub type HandOfCards = [Card; 5];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
pub struct Hand {
    bid: usize,
    cards: HandOfCards,
    combo: ComboType,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.combo.cmp(&other.combo);
        Some(match cmp {
            std::cmp::Ordering::Less => Ordering::Less,
            std::cmp::Ordering::Equal => {
                for (own, other) in self.cards.iter().zip(other.cards.iter()) {
                    match own.cmp(other) {
                        Ordering::Equal => {}
                        _ => return own.partial_cmp(other),
                    }
                }
                dbg!("The cards were the same", self);
                Ordering::Equal
            }
            std::cmp::Ordering::Greater => Ordering::Greater,
        })
    }
}
// impl Ord for Hand {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let cmp = self.combo.cmp(&other.combo);
//         match cmp {
//             std::cmp::Ordering::Less => Ordering::Less,
//             std::cmp::Ordering::Equal => {
//                 for (own, other) in self.cards.iter().zip(other.cards.iter()) {
//                     match own.cmp(other) {
//                         Ordering::Equal => {}
//                         _ => return own.cmp(other),
//                     }
//                 }
//                 dbg!("The cards were the same", self);
//                 Ordering::Equal
//             }
//             std::cmp::Ordering::Greater => Ordering::Greater,
//         }
//     }
// }

impl<'a> From<&'a str> for Hand {
    fn from(value: &'a str) -> Self {
        let (cards, bid) = value.split_once(' ').unwrap();

        let cards = cards
            .chars()
            .map_into()
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let bid = bid.trim().parse().unwrap();
        let combo = ComboType::from(&cards);
        Hand { bid, cards, combo }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'J' => Card::J,
            'Q' => Card::Q,
            'T' => Card::T,
            '2' => Card::_2,
            '3' => Card::_3,
            '4' => Card::_4,
            '5' => Card::_5,
            '6' => Card::_6,
            '7' => Card::_7,
            '8' => Card::_8,
            '9' => Card::_9,
            other => panic!("Invalid card: {other}"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComboType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl<'a> From<&'a HandOfCards> for ComboType {
    fn from(value: &'a HandOfCards) -> Self {
        let value = value.to_vec();
        let mut counter = CardCounter::new();
        for c in value {
            counter.increment(c);
        }
        let five = counter.get_by_amount(5);
        let four = counter.get_by_amount(4);
        let three = counter.get_by_amount(3);
        let two = counter.get_by_amount(2);
        if !five.is_empty() {
            return ComboType::FiveOfAKind;
        }
        if !four.is_empty() {
            return ComboType::FourOfAKind;
        }
        if !three.is_empty() {
            if !two.is_empty() {
                return ComboType::FullHouse;
            } else {
                return ComboType::ThreeOfAKind;
            }
        }
        if !two.is_empty() {
            if two.len() >= 2 {
                return ComboType::TwoPair;
            } else {
                return ComboType::OnePair;
            }
        }
        return ComboType::HighCard;
    }
}

struct CardCounter {
    _values: HashMap<Card, usize>,
}
impl CardCounter {
    fn new() -> Self {
        let mut counter = HashMap::new();
        counter.insert(Card::A, 0);
        counter.insert(Card::J, 0);
        counter.insert(Card::K, 0);
        counter.insert(Card::Q, 0);
        counter.insert(Card::T, 0);
        counter.insert(Card::_9, 0);
        counter.insert(Card::_8, 0);
        counter.insert(Card::_7, 0);
        counter.insert(Card::_6, 0);
        counter.insert(Card::_5, 0);
        counter.insert(Card::_4, 0);
        counter.insert(Card::_3, 0);
        counter.insert(Card::_2, 0);
        Self { _values: counter }
    }

    fn increment(&mut self, c: Card) {
        if let Some(v) = self._values.get_mut(&c) {
            *v += 1;
        } else {
            self._values.insert(c, 1);
        }
    }
    fn get_by_amount(&self, amount: usize) -> Vec<Card> {
        let mut cards = vec![];
        for (key, value) in &self._values {
            if *value == amount {
                cards.push(*key);
            }
        }
        cards
    }
}
