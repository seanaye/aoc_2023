use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::sync::Mutex;

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Working,
    Borken,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Unknown,
    Known(State),
}

impl Spring {
    fn is_option(&self) -> bool {
        matches!(self, Self::Unknown | Self::Known(State::Borken))
    }
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Known(State::Borken),
            '?' => Self::Unknown,
            '.' => Self::Known(State::Working),
            _ => panic!("Invalid char"),
        }
    }
}

struct Row {
    inner: Vec<Spring>,
    contiguous_broken: Vec<usize>,
}

impl Row {
    fn expand(&mut self, times: usize) {
        self.inner.push(Spring::Unknown);
        let cur = self.inner.len();
        self.inner = self
            .inner
            .clone()
            .into_iter()
            .cycle()
            .take(cur * times - 1)
            .collect();
        let contig_len = self.contiguous_broken.len();
        self.contiguous_broken = self
            .contiguous_broken
            .clone()
            .into_iter()
            .cycle()
            .take(contig_len * times)
            .collect()
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let inner = split.next().unwrap().chars().map(Spring::from).collect();
        let contiguous_broken = split
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .unwrap();

        Ok(Self {
            inner,
            contiguous_broken,
        })
    }
}

struct RowSlice<'a, 'b> {
    inner: &'a [Spring],
    contiguous_broken: &'a [usize],
    row: &'a Row,
    cache: &'b Mutex<HashMap<(&'a [Spring], usize), usize>>,
}

impl Debug for RowSlice<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowSlice")
            .field("inner", &self.inner)
            .field("contiguous_broken", &self.contiguous_broken)
            .finish()
    }
}

impl<'a, 'b> RowSlice<'a, 'b> {
    fn consume_next(&self, consume: bool) -> (Option<Self>, usize) {
        // if we used all the required broken then this is valid
        let count_remaining = self
            .inner
            .iter()
            .filter(|x| x == &&Spring::Known(State::Borken))
            .count();
        let target = match (consume, self.contiguous_broken.first(), count_remaining) {
            (_, Some(x), _) => *x,
            (false, None, 0) => {
                return (None, 1);
            }
            (false, None, _) => return (None, 0),
            (true, None, _) => return (None, 0),
        };

        let mut iter = self.inner.iter();
        let start = iter.len();
        // advance to next place
        // if there are no more places then this is invalid
        let next = match iter.find(|s| s.is_option()) {
            Some(x) => x,
            None => return (None, 0),
        };

        match (consume, next) {
            (false, Spring::Unknown) => {
                let out = RowSlice {
                    inner: iter.as_slice(),
                    contiguous_broken: self.contiguous_broken,
                    row: self.row,
                    cache: self.cache,
                };
                return (Some(out), 0);
            }
            (false, _) => return (None, 0),
            _ => (),
        };

        let valid = (&mut iter)
            .take(target - 1)
            .filter(|s| s.is_option())
            .count()
            == target - 1;

        // if the next is also option this is invalid because its not contiguous
        let cur = start - iter.len();
        let val = self.inner.get(cur);

        let invalid = match val {
            Some(Spring::Known(State::Borken)) => true,
            Some(Spring::Known(State::Working)) => false,
            Some(Spring::Unknown) => {
                // we are now forced to not consume this
                // dbg!(("not consume", cur, val, &self));
                iter.next();
                false
            }
            None => false,
        };

        if !valid || invalid {
            return (None, 0);
        }

        let out = RowSlice {
            inner: iter.as_slice(),
            contiguous_broken: &self.contiguous_broken[1..],
            row: self.row,
            cache: self.cache,
        };
        (Some(out), 0)
    }

    fn sum(&mut self) -> usize {
        let sum_remaining = self.contiguous_broken.iter().sum();
        let cache_key = (self.inner, sum_remaining);
        if let Some(res) = self.cache.lock().unwrap().get(&cache_key) {
            return *res;
        }

        let (next_a, count_a) = self.consume_next(true);
        let (next_b, count_b) = self.consume_next(false);
        let a = next_a.map(|mut a| a.sum()).unwrap_or(0) + count_a;
        let b = next_b.map(|mut b| b.sum()).unwrap_or(0) + count_b;
        let res = a + b;
        // self.cache
        //     .lock()
        //     .unwrap()
        //     .insert((&self.inner, sum_remaining), res);
        res
    }

