use nom::bytes::complete::take_till;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;
use std::{convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("../input");

fn main() {
    dbg!(part_one(INPUT));
    dbg!(part_two(INPUT));
}

#[derive(Debug)]
struct Grid {
    inner: String,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone)]
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
    fn adjacent(&self) -> impl Iterator<Item = Coord> + '_ {
        (self.x.saturating_sub(1)..=self.x + 1)
            .flat_map(|x| (self.y.saturating_sub(1)..=self.y + 1).map(move |y| Coord { x, y }))
            .filter(move |c| c != self)
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
            .filter_map(|c| self.get(&c))
            .any(|char| !matches!(char, '.' | '0'..='9'))
    }

    fn index_to_coord(&self, i: &usize) -> Coord {
        let x = i % self.width;
        let y = i / self.width;
        Coord { x, y }
    }

    fn iter_lines(&self) -> impl Iterator<Item = &str> {
        (0..self.height).map(|i| {
            let start = i * self.height;
            let end = start + self.width;
            &self.inner[start..end]
        })
    }

    fn iter_ranges(&self) -> impl Iterator<Item = NumberRange> + '_ {
        self.iter_lines()
            .enumerate()
            .filter_map(|(y, line)| {
                let (_, line) = digit_indexes(line).ok()?;
                Some(
                    line.into_iter()
                        .map(move |NumberLine { origin, num }| NumberRange {
                            coord: Coord { x: origin, y },
                            num,
                        }),
                )
            })
            .flatten()
    }
}

struct GridCharIter<'a> {
    grid: &'a Grid,
    cur: usize,
}

impl Iterator for GridCharIter<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.grid.inner.chars().nth(self.cur);
        self.cur += 1;
        out
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = char;

    type IntoIter = GridCharIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridCharIter { grid: self, cur: 0 }
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

fn digit_index(s: &str) -> IResult<&str, NumberLine> {
    let (digit, taken) = take_till(|c: char| c.is_ascii_digit())(s)?;
    let (next, d) = map_res(digit1, str::parse)(digit)?;
    Ok((
        next,
        NumberLine {
            origin: taken.len(),
            num: d,
        },
    ))
}

#[derive(Debug, Copy, Clone, Hash)]
struct NumberLine {
    origin: usize,
    num: u32,
}

#[derive(Debug, Copy, Clone)]
struct NumberRange {
    num: u32,
    coord: Coord,
}

impl NumberRange {
    fn places(&self) -> usize {
        number_of_places(self.num as usize)
    }

    fn range(&self) -> std::ops::Range<usize> {
        self.coord.x..(self.coord.x + self.places())
    }

    fn intersects(&self, coord: &Coord) -> bool {
        self.range().contains(&coord.x) && coord.y == self.coord.y
    }
}

fn digit_indexes(s: &str) -> IResult<&str, Vec<NumberLine>> {
    let (_, mut vec) = many1(digit_index)(s)?;
    // we need to calculates the offsets for each line
    let mut cur_offset = 0;
    for NumberLine { origin, num } in vec.iter_mut() {
        let next = *origin + number_of_places(*num as usize);
        *origin += cur_offset;
        cur_offset += next;
    }

    Ok(("", vec))
}

fn part_one(input: &str) -> u32 {
    let grid = input.parse::<Grid>().unwrap();
    grid.iter_ranges()
        .filter_map(
            |NumberRange {
                 coord: Coord { x, y },
                 num,
             }| {
                let is_part_number = (x..(x + number_of_places(num as usize)))
                    .any(|x| grid.has_adjacent_symbol(&Coord { x, y }));
                match is_part_number {
                    true => Some(num),
                    false => None,
                }
            },
        )
        .sum::<u32>()
}

fn part_two(s: &str) -> u32 {
    let grid = s.parse::<Grid>().unwrap();
    let ranges = grid.iter_ranges().collect::<Vec<_>>();
    grid.into_iter()
        .enumerate()
        .filter_map(|(i, c)| {
            let coord = grid.index_to_coord(&i);
            match c {
                '*' => Some(coord),
                _ => None,
            }
        })
        .filter_map(|coord| {
            let adjacent_ranges = ranges
                .iter()
                .filter(|range| coord.adjacent().any(|coord| range.intersects(&coord)))
                .collect::<Vec<_>>();

            match adjacent_ranges.len() {
                2 => Some(adjacent_ranges[0].num * adjacent_ranges[1].num),
                _ => None,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 554003);
        assert_eq!(part_one(include_str!("../test")), 4361)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(include_str!("../test")), 467835);
    }
}

