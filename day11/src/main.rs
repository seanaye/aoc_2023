use std::{
    fmt::Write,
    ops::{Add, Sub},
    str::FromStr,
};

use itertools::Itertools;

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input"), 10usize.pow(6)));
}

// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// impl Direction {
//     fn iter() -> impl Iterator<Item = Self> {
//         [Self::Up, Self::Down, Self::Left, Self::Right].into_iter()
//     }
// }

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

    // fn in_range(&self, coord: &Coord) -> bool {
    //     coord.x >= 0 && coord.y >= 0 && coord.x < self.width as i64 && coord.y < self.height as i64
    // }

    // pub fn adjacent<'a>(&'a self, coord: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
    //     Direction::iter()
    //         .map(move |dir| coord + &Coord::from(&dir))
    //         .filter(|coord| self.in_range(coord))
    // }

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

    pub fn insert_row(&mut self, at: &usize, to_add: impl Iterator<Item = T>) {
        let new_row = to_add.take(self.width);
        let len = self.inner.len();
        let i = self.width * at;
        self.inner.splice(i..i, new_row);
        let cur = self.inner.len();
        self.height += 1;
        assert!(len + self.width == cur);
    }

    pub fn insert_col(&mut self, at: &usize, to_add: impl Iterator<Item = T>) {
        let new_col = to_add.take(self.height);
        let len = self.inner.len();
        self.width += 1;
        for (i, e) in new_col.enumerate() {
            self.inner.insert(at + i * self.width, e);
        }
        let cur = self.inner.len();
        assert!(len + self.height == cur);
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

// impl From<&Direction> for Coord {
//     fn from(value: &Direction) -> Self {
//         match value {
//             Direction::Up => Self { x: 0, y: -1 },
//             Direction::Down => Self { x: 0, y: 1 },
//             Direction::Right => Self { x: 1, y: 0 },
//             Direction::Left => Self { x: -1, y: 0 },
//         }
//     }
// }

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

impl Coord {
    pub fn magnitude(&self) -> u64 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Galaxy,
    Debug,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            '+' => Self::Debug,
            _ => panic!("Invalid tile"),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Galaxy => '#',
            Tile::Debug => '+',
        }
    }
}

impl FromStr for Grid<Tile> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let inner = s
            .lines()
            .flat_map(|line| line.chars().map(Tile::from))
            .collect();
        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

struct OffsetGrid {
    grid: Grid<Tile>,
    offset: usize,
    expand_rows: Vec<usize>,
    expand_cols: Vec<usize>,
}

impl OffsetGrid {
    fn new(grid: Grid<Tile>, offset: usize) -> Self {
        let empty_cols = (0..grid.width)
            .filter(|col| grid.iter_col(*col).all(|(_, tile)| tile == &Tile::Empty))
            .collect::<Vec<_>>();

        let empty_rows = (0..grid.height)
            .filter(|row| grid.iter_row(*row).all(|(_, tile)| tile == &Tile::Empty))
            .collect::<Vec<_>>();

        Self {
            grid,
            offset,
            expand_rows: empty_rows,
            expand_cols: empty_cols,
        }
    }

    fn count_expanded_rows_before(&self, this_row: usize) -> usize {
        self.expand_rows
            .iter()
            .filter(|row| row < &&this_row)
            .count()
    }

    fn count_expanded_cols_before(&self, this_col: usize) -> usize {
        self.expand_cols
            .iter()
            .filter(|col| col < &&this_col)
            .count()
    }

    fn iter_with_offset(&self) -> impl Iterator<Item = (Coord, &'_ Tile)> {
        self.grid.iter_coord().map(|(coord, tile)| {
            let x_offset = self.count_expanded_cols_before(coord.x as usize) * self.offset;
            let y_offset = self.count_expanded_rows_before(coord.y as usize) * self.offset;
            (
                Coord {
                    x: coord.x + x_offset as i64,
                    y: coord.y + y_offset as i64,
                },
                tile,
            )
        })
    }
}


fn part_one(s: &str) -> u64 {
    let grid = Grid::<Tile>::from_str(s).unwrap();
    let offset_grid = OffsetGrid::new(grid, 1);
    offset_grid.iter_with_offset()
        .filter(|(_, tile)| tile == &&Tile::Galaxy)
        .combinations(2)
        .map(|vec| {
            let diff = &vec[0].0 - &vec[1].0;
            diff.magnitude()
        })
        .sum()
}

fn part_two(s: &str, size: usize) -> u64 {
    let grid = Grid::<Tile>::from_str(s).unwrap();
    let offset_grid = OffsetGrid::new(grid, size - 1);
    offset_grid.iter_with_offset()
        .filter(|(_, tile)| tile == &&Tile::Galaxy)
        .combinations(2)
        .map(|vec| {
            let diff = &vec[0].0 - &vec[1].0;
            diff.magnitude()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_part_one() {
        assert_eq!(part_one(include_str!("../test1")), 374)
    }

    #[test]
    fn test_part_one_answer() {
        assert_eq!(part_one(include_str!("../input")), 9591768)
    }

    #[test]
    fn test_10_times_larger() {
        assert_eq!(part_two(include_str!("../test1"), 10), 1030)
    }

    #[test]
    fn test_100_times_larger() {
        assert_eq!(part_two(include_str!("../test1"), 100), 8410)
    }
}

