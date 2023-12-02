use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::is_alphabetic,
    combinator::{map_res, value},
    number::complete::u32,
    IResult, Parser,
};

const INPUT: &str = include_str!("../test");
const RADIX: u32 = 10u32;

fn part_one() -> u32 {
    iter_lines()
        .map(|line| {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            for char in line.chars() {
                match (char.is_ascii_digit(), first, last) {
                    (true, None, None) => {
                        first = char.to_digit(RADIX);
                    }
                    (true, Some(_), _) => {
                        last = char.to_digit(RADIX);
                    }
                    _ => (),
                }
            }
            let pos1 = first.unwrap_or(0);
            let out: u32 = pos1 * 10 + last.unwrap_or(pos1);
            out
        })
        .sum()
}

fn get_str_nums() -> Vec<&'static str> {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";
    let six = "six";
    let seven = "seven";
    let eight = "eight";
    let nine = "nine";
    let zero = "zero";

    vec![one, two, three, four, five, six, seven, eight, nine, zero]
}

fn word_to_num(s: &str) -> Option<u32> {
    let out = match s {
        "one" => Some(1u32),
        "two" => Some(2u32),
        "three" => Some(3u32),
        "four" => Some(4u32),
        "five" => Some(5u32),
        "six" => Some(6u32),
        "seven" => Some(7u32),
        "eight" => Some(8u32),
        "nine" => Some(9u32),
        "zero" => Some(0u32),
        _ => None,
    };
    dbg!(&out);
    out
}

fn part_two() -> u32 {
    iter_lines()
        .map(|line| {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            for (i, char) in line.chars().enumerate() {
                let is_digit = char.is_ascii_digit();

                if !is_digit {
                    match (search_slice(&line[i..]), first, last) {
                        (Some(num), None, None) => {
                            first = Some(num);
                        }
                        (Some(num), Some(_), _) => {
                            last = Some(num);
                        }
                        _ => (),
                    }
                }

                match (is_digit, first, last) {
                    (true, None, None) => {
                        first = char.to_digit(RADIX);
                    }
                    (true, Some(_), _) => {
                        last = char.to_digit(RADIX);
                    }
                    _ => (),
                }
            }
            let pos1 = first.unwrap_or(0);
            let out: u32 = pos1 * 10 + last.unwrap_or(pos1);
            out
        })
        .sum()
}

fn search_slice(s: &str) -> Option<u32> {
    let nums = get_str_nums();
    for num in nums {
        dbg!(num, &s);
        for (i, c) in s.chars().enumerate() {
            if i >= num.len() {
                return word_to_num(num);
            }
            let this_char = num.chars().nth(i).unwrap_or('.');
            if this_char != c {
                break;
            }
            if i == s.len() - 1 {
                return word_to_num(num);
            }
        }
    }
    None
}

// fn is_num(s: &str) -> Result<u32, ()> {
//     match s {
//         _ => Err(())
//     }
// }
//
//
// fn is_num(s: &str) -> IResult<&str, u32> {
//     alt((
//         value(1u32, tag("one")),
//         value(2u32, tag("two")),
//         value(3u32, tag("three")),
//         value(4u32, tag("four")),
//         value(5u32, tag("five")),
//         value(6u32, tag("six")),
//         value(7u32, tag("seven")),
//         value(8u32, tag("eight")),
//         value(9u32, tag("nine")),
//         value(0u32, tag("zero")),
//     ))(s)
// }

// fn num_str(s: &str) -> IResult<(&str, u32), u8> {
//     let out = (s);
//     dbg!(out);
//     todo!()
// }

// fn part_two() -> u32 {
//     iter_lines().map(num_str);
//     todo!()
// }

fn main() {
    dbg!(part_two());
    // dbg!(part_one());
}

fn iter_lines() -> impl Iterator<Item = &'static str> {
    INPUT.lines()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_main() {
//         main();
//     }
// }

