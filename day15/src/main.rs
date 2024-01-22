use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, Hasher},
    str::FromStr,
};

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

#[derive(Default, Clone, Copy, Debug)]
struct MyHasher {
    current: u64,
}

impl MyHasher {
    pub fn write_char(&mut self, c: u8) {
        self.current += c as u64;
        self.current *= 17;
        let remainder = self.current % 256;
        self.current = remainder;
    }
}

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.current
    }

    fn write(&mut self, bytes: &[u8]) {
        bytes
            .iter()
            .filter(|u| !u.is_ascii_whitespace() && u.is_ascii())
            .for_each(|u| self.write_char(*u))
    }
}

fn hash_str(s: &str) -> u64 {
    let mut h = MyHasher::default();
    s.hash(&mut h);
    let out = h.finish();
    out
}

fn part_one(input: &str) -> u64 {
    input.split(',').map(hash_str).sum()
}

#[derive(Debug)]
struct Inst<'a> {
    label: &'a str,
    power: u64,
}

#[derive(Debug)]
enum Dir<'a> {
    Minus { label: &'a str },
    Equal(Inst<'a>),
}

impl<'a> From<&'a str> for Dir<'a> {
    fn from(s: &'a str) -> Dir<'a> {
        let a = s
            .trim()
            .split(['=', '-'])
            .filter(|a| !a.is_empty())
            .collect::<Vec<_>>();
        match a.len() {
            1 => Self::Minus { label: a[0] },
            2 => Self::Equal(Inst {
                label: a[0],
                power: a[1].parse().unwrap(),
            }),
            _ => panic!("invalid input"),
        }
    }
}

struct BoxesMap<'a>(HashMap<u64, Vec<Inst<'a>>>);

impl<'a> BoxesMap<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn operation(&mut self, s: &'a str) {
        let dir: Dir = s.into();
        let label = match &dir {
            Dir::Minus { label } => label,
            Dir::Equal(inst) => inst.label,
        };
        let b = self.0.entry(hash_str(label)).or_default();
        match dir {
            Dir::Minus { label } => {
                b.retain(|i| i.label != label);
            }
            Dir::Equal(inst) => match b.iter_mut().find(|i| i.label == inst.label) {
                Some(i) => *i = inst,
                None => b.push(inst),
            },
        }
    }

    fn sum(&self) -> u64 {
        self.0
            .iter()
            .map(|(boxnum, thisbox)| {
                thisbox
                    .iter()
                    .enumerate()
                    .map(|(idx, lens)| {
                        let boxnum_mul = boxnum + 1;
                        let lensnum_mul = idx + 1;
                        lens.power * boxnum_mul * lensnum_mul as u64
                    })
                    .sum::<u64>()
            })
            .sum()
    }
}

fn part_two(input: &str) -> u64 {
    let mut bm = BoxesMap::new();
    input.split(',').for_each(|s| bm.operation(s));
    bm.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut h = MyHasher::default();
        "HASH".hash(&mut h);
        assert_eq!(h.finish(), 52)
    }

    #[test]
    fn example_ot() {
        let mut h = MyHasher::default();
        "ot=7".hash(&mut h);
        assert_eq!(h.finish(), 231)
    }

    #[test]
    fn example2() {
        assert_eq!(part_one(include_str!("../example")), 1320)
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_two(include_str!("../example")), 145)
    }
}

