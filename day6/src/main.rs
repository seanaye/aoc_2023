use std::iter::zip;

use nom::{
    bytes::complete::{tag, take_till, take_while},
    character::complete::digit1,
    combinator::map_res,
    multi::many1,
    IResult,
};
use std::fmt::Write;

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

trait Parse: Sized {
    fn parse(s: &str) -> IResult<&str, Self>;
}

struct Race {
    time: usize,
    target_distance: usize,
}

struct Solution {
    expected_distance: usize,
    hold_time: usize,
    run_time: usize,
}

impl Solution {
    fn new(hold_time: usize, race_time: usize) -> Solution {
        let run_time = race_time - hold_time;
        let expected_distance = run_time * hold_time;
        Solution {
            expected_distance,
            hold_time,
            run_time,
        }
    }

    fn is_valid(&self, target_distance: usize) -> bool {
        self.expected_distance > target_distance
    }
}

impl Race {
    fn solve(&self) -> Vec<Solution> {
        (0..self.time)
            .map(|hold_time| Solution::new(hold_time, self.time))
            .filter(|solution| solution.is_valid(self.target_distance))
            .collect()
    }
}

struct Races {
    inner: Vec<Race>,
}

impl Races {
    fn concat(&mut self) {
        let time = self.inner.iter().fold(String::new(), |mut out, race| {
            let _ = write!(out, "{}", race.time);
            out
        });
        let distance = self.inner.iter().fold(String::new(), |mut out, race| {
            let _ = write!(out, "{}", race.target_distance);
            out
        });

        self.inner = vec![Race {
            time: time.parse().unwrap(),
            target_distance: distance.parse().unwrap(),
        }]
    }
}

fn num(s: &str) -> IResult<&str, usize> {
    let (next, _) = take_while(|c: char| c == ' ')(s)?;
    map_res(digit1, str::parse)(next)
}

impl Parse for Races {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (next, _) = tag("Time: ")(s)?;
        let (next, times) = many1(num)(next)?;
        let (next, _) = take_till(|c: char| c.is_ascii_digit())(next)?;
        let (_next, distances) = many1(num)(next)?;
        let inner = zip(times, distances)
            .map(|(time, distance)| Race {
                time,
                target_distance: distance,
            })
            .collect::<Vec<_>>();
        Ok(("", Races { inner }))
    }
}

fn part_one(s: &str) -> usize {
    let (_, races) = Races::parse(s).unwrap();
    races.inner.iter().map(|race| race.solve().len()).product()
}

fn part_two(s: &str) -> usize {
    let (_, mut races) = Races::parse(s).unwrap();
    races.concat();

    races.inner.iter().map(|race| race.solve().len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(include_str!("../test")), 288);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(include_str!("../test")), 71503);
    }
}

