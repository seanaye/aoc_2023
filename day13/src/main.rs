use std::{
    cmp::{max, min},
    fmt::Write,
    ops::{Add, Sub},
    str::FromStr,
};

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

#[derive(PartialEq, Eq)]
struct Grid<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn get(&self, coord: &Coord) -> &T {
        self.inner
            .get(coord.y as usize * self.width + coord.x as usize)
            .unwrap()
    }

    fn coord_from_index(&self, index: usize) -> Coord {
        Coord {
            x: index as i64 % self.width as i64,
            y: index as i64 / self.width as i64,
        }
    }

    pub fn iter_coord(&self) -> impl Iterator<Item = (Coord, &'_ T)> {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, e)| (self.coord_from_index(i), e))
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = (Coord, &'_ T)> {
        let row = row as i64;
        (0..self.width as i64).map(move |col| {
            (
                Coord { x: col, y: row },
                self.get(&Coord { x: col, y: row }),
            )
        })
    }

    pub fn iter_col(&self, col: usize) -> impl Iterator<Item = (Coord, &'_ T)> {
        let col = col as i64;
        (0..self.height as i64).map(move |row| {
            (
                Coord { x: col, y: row },
                self.get(&Coord { x: col, y: row }),
            )
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> std::fmt::Debug for &Grid<T>
where
    T: Into<char> + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in 0..self.height * self.width {
            if c % self.width == 0 {
                f.write_char('\n')?
            }
            f.write_char((*self.inner.get(c).unwrap()).into())?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => panic!("Invalid tile"),
        }
    }
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

impl FromStr for Grid<Tile> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.lines().flat_map(|l| l.chars().map(Tile::from)).collect();
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

impl Sub for &Tile {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Tile::Ash, Tile::Rock) => 1,
            (Tile::Rock, Tile::Ash) => 1,
            _ => 0,
        }
    }
}

// impl<T> Sub for Vec<T>
// where
//     for<'a> &'a T: Sub<Output = usize>,
// {
//     fn diff(&self, rhs: &T) -> usize {
//         self.iter().zip(rhs.iter()).map(|(a, b)| a - b).sum()
//     }
// }
//
fn diff_iter<'a, T: 'a>(a: impl Iterator<Item = &'a T>, b: impl Iterator<Item = &'a T>) -> usize
where
    for<'b> &'b T: Sub<Output = usize>,
{
    a.zip(b).map(|(a, b)| a - b).sum()
}

fn diff_vec(a: Vec<&Tile>, b: Vec<&Tile>) -> usize {
    a.iter().zip(b.iter()).map(|(a, b)| *a - *b).sum()
}

impl Grid<Tile> {
    pub fn find_mirror_col(&self, diff: usize) -> Option<usize> {
        (1..self.width).find(|col| {
            let c = *col;
            let distance_from_edge = min(c, self.width - c);
            let right = c..c + distance_from_edge;
            let left = (c - distance_from_edge..c).rev();
            let right_iter = right.map(|i| self.iter_col(i).map(|(_, v)| v).collect::<Vec<_>>());
            let left_iter = left.map(|i| self.iter_col(i).map(|(_, v)| v).collect::<Vec<_>>());
            dbg!(&right_iter, &left_iter);
            let f = right_iter.zip(left_iter).map(|(a, b)| diff_vec(a, b));
            f.sum::<usize>() == diff
        })
    }

    pub fn find_mirror_row(&self, diff: usize) -> Option<usize> {
        (1..self.height).find(|row| {
            let r = *row;
            let distance_from_edge = min(r, self.height - r);
            let top = r..r + distance_from_edge;
            let bottom = (r - distance_from_edge..r).rev();
            let top_iter = top.map(|i| self.iter_row(i).map(|(_, v)| v).collect::<Vec<_>>());
            let bottom_iter = bottom.map(|i| self.iter_row(i).map(|(_, v)| v).collect::<Vec<_>>());
            let f = top_iter.zip(bottom_iter).map(|(a, b)| diff_vec(a, b));
            f.sum::<usize>() == diff
        })
    }

    pub fn sum(&self, diff: usize) -> usize {
        match self.find_mirror_col(diff) {
            Some(x) => x,
            None => self.find_mirror_row(diff).unwrap() * 100,
        }
    }
}

fn part_one(s: &str) -> usize {
    s.split("\n\n")
        .map(|s| s.parse::<Grid<Tile>>().unwrap().sum(0))
        .sum()
}

fn part_two(s: &str) -> usize {
    s.split("\n\n")
        .map(|s| s.parse::<Grid<Tile>>().unwrap().sum(1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_col() {
        assert_eq!(
            include_str!("../example")
                .split("\n\n")
                .next()
                .unwrap()
                .parse::<Grid<Tile>>()
                .unwrap()
                .find_mirror_col(0)
                .unwrap(),
            5
        )
    }

    #[test]
    fn example_row() {
        assert_eq!(
            include_str!("../example")
                .split("\n\n")
                .nth(1)
                .unwrap()
                .parse::<Grid<Tile>>()
                .unwrap()
                .find_mirror_row(0)
                .unwrap(),
            4
        )
    }

    #[test]
    fn example() {
        assert_eq!(part_one(include_str!("../example")), 405)
    }

    #[test]
    fn example_pt2() {
        assert_eq!(part_two(include_str!("../example")), 400)
    }
}