    fn from_row(row: &'a Row, cache: &'b Mutex<HashMap<(&'a [Spring], usize), usize>>) -> Self {
        Self {
            inner: &row.inner,
            contiguous_broken: &row.contiguous_broken,
            row,
            cache,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Decision {
    One(State),
    Either,
    // indicates dead end
    Invalid,
    Complete,
}

#[derive(Clone)]
struct DecisionNode<'a> {
    current: Vec<State>,
    active_index: usize,
    row: &'a Row,
}

impl PartialEq for DecisionNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current && self.active_index == other.active_index
    }
}

impl Eq for DecisionNode<'_> {}

impl Hash for DecisionNode<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current.hash(state);
        self.active_index.hash(state);
    }
}

impl std::fmt::Debug for DecisionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecisionNode")
            .field("current", &self.current)
            .field("active_index", &self.active_index)
            .finish()
    }
}

impl<'a> DecisionNode<'a> {
    fn empty(row: &'a Row) -> Self {
        Self {
            row,
            active_index: 0,
            current: Vec::new(),
        }
    }

    fn last_contiguous_broken(&self) -> usize {
        self.current
            .iter()
            .rev()
            .take_while(|s| s == &&State::Borken)
            .count()
    }

    fn is_valid(&self) -> bool {
        let out = self.last_contiguous_broken()
            <= *self
                .row
                .contiguous_broken
                .get(self.active_index)
                .unwrap_or(&0usize);
        let enough_left = self.remaining_borken() <= self.remaining_total();
        out && enough_left
    }

    fn remaining_borken(&self) -> usize {
        self.row
            .contiguous_broken
            .iter()
            .sum::<usize>()
            .saturating_sub(self.current.iter().filter(|f| f == &&State::Borken).count())
    }

    fn remaining_total(&self) -> usize {
        self.row.inner.len() - self.current.len()
    }

    fn remaining_slice(&self, offset: usize) -> (&'a [Spring], usize) {
        (
            &self.row.inner[self.current.len().saturating_sub(offset)..],
            self.row.contiguous_broken[self.active_index..].iter().sum(),
        )
    }

    fn decision(&self) -> Decision {
        match (self.is_valid(), self.row.inner.get(self.current.len())) {
            (false, _) => Decision::Invalid,
            (true, Some(Spring::Known(x))) => Decision::One(*x),
            (true, Some(Spring::Unknown)) => Decision::Either,
            (true, None) => Decision::Complete,
        }
    }

    fn append(&self, state: State) -> DecisionNode<'a> {
        let mut next = self.clone();
        if state == State::Working && next.current.last() == Some(&State::Borken) {
            next.active_index += 1;
        }
        next.current.push(state);
        next
    }
}

/// hashmap of counts of success
/// from remaining tiles
/// retrieve memo

struct DecisionNodeIterator<'a> {
    to_visit: Vec<DecisionNode<'a>>,
    visited: HashMap<(&'a [Spring], usize), usize>,
}

impl<'a> Iterator for DecisionNodeIterator<'a> {
    type Item = (DecisionNode<'a>, Decision);

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.to_visit.pop()?;
        let decision = cur.decision();
        // dbg!((decision, &cur, &slice));
        match decision {
            Decision::Either => {
                let left = cur.append(State::Working);
                let right = cur.append(State::Borken);
                self.to_visit.push(left);
                self.to_visit.push(right)
            }
            Decision::One(s) => {
                let next = cur.append(s);

                self.to_visit.push(next);
            }
            Decision::Complete => {
                // let to_insert = cur.remaining_slice(1);
                // dbg!(&to_insert);
                // self.visited.insert(to_insert, decision);
            }
            Decision::Invalid => {}
        }
        Some((cur, decision))
    }
}

impl<'a> DecisionNodeIterator<'a> {
    fn new_from_row(row: &'a Row) -> Self {
        let first = DecisionNode::empty(row);
        Self {
            to_visit: vec![first],
            visited: HashMap::new(),
        }
    }
}

fn count_line(r: Row) -> usize {
    let cache = Mutex::new(HashMap::new());
    RowSlice::from_row(&r, &cache).sum()
}

