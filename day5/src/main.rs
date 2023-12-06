use std::{collections::HashMap, ops::RangeBounds};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::{map_res, opt, value},
    multi::many1,
    sequence::{separated_pair, tuple},
    IResult,
};

trait Parse: Sized {
    fn parse(s: &str) -> IResult<&str, Self>;
}

#[derive(Debug)]
struct SeedRange {
    start: usize,
    end: usize,
}

impl SeedRange {
    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.start..self.start + self.end
    }
}

impl Parse for SeedRange {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (next, _) = opt(tag(" "))(s)?;
        let (next, (start, end)) = separated_pair(num, tag(" "), num)(next)?;
        Ok((next, Self { start, end }))
    }
}

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Attribute {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Parse for Attribute {
    fn parse(s: &str) -> IResult<&str, Attribute> {
        alt((
            value(Attribute::Seed, tag("seed")),
            value(Attribute::Soil, tag("soil")),
            value(Attribute::Fertilizer, tag("fertilizer")),
            value(Attribute::Water, tag("water")),
            value(Attribute::Light, tag("light")),
            value(Attribute::Temperature, tag("temperature")),
            value(Attribute::Humidity, tag("humidity")),
            value(Attribute::Location, tag("location")),
        ))(s)
    }
}

#[derive(Debug)]
struct Map {
    link: Vec<Range>,
    from: Attribute,
    to: Attribute,
}

#[derive(Debug)]
struct Range {
    destination: usize,
    source: usize,
    range: usize,
}

impl Parse for Range {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (next, _) = take_till(|c: char| !c.is_whitespace())(s)?;
        let (next, (destination, source, range)) = tuple((num, num, num))(next)?;
        Ok((
            next,
            Range {
                destination,
                source,
                range,
            },
        ))
    }
}

impl Range {
    fn get(&self, val: &usize) -> Option<usize> {
        match (self.source..self.source + self.range).contains(val) {
            true => {
                let diff = val - self.source;
                Some(self.destination + diff)
            }
            false => None,
        }
    }
}

impl Map {
    fn get(&self, key: usize) -> usize {
        self.link
            .iter()
            .find_map(|range| range.get(&key))
            .unwrap_or(key)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<SeedRange>,
    maps: Vec<Map>,
}

impl Almanac {
    fn traverse(&self, to: Attribute, start: usize, cur: &Map) -> Option<usize> {
        let next_value = cur.get(start);
        if cur.to == to {
            return Some(next_value);
        }

        let next_map = cur.to;
        let this = self
            .maps
            .iter()
            .find(|Map { from, .. }| from == &next_map)?;
        self.traverse(to, next_value, this)
    }

    pub fn traverse_from(&self, to: Attribute, start: usize, from: Attribute) -> Option<usize> {
        let start_from = self.maps.iter().find(|Map { from: f, .. }| f == &from)?;
        self.traverse(to, start, start_from)
    }

    pub fn seeds(&self) -> impl Iterator<Item = &usize> + '_ {
        self.seeds
            .iter()
            .flat_map(|SeedRange { start, end }| [start, end].into_iter())
    }

    pub fn seed_ranges(&self) -> impl Iterator<Item = usize> + '_ {
        self.seeds.iter().flat_map(|seed_range| seed_range.iter())
    }

    // this is the faster way to search but rust finished the calculation
    // while i was starting the reverse implementation
    // pub fn invert(&mut self) {
    //     self.maps.iter_mut().for_each(|&mut map| {
    //         map.invert()
    //     })
    // }
}

fn num(s: &str) -> IResult<&str, usize> {
    let (next, _) = opt(tag(" "))(s)?;
    map_res(digit1, str::parse)(next)
}

impl Parse for Almanac {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (next, _) = tag("seeds: ")(s)?;
        let (next, seeds) = many1(SeedRange::parse)(next)?;
        let (next, maps) = many1(Map::parse)(next)?;
        Ok((next, Self { seeds, maps }))
    }
}

impl Parse for Map {
    fn parse(s: &str) -> IResult<&str, Map> {
        let (next, _) = take_till(|c: char| !c.is_whitespace())(s)?;
        let (next, (from, to)) =
            separated_pair(Attribute::parse, tag("-to-"), Attribute::parse)(next)?;
        let (next, _) = take_till(|c: char| c.is_ascii_digit())(next)?;
        let (next, ranges) = many1(Range::parse)(next)?;
        dbg!(ranges.len());
        Ok((
            next,
            Self {
                link: ranges,
                from,
                to,
            },
        ))
    }
}

fn part_one(s: &str) -> usize {
    let (_, almanac) = Almanac::parse(s).unwrap();

    almanac
        .seeds()
        .filter_map(|seed| almanac.traverse_from(Attribute::Location, *seed, Attribute::Seed))
        .min()
        .unwrap()
}

fn part_two(s: &str) -> usize {
    let (_, almanac) = Almanac::parse(s).unwrap();

        almanac
        .seed_ranges()
        .filter_map(|seed| almanac.traverse_from(Attribute::Location, seed, Attribute::Seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(include_str!("../test")), 35)
    }

    #[test]
    fn test_part_one_answer() {
        assert_eq!(part_one(include_str!("../input")), 177942185)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(include_str!("../test")), 46)
    }
}

