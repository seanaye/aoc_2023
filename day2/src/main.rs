use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::{map, opt},
    sequence::delimited,
    IResult, multi::many1,
};

fn main() {
    dbg!(part_one());
}

const INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct Set {
    red: u32,
    blue: u32,
    green: u32,
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let r = self.red.partial_cmp(&other.red);
        let g = self.green.partial_cmp(&other.green);
        let b = self.blue.partial_cmp(&other.blue);

        match (r, g, b) {
            (
                Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal),
                Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal),
                Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal),
            ) => Some(std::cmp::Ordering::Less),
            (
                Some(std::cmp::Ordering::Greater),
                Some(std::cmp::Ordering::Greater),
                Some(std::cmp::Ordering::Greater),
            ) => Some(std::cmp::Ordering::Greater),
            _ => None,
        }
    }
}

fn iter_lines() -> impl Iterator<Item = &'static str> {
    INPUT.lines()
}

fn blue(s: &str) -> IResult<&str, u32> {
    let (next, out) = u32(s)?;
    let (next, _) = tag(" blue")(next)?;
    Ok((next, out))
}

fn red(s: &str) -> IResult<&str, u32> {
    let (next, out) = u32(s)?;
    let (next, _) = tag(" red")(next)?;
    Ok((next, out))
}

fn green(s: &str) -> IResult<&str, u32> {
    let (next, out) = u32(s)?;
    let (next, _) = tag(" green")(next)?;
    Ok((next, out))
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn rgb(s: &str) -> IResult<&str, Color> {
    let (next, _) = opt(tag(", "))(s)?;
    let (next, out) = alt((
        map(red, Color::Red),
        map(green, Color::Green),
        map(blue, Color::Blue),
    ))(next)?;
    // dbg!(&next, &out);
    Ok((next, out))
}

fn set(s: &str) -> IResult<&str, Option<Set>> {
    dbg!(&s);
    let mut red = 0u32;
    let mut green = 0u32;
    let mut blue = 0u32;

    let res = many1(rgb)(s);
    let (next, colors) = res?;

    for color in colors {
        match color {
            Color::Red(r) => red = r,
            Color::Green(g) => green = g,
            Color::Blue(b) => blue = b,
        }
    }

    Ok((next, Some(Set { red, green, blue })))
}

#[derive(Debug)]
struct Game {
    sets: Vec<Set>,
}

fn game_from_s(s: &str) -> IResult<&str, Game> {
    let (next, _) = delimited(tag("Game "), u32, tag(": "))(s)?;
    let sets = next
        .split("; ")
        .filter_map(|s| {
            let (_, set) = set(s).unwrap();
            set
        })
        .collect::<Vec<_>>();
    Ok(("", Game { sets }))
}

// fn game_is_valid(this_game: &Set, max_game: &Set) -> bool {
//     this_game <= max_game
// }

fn part_one() -> u32 {
    let max_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    iter_lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let id = i + 1;
            let (_, game) = game_from_s(line).ok()?;
            match game.sets.iter().all(|set| {
                set <= &max_set
            }) {
                true => Some(id as u32),
                false => None,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert!(Set { red: 0 , blue: 2, green: 13 } <= Set { red: 12, blue: 14, green: 13 });
    }
}
