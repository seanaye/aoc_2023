use std::{collections::HashSet, ops::Add, str::FromStr};

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            x => panic!("Invalid tile {}", x),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Tile {
    // returns the output direction if its possible to enter the pipe from the starting direction
    fn enter_from(&self, dir: &Direction) -> Option<Direction> {
        match (self, dir) {
            (Tile::NorthSouth, Direction::North) => Some(Direction::South),
            (Tile::NorthSouth, Direction::South) => Some(Direction::North),
            (Tile::EastWest, Direction::East) => Some(Direction::West),
            (Tile::EastWest, Direction::West) => Some(Direction::East),
            (Tile::NorthEast, Direction::North) => Some(Direction::East),
            (Tile::NorthEast, Direction::East) => Some(Direction::North),
            (Tile::NorthWest, Direction::North) => Some(Direction::West),
            (Tile::NorthWest, Direction::West) => Some(Direction::North),
            (Tile::SouthWest, Direction::South) => Some(Direction::West),
            (Tile::SouthWest, Direction::West) => Some(Direction::South),
            (Tile::SouthEast, Direction::South) => Some(Direction::East),
            (Tile::SouthEast, Direction::East) => Some(Direction::South),
            (Tile::Start, x) => Some(*x),
            _ => None,
        }
    }

    fn is_vertex(&self) -> bool {
        !matches!(self, Tile::NorthSouth | Tile::EastWest)
    }
}

struct Grid<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
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

impl From<&Direction> for Coord {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => Self { x: 0, y: -1 },
            Direction::South => Self { x: 0, y: 1 },
            Direction::East => Self { x: 1, y: 0 },
            Direction::West => Self { x: -1, y: 0 },
        }
    }
}

impl Coord {
    fn offset(&self, dir: &Direction) -> Coord {
        let offset: Coord = dir.into();
        &offset + self
    }
}

impl<T> Grid<T> {
    fn get(&self, coord: &Coord) -> &T {
        self.inner
            .get(coord.y as usize * self.width + coord.x as usize)
            .unwrap()
    }

    fn in_range(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.y >= 0 && coord.x < self.width as i64 && coord.y < self.height as i64
    }

    // fn adjacent(&self, coord: &Coord) -> Vec<Coord> {
    //     coord
    //         .range(-1, 1)
    //         .into_iter()
    //         .filter(|coord| self.in_range(coord))
    //         .collect()
    // }

    fn coord_from_index(&self, index: usize) -> Coord {
        Coord {
            x: index as i64 % self.width as i64,
            y: index as i64 / self.width as i64,
        }
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl Grid<Tile> {
    fn find_start(&self) -> Coord {
        let index = self.into_iter().position(|t| t == &Tile::Start).unwrap();
        self.coord_from_index(index)
    }

    fn paths(&self, coord: Coord, direction: Direction) -> Paths<'_> {
        Paths::new(self, coord, direction)
    }

    fn traverse_from(&self, coord: &Coord, direction: &Direction) -> Vec<(Coord, Direction)> {
        if let Some(next_dir) = self.get(coord).enter_from(direction) {
            let next_coord = coord.offset(&next_dir);
            return vec![(next_coord, next_dir.inverse())];
        }
        vec![]
    }

    fn find_starts(&self) -> impl Iterator<Item = (Coord, Direction)> + '_ {
        let coord = self.find_start();
        [
            Direction::North,
            Direction::East,
            Direction::West,
            Direction::South,
        ]
        .iter()
        .filter_map(move |dir| {
            let dest = coord.offset(dir);
            if !self.in_range(&dest) {
                return None;
            }
            let inverse = dir.inverse();
            let opts = self.get(&dest).enter_from(&inverse);
            opts.map(|_| (coord, *dir))
        })
    }
}

impl FromStr for Grid<Tile> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        let inner = s
            .lines()
            .flat_map(|line| line.chars().map(Tile::from))
            .collect();
        Ok(Self {
            inner,
            height,
            width,
        })
    }
}

#[derive(Debug)]
struct Segment {
    coord: Coord,
    enter_from: Direction,
}

struct Paths<'a> {
    visited: HashSet<Coord>,
    grid: &'a Grid<Tile>,
    to_visit: Vec<Segment>,
}

impl<'a> Paths<'a> {
    fn new(grid: &'a Grid<Tile>, start: Coord, direction: Direction) -> Paths<'a> {
        Paths {
            grid,
            visited: HashSet::new(),
            to_visit: vec![Segment {
                coord: start,
                enter_from: direction,
            }],
        }
    }
}

impl Iterator for Paths<'_> {
    type Item = Segment;
    fn next(&mut self) -> Option<Self::Item> {
        let segment = self.to_visit.pop()?;
        if self.visited.contains(&segment.coord) {
            return Some(segment);
        }
        self.visited.insert(segment.coord);

        let direction = self.grid.traverse_from(&segment.coord, &segment.enter_from);

        let next = direction
            .into_iter()
            .map(|(coord, dir)| Segment {
                coord,
                enter_from: dir,
            })
            .filter(|segment| self.grid.in_range(&segment.coord));
        self.to_visit.extend(next);
        Some(segment)
    }
}

fn part_one(input: &str) -> usize {
    let grid: Grid<Tile> = input.parse().unwrap();

    let (coord, direction) = grid.find_starts().next().unwrap();
    grid.paths(coord, direction).count() / 2
}

fn part_two(input: &str) -> usize {
    let grid: Grid<Tile> = input.parse().unwrap();

    let (coord, direction) = grid.find_starts().next().unwrap();
    let points = grid.paths(coord, direction).collect::<Vec<_>>();

    let vertices = points
        .iter()
        .filter_map(|seg| match grid.get(&seg.coord).is_vertex() {
            true => Some(seg.coord),
            false => None,
        })
        .collect::<Vec<_>>();
    internal_points(points.len() - 1, &vertices)
}

fn shoelace(vertices: &[Coord]) -> i64 {
    let mut sum = 0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        sum += vertices[i].x * vertices[j].y - vertices[j].x * vertices[i].y;
    }
    sum.abs() / 2
}

fn internal_points(boundary_points: usize, vertices: &[Coord]) -> usize {
    let area = shoelace(vertices);
    let internal_points = area - boundary_points as i64 / 2 + 1;
    internal_points as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_part_one() {
        assert_eq!(part_one(include_str!("../test1")), 4)
    }

    #[test]
    fn test_two_part_one() {
        assert_eq!(part_one(include_str!("../test2")), 8)
    }

    #[test]
    fn part_one_answer() {
        assert_eq!(part_one(include_str!("../input")), 6812)
    }

    #[test]
    fn part_two_test_one() {
        assert_eq!(part_two(include_str!("../test3")), 4)
    }

    #[test]
    fn part_two_test_two() {
        assert_eq!(part_two(include_str!("../test4")), 10)
    }
}

