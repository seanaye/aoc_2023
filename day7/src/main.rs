use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

struct Hand<'a> {
    bet: usize,
    cards: Cards<'a>,
}

impl<'a> Hand<'a> {
    fn from_str(s: &'a str, is_part_2: bool) -> Result<Self, ()> {
        let mut iter = s.split_whitespace();
        let cards = iter.next().ok_or(())?;
        let bet: Result<usize, ()> = match iter.next() {
            Some(s) => s.parse().map_err(|_| ()),
            None => Err(()),
        };

        Ok(Hand {
            bet: bet?,
            cards: Cards {
                inner: cards,
                j_wildcard: is_part_2,
            },
        })
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand<'_> {}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cards.cmp(&other.cards))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Cards<'a> {
    inner: &'a str,
    j_wildcard: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl<'a> Cards<'a> {
    fn group(&self) -> HashMap<char, usize> {
        self.inner
            .chars()
            .into_grouping_map_by(|&x| x)
            .fold(0, |acc, _key, _value| acc + 1)
    }

    fn group_pt2(&self) -> HashMap<char, usize> {
        let mut map = self.group();
        if let Some(v) = map.remove(&'J') {
            // put the count of Js in the largest bin
            let c = map
                .iter()
                .max_by(|(_key_a, val_a), (_key_b, val_b)| val_a.cmp(val_b))
                .map(|(c, _)| c)
                .unwrap_or(&'J');
            map.entry(*c).and_modify(|a| *a += v).or_insert(5);
        }
        map
    }

    fn hand_type(&self) -> HandType {
        let group = if self.j_wildcard {
            self.group_pt2()
        } else {
            self.group()
        };
        let mut values = group.values().collect::<Vec<_>>();
        values.sort();
        match values.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::Pair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            x => {
                dbg!(x);
                panic!("invalid hand")
            }
        }
    }
}

impl<'a> PartialEq for Cards<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() == other.hand_type()
    }
}

fn char_to_points(char: &char) -> usize {
    match char {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("invalid char {}", char),
    }
}

fn char_to_points_pt_2(char: &char) -> usize {
    match char {
        'J' => 1,
        x => char_to_points(x),
    }
}

impl Eq for Cards<'_> {}

impl<'a> PartialOrd for Cards<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Cards<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let self_char = self.inner.chars().nth(i).unwrap();
                    let other_char = other.inner.chars().nth(i).unwrap();
                    let self_points = if self.j_wildcard {
                        char_to_points_pt_2(&self_char)
                    } else {
                        char_to_points(&self_char)
                    };
                    let other_points = if self.j_wildcard {
                        char_to_points_pt_2(&other_char)
                    } else {
                        char_to_points(&other_char)
                    };
                    match self_points.cmp(&other_points) {
                        std::cmp::Ordering::Equal => continue,
                        x => return x,
                    }
                }
                panic!("invalid hand");
            }
            x => x,
        }
    }
}

struct AllHands<'a> {
    inner: Vec<Hand<'a>>,
}

impl<'a> AllHands<'a> {
    fn from_str(s: &'a str, is_part_2: bool) -> Result<Self, ()> {
        let inner = s
            .lines()
            .map(|l| Hand::from_str(l, is_part_2))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AllHands { inner })
    }

    fn sort(&mut self) {
        self.inner.sort()
    }

    fn sum(&mut self) -> usize {
        self.sort();
        self.inner
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bet)
            .sum()
    }
}

fn part_one(s: &str) -> usize {
    let mut all_hands = AllHands::from_str(s, false).unwrap();
    all_hands.sum()
}

fn part_two(s: &str) -> usize {
    let mut all_hands = AllHands::from_str(s, true).unwrap();
    all_hands.sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(include_str!("../test")), 6440)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(include_str!("../test2")), 5905)
    }
}

