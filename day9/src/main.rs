use std::str::FromStr;

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

#[derive(Debug)]
struct Line {
    inner: Vec<i64>,
}

impl Line {
    fn diff(&self) -> Line {
        let mut inner = Vec::new();
        for i in 0..self.inner.len() - 1 {
            inner.push(self.inner[i + 1] - self.inner[i]);
        }
        Line { inner }
    }

    fn is_end(&self) -> bool {
        self.inner.iter().all(|v| v == &0)
    }

    fn elem(&self, direction: &Direction) -> &i64 {
        match direction {
            Direction::Forward => self.inner.last().unwrap_or(&0),
            Direction::Backward => self.inner.first().unwrap_or(&0),
        }
    }

    fn push(&mut self, v: i64, direction: &Direction) {
        match direction {
            Direction::Forward => self.inner.push(v),
            Direction::Backward => self.inner.insert(0, v),
        }
    }

    fn next_val(&self, other: &Line, direction: &Direction) -> i64 {
        match direction {
            Direction::Forward => self.elem(direction) + other.elem(direction),
            Direction::Backward => self.elem(direction) - other.elem(direction)
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let original = s
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<i64>, _>>();


        let original = original.map_err(|_| ()).unwrap();
        Ok(Self { inner: original })
    }
}

struct History {
    inner: Vec<Line>,
}

impl Iterator for History {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        let last = self.inner.last()?;
        if last.is_end() {
            return None;
        }
        let next = last.diff();
        self.inner.push(next);
        Some(())
    }
}

enum Direction {
    Forward,
    Backward,
}

impl History {
    fn finish(&mut self) {
        while self.next().is_some() {}
    }

    fn extrapolate(&mut self, direction: &Direction) {
        self.finish();
        let end = self.inner.len();
        for i in (0..end).rev() {
            if i == end - 1 {
                self.inner.get_mut(i).unwrap().push(0, direction);
            } else {
                let (slice1, slice2) = self.inner.split_at_mut(i + 1);
                let cur = slice1.last_mut().unwrap();
                let prev = slice2.first().unwrap();
                let to_append = cur.next_val(prev, direction);
                cur.push(to_append, direction);
            }
        }
    }

    fn estimate(&mut self, direction: &Direction) -> i64 {
        self.extrapolate(direction);
        dbg!(&self.inner);
        *self.inner.first().unwrap().elem(direction)
    }
}

struct Board {
    histories: Vec<History>,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(Line::from_str)
            .collect::<Result<Vec<Line>, _>>();

        let lines = lines.map_err(|_| ()).unwrap();

        let histories = lines
            .into_iter()
            .map(|l| History { inner: vec![l] })
            .collect::<Vec<_>>();

        Ok(Self { histories })
    }
}

fn part_one(input: &str) -> i64 {
    let mut board: Board = input.parse().unwrap();
    board
        .histories
        .iter_mut()
        .map(|h| h.estimate(&Direction::Forward))
        .sum()
}

fn part_two(input: &str) -> i64 {
    let mut board: Board = input.parse().unwrap();
    board
        .histories
        .iter_mut()
        .map(|h| h.estimate(&Direction::Backward))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(include_str!("../test")), 114)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(include_str!("../test")), 2)
    }
}

