use nom::bytes::complete::take_till;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;
use std::{convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("../input");

fn main() {
    dbg!(part_one(INPUT));
}

#[derive(Debug)]
struct Grid {
    inner: String,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coord {
    fn adjacent(&self) -> Vec<Coord> {
        (self.x.saturating_sub(1)..=self.x + 1)
            .flat_map(|x| (self.y.saturating_sub(1)..=self.y + 1).map(move |y| Coord { x, y }))
            .filter(|c| c != self)
            .collect()
    }
}

impl Grid {
    fn get(&self, Coord { x, y }: &Coord) -> Option<char> {
        if x >= &self.width || y >= &self.height {
            return None;
        }
        let index = y * self.width + x;
        let out = self.inner.chars().nth(index);
        out
    }

    fn has_adjacent_symbol(&self, coord: &Coord) -> bool {
        coord
            .adjacent()
            .iter()
            .filter_map(|c| self.get(c))
            .any(|char| {
                !matches!(char, '.' | '0'..='9')
            })
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut inner = s.to_owned();
        inner.retain(|c| !c.is_whitespace());

        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

fn number_of_places(n: usize) -> usize {
    if n < 10 {
        return 1;
    };
    1 + number_of_places(n / 10)
}

fn digit_index(s: &str) -> IResult<&str, (usize, u32)> {
    let (digit, taken) = take_till(|c: char| c.is_ascii_digit())(s)?;
    let (next, d) = map_res(digit1, str::parse)(digit)?;
    Ok((next, (taken.len(), d)))
}

fn digit_indexes(s: &str) -> IResult<&str, Vec<(usize, u32)>> {
    let (_, mut vec) = many1(digit_index)(s)?;
    // we need to calculates the offsets for each line
    let mut cur_offset = 0;
    for (off, num) in vec.iter_mut() {
        let next = *off + number_of_places(*num as usize);
        *off += cur_offset;
        cur_offset += next;
    }

    Ok(("", vec))
}

fn part_one(input: &str) -> u32 {
    let grid = input.parse::<Grid>().unwrap();
    input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let (_, vec) = digit_indexes(line).ok()?;
            let out = vec
                .iter()
                .filter_map(|(x, num)| {
                    let is_part_number = (*x..(x + number_of_places(*num as usize)))
                        .any(|x| grid.has_adjacent_symbol(&Coord { x, y }));
                    match is_part_number {
                        true => Some(*num),
                        false => None,
                    }
                })
                .sum::<u32>();
            Some(out)
        })
        .sum()
}

