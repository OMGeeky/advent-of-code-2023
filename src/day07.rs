use std::{cmp::Ordering, collections::HashMap, usize};

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

impl DayPart2 for Day07{
    fn run_part2(data: Self::Input) -> Self::Output {
        //TODO: does not handle case right: J345A should just be OnePair since J can only mimic one card not multiple
        let mut ranks = data.hands;
        ranks.sort_by(|s, o|{
            let cmp = s.combo_part2.cmp(&o.combo_part2);
            match cmp {
                std::cmp::Ordering::Less => Ordering::Less,
                std::cmp::Ordering::Equal => {
                    for (own, other) in s.cards_part2.iter().zip(o.cards_part2.iter()) {
                        match own.cmp(other) {
                            Ordering::Equal => {}
                            _ => return own.cmp(other),
                        }
                    }
                    dbg!("The cards were the same", s);
                    Ordering::Equal
                }
                std::cmp::Ordering::Greater => Ordering::Greater,
            }
        });
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
                    hand.combo_part2,
                    hand.cards_part2
                )
            })
            .map(|(r, h)| r * h.bid)
            .sum()
    }

    fn get_test_data_part2() -> Self::Input {
        "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41".to_string().into()
    }
    fn get_test_result_part2() -> Self::Output {
        6839
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
pub type HandOfCardsPart2 = [CardPart2; 5];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
pub struct Hand {
    bid: usize,
    cards: HandOfCards,
    cards_part2: HandOfCardsPart2,
    combo: ComboType,
    combo_part2: ComboType,
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
        let (s, bid) = value.split_once(' ').unwrap();

        let cards = s
            .chars()
            .map_into()
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let cards_part2 = s
            .chars()
            .map_into()
            .collect::<Vec<CardPart2>>()
            .try_into()
            .unwrap();
        let bid = bid.trim().parse().unwrap();
        let combo = ComboType::from(&cards);
        let combo_part2 = ComboType::from_part2(&cards_part2);

        Hand { bid, cards, combo, cards_part2, combo_part2 }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardPart2 {
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}
impl From<Card> for CardPart2{
    fn from(value: Card) -> Self {
        match value {
            Card::_2 =>CardPart2::_2,
            Card::_3 =>CardPart2::_3,
            Card::_4 =>CardPart2::_4,
            Card::_5 =>CardPart2::_5,
            Card::_6 =>CardPart2::_6,
            Card::_7 =>CardPart2::_7,
            Card::_8 =>CardPart2::_8,
            Card::_9 =>CardPart2::_9,
            Card::T =>CardPart2::T,
            Card::J =>CardPart2::J,
            Card::Q =>CardPart2::Q,
            Card::K =>CardPart2::K,
            Card::A =>CardPart2::A,
        }
    }
}
impl Into<Card> for CardPart2{
    fn into(self) -> Card {
        match self {
            CardPart2::_2 =>Card::_2,
            CardPart2::_3 =>Card::_3,
            CardPart2::_4 =>Card::_4,
            CardPart2::_5 =>Card::_5,
            CardPart2::_6 =>Card::_6,
            CardPart2::_7 =>Card::_7,
            CardPart2::_8 =>Card::_8,
            CardPart2::_9 =>Card::_9,
            CardPart2::T =>Card::T,
            CardPart2::J =>Card::J,
            CardPart2::Q =>Card::Q,
            CardPart2::K =>Card::K,
            CardPart2::A =>Card::A,
        }
    }
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
impl From<char> for CardPart2 {
    fn from(value: char) -> Self {
        match value {
            'A' => CardPart2::A,
            'K' => CardPart2::K,
            'J' => CardPart2::J,
            'Q' => CardPart2::Q,
            'T' => CardPart2::T,
            '2' => CardPart2::_2,
            '3' => CardPart2::_3,
            '4' => CardPart2::_4,
            '5' => CardPart2::_5,
            '6' => CardPart2::_6,
            '7' => CardPart2::_7,
            '8' => CardPart2::_8,
            '9' => CardPart2::_9,
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
impl ComboType {
    fn from(value: &HandOfCards) -> Self {
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
    fn from_part2(value: &HandOfCardsPart2) -> Self {
        let value = value.to_vec();
        let mut counter = CardCounter::new();
        for c in value {
            counter.increment_part2(c.into());
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
    fn increment_part2(&mut self, c: Card) {
        if c == Card::J{
            for value in self._values.values_mut(){
                *value += 1;
            }
            return;
        }
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