fn count_line_cache<'a>(r: &'a Row, cache: &Mutex<HashMap<(&'a [Spring], usize), usize>>) -> usize {
    RowSlice::from_row(r, cache).sum()
}

fn count_line_old(r: &Row) -> usize {
    let iter = DecisionNodeIterator::new_from_row(r);
    iter.filter(|(node, decision)| {
        // if decision == &Decision::Complete {
        //     dbg!(node);
        // }
        decision == &Decision::Complete
    })
    .count()
}

fn part_one(s: &str) -> usize {
    s.lines().map(|l| l.parse().unwrap()).map(count_line).sum()
}

fn part_two(s: &str) -> usize {
    let cache = Mutex::new(HashMap::new());
    s.lines()
        .map(|l| {
            let mut r = l.parse::<Row>().unwrap();
            r.expand(5);
            r
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|r| count_line_cache(r, &cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(part_one(include_str!("../example")), 21)
    }

    #[test]
    #[ignore]
    fn compare_old() {
        let s = include_str!("../input");
        for (i, l) in s.lines().enumerate() {
            dbg!(i, &l);
            assert_eq!(
                count_line(l.parse().unwrap()),
                count_line_old(&l.parse().unwrap())
            );
        }
    }

    #[test]
    fn compare_line_12() {
        let s = include_str!("../input");
        assert_eq!(
            count_line(s.lines().nth(11).unwrap().parse::<Row>().unwrap()),
            2
        );
    }

    #[test]
    fn test_example_line_one() {
        let s = include_str!("../example")
            .lines()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_line(s), 1)
    }

    #[test]
    fn test_example_line_two() {
        let s = include_str!("../example")
            .lines()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_line(s), 4)
    }

    #[test]
    fn test_example_line_three() {
        let s = include_str!("../example")
            .lines()
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_line(s), 1)
    }

    #[test]
    fn test_example_line_four() {
        let s = include_str!("../example")
            .lines()
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_line(s), 1)
    }

    #[test]
    fn test_example_line_five() {
        let s = include_str!("../example")
            .lines()
            .nth(4)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_line(s), 4)
    }

    #[test]
    fn test_example_line_six() {
        let s = include_str!("../example")
            .lines()
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(count_line(s), 10)
    }

    #[test]
    fn test_part_two_example_line_six() {
        let mut s = include_str!("../example")
            .lines()
            .nth(5)
            .unwrap()
            .parse::<Row>()
            .unwrap();
        s.expand(5);
        assert_eq!(count_line(s), 506250)
    }

    #[test]
    fn test_part_two_example_line_one() {
        let mut s = include_str!("../example")
            .lines()
            .next()
            .unwrap()
            .parse::<Row>()
            .unwrap();
        s.expand(5);
        assert_eq!(count_line(s), 1)
    }

    #[test]
    fn test_part_two_example_line_two() {
        let mut s = include_str!("../example")
            .lines()
            .nth(1)
            .unwrap()
            .parse::<Row>()
            .unwrap();
        s.expand(5);
        assert_eq!(count_line(s), 16384)
    }

    #[test]
    fn test_part_two_example_line_three() {
        let mut s = include_str!("../example")
            .lines()
            .nth(2)
            .unwrap()
            .parse::<Row>()
            .unwrap();
        s.expand(5);
        assert_eq!(count_line(s), 1)
    }

    #[test]
    fn test_part_two_example_line_four() {
        let mut s = include_str!("../example")
            .lines()
            .nth(3)
            .unwrap()
            .parse::<Row>()
            .unwrap();
        s.expand(5);
        assert_eq!(count_line(s), 16)
    }

    #[test]
    fn test_part_two_example_line_five() {
        let mut s = include_str!("../example")
            .lines()
            .nth(4)
            .unwrap()
            .parse::<Row>()
            .unwrap();
        s.expand(5);
        assert_eq!(count_line(s), 2500)
    }

    #[test]
    fn test_part_two_weird_line_one() {
        let mut s = include_str!("../example")
            .lines()
            .next()
            .unwrap()
            .parse::<Row>()
            .unwrap();
        s.expand(2);
        assert_eq!(count_line(s), 1)
    }

    #[test]
    fn test_anwer_one() {
        assert_eq!(part_one(include_str!("../input")), 7090)
    }
}

