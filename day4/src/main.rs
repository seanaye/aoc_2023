use std::{collections::VecDeque, str::FromStr};

use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::complete::digit1,
    character::complete::u32,
    combinator::map_res,
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    winners: Vec<usize>,
    members: Vec<usize>,
}

impl Card {
    fn count_winners(&self) -> usize {
        self.members
            .iter()
            .fold(0, |acc, cur| match self.winners.contains(cur) {
                true => acc + 1,
                false => acc,
            })
    }

    fn score_card(&self) -> usize {
        match self.count_winners() {
            0 => 0,
            n => 2usize.pow(n as u32 - 1),
        }
    }
}

fn num(s: &str) -> IResult<&str, usize> {
    let (next, _) = take_till(|c: char| !c.is_whitespace())(s)?;

    map_res(digit1, str::parse)(next)
}

fn num_list(s: &str) -> IResult<&str, Vec<usize>> {
    many1(num)(s)
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (next, _) = take_till(|c: char| c.is_ascii_digit())(s)?;
    let (next, id) = map_res(digit1, str::parse)(next)?;
    let (next, _) = tag(": ")(next)?;
    let (_, (winners, members)) = separated_pair(num_list, tag(" | "), num_list)(next)?;
    Ok((
        "",
        Card {
            winners,
            members,
            id,
        },
    ))
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_card(s) {
            Ok((_, card)) => Ok(card),
            Err(_) => Err(()),
        }
    }
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|l| Card::from_str(l).unwrap().score_card())
        .sum()
}

struct CardsQueue {
    original: Vec<Card>,
}

impl CardsQueue {
    fn process(&mut self) {
        let original = self.original.len();
        for i in 1..=original {
            let to_add = self
                .original
                .iter()
                .filter(|c| c.id == i)
                .flat_map(|card| {
                    let winners = card.count_winners();
                    (i..winners + i).map(|idx| self.original.get(idx).unwrap().clone())
                })
                .collect::<Vec<_>>();
            self.original.extend(to_add)
        }
    }

    fn count_id(&self, id: usize) -> usize {
        self.original.iter().filter(|c| c.id == id).count()
    }
}

impl FromStr for CardsQueue {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            original: s
                .lines()
                .map(|l| Card::from_str(l).unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

fn part_two(s: &str) -> usize {
    let mut q = CardsQueue::from_str(s).unwrap();
    q.process();
    q.original.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(include_str!("../test")), 13);
        assert_eq!(part_one(include_str!("../input")), 26443);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(include_str!("../test2")), 30);
    }

    #[test]
    fn test_number_cards() {
        let mut q = CardsQueue::from_str(include_str!("../test2")).unwrap();
        q.process();
        assert_eq!(q.count_id(1), 1);
        assert_eq!(q.count_id(2), 2);
        assert_eq!(q.count_id(6), 1);
        assert_eq!(q.count_id(3), 4);
        assert_eq!(q.count_id(4), 8);
        assert_eq!(q.count_id(5), 14);
    }
}

