use memoize::memoize;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
    str::FromStr,
};

fn main() {
    println!("{}", part_one(include_str!("../input")));
    println!("{}", part_two(include_str!("../input"), 1000000000));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Round,
    Square,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => Tile::Round,
            '#' => Tile::Square,
            _ => Tile::Empty,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Grid<T> {
    inner: Vec<T>,
    dim: Dim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dim {
    width: usize,
    height: usize,
}

impl Dim {
    pub fn coord_from_index(&self, index: usize) -> Coord {
        Coord {
            x: index as i64 % self.width as i64,
            y: index as i64 / self.width as i64,
        }
    }

    pub fn index_from_coord(&self, coord: &Coord) -> usize {
        (coord.y * self.width as i64 + coord.x) as usize
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Coord {
    type Output = Coord;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, coord: &Coord) -> &T {
        self.inner.get(self.dim.index_from_coord(coord)).unwrap()
    }

    pub fn get_mut(&mut self, coord: &Coord) -> &mut T {
        self.inner
            .get_mut(self.dim.index_from_coord(coord))
            .unwrap()
    }

    pub fn iter_coord(&self) -> impl Iterator<Item = (Coord, &'_ T)> {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, e)| (self.dim.coord_from_index(i), e))
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = (Coord, &'_ T)> {
        let row = row as i64;
        (0..self.dim.width as i64).map(move |col| {
            (
                Coord { x: col, y: row },
                self.get(&Coord { x: col, y: row }),
            )
        })
    }

    pub fn iter_row_mut(&'_ mut self, row: usize) -> impl Iterator<Item = (Coord, &'_ mut T)> + '_ {
        let row = row as i64;
        let dim = self.dim;
        self.inner
            .iter_mut()
            .enumerate()
            .filter_map(move |(idx, t)| {
                let coord = dim.coord_from_index(idx);
                match coord.y == row {
                    true => Some((coord, t)),
                    false => None,
                }
            })
    }

    pub fn iter_col_mut(&'_ mut self, col: usize) -> impl Iterator<Item = (Coord, &'_ mut T)> + '_ {
        let col = col as i64;
        let dim = self.dim;
        self.inner
            .iter_mut()
            .enumerate()
            .filter_map(move |(idx, t)| {
                let coord = dim.coord_from_index(idx);
                match coord.x == col {
                    true => Some((coord, t)),
                    false => None,
                }
            })
    }

    pub fn iter_col(&self, col: usize) -> impl Iterator<Item = (Coord, &'_ T)> {
        let col = col as i64;
        (0..self.dim.height as i64).map(move |row| {
            (
                Coord { x: col, y: row },
                self.get(&Coord { x: col, y: row }),
            )
        })
    }

    pub fn width(&self) -> usize {
        self.dim.width
    }

    pub fn height(&self) -> usize {
        self.dim.height
    }
}

impl FromStr for Grid<Tile> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        let inner: Vec<_> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.into())
            .collect();
        assert_eq!(inner.len(), width * height);
        Ok(Self {
            dim: Dim { width, height },
            inner,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn mutate_slice(s: &mut [Tile]) {
    for i in 0..s.len() {
        if let Tile::Empty = s[i] {
            // find next movable
            for j in i..s.len() {
                if s[j] == Tile::Round {
                    s.swap(i, j);
                    break;
                }
                if s[j] == Tile::Square {
                    break;
                }
            }
        }
    }
}

#[memoize]
fn move_vec(mut slice: Vec<Tile>, dir: Direction) -> Vec<Tile> {
    let should_reverse = matches!(dir, Direction::East | Direction::South);

    if should_reverse {
        slice.reverse();
    }
    mutate_slice(&mut slice);
    if should_reverse {
        slice.reverse();
    }
    slice
}

impl Grid<Tile> {
    fn shift(&mut self, dir: Direction) {
        let wl = match dir {
            Direction::North | Direction::South => self.width(),
            Direction::East | Direction::West => self.height(),
        };

        for i in 0..wl {
            let slice = match dir {
                Direction::North | Direction::South => {
                    self.iter_col(i).map(|(_, t)| *t).collect::<Vec<_>>()
                }
                Direction::West | Direction::East => {
                    self.iter_row(i).map(|(_, t)| *t).collect::<Vec<_>>()
                }
            };

            let mut iter = move_vec(slice, dir).into_iter();

            match dir {
                Direction::North | Direction::South => self
                    .iter_col_mut(i)
                    .for_each(|(_, t)| *t = iter.next().unwrap()),
                Direction::East | Direction::West => self
                    .iter_row_mut(i)
                    .for_each(|(_, t)| *t = iter.next().unwrap()),
            };
        }
    }

    fn measure_load(&self) -> usize {
        let h = self.height();
        (0..h)
            .map(|i| {
                let row_load = h - i;
                self.iter_row(i)
                    .filter(|(_, t)| matches!(t, &Tile::Round))
                    .count()
                    * row_load
            })
            .sum()
    }

    fn cycle(&mut self) {
        self.shift(Direction::North);
        self.shift(Direction::West);
        self.shift(Direction::South);
        self.shift(Direction::East);
    }
}

fn part_one(s: &str) -> usize {
    let mut grid: Grid<Tile> = s.parse().unwrap();
    grid.shift(Direction::North);
    grid.measure_load()
}

fn part_two(s: &str, cycles: usize) -> usize {
    let mut grid: Grid<Tile> = s.parse().unwrap();
    let mut cache: HashMap<Grid<Tile>, usize> = HashMap::new();
    let mut found_cycle = false;
    (0..cycles)
        .scan(0, |state, _| {
            if state == &(cycles - 1) {
                return None;
            }

            grid.cycle();
            let mut to_incr = 1;
            if !found_cycle {
                if let Some(c) = cache.get(&grid) {
                    let cycle_len = *state - c;
                    dbg!(&cycle_len, &state, &c);
                    let a = (cycles - *state) / cycle_len;
                    let n = a * cycle_len;
                    found_cycle = true;
                    dbg!(&n);
                    to_incr = n
                }
            }
            cache.insert(grid.clone(), *state);
            *state += to_incr;
            Some(*state)
        })
        .count();
    grid.measure_load()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_one(include_str!("../example")), 136)
    }

    #[test]
    fn part_one_answer() {
        assert_eq!(part_one(include_str!("../input")), 110821)
    }

    #[test]
    fn cycle3() {
        let mut start: Grid<Tile> = include_str!("../example").parse().unwrap();
        let end: Grid<Tile> = include_str!("../3cycle").parse().unwrap();
        for _ in 0..3 {
            start.cycle();
        }
        assert_eq!(start, end)
    }

    #[test]
    fn cycle1() {
        let mut start: Grid<Tile> = include_str!("../example").parse().unwrap();
        let end: Grid<Tile> = include_str!("../1cycle").parse().unwrap();
        start.cycle();
        assert_eq!(start, end)
    }

    #[test]
    #[ignore]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example"), 1000000000), 64)
    }
}

