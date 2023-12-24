use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use nom::bytes::complete::{tag, take_till, take_while1};
use nom::character::complete::alpha1;
use nom::multi::many1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use num::integer::lcm;

fn main() {
    dbg!(part_one(include_str!("../input")));
    dbg!(part_two(include_str!("../input")));
}

trait Parse: Sized {
    fn parse(s: &str) -> IResult<&str, Self>;
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    children: NodeChildren,
}

impl Node {
    fn is_start_node(&self) -> bool {
        self.id.ends_with('A')
    }

    fn is_end_node(&self) -> bool {
        self.id.ends_with('Z')
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl Borrow<str> for Node {
    fn borrow(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct NodeChildren {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    instruction: String,
    nodes: HashSet<Node>,
}

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'l' => Direction::Left,
            'r' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

struct MapNodeIter<'a> {
    map: &'a Map,
    current: &'a Node,
    step: usize,
    is_end: fn(&Node) -> bool,
}

impl<'a> Iterator for MapNodeIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.is_end)(self.current) {
            return None;
        }
        let direction = self.map.get_direction(self.step);
        let next_id = match direction {
            Direction::Left => self.current.children.left.as_str(),
            Direction::Right => self.current.children.right.as_str(),
        };
        self.current = self.map.nodes.get(next_id).unwrap();
        self.step += 1;
        Some(self.current)
    }
}

impl Map {
    fn get_direction(&self, index: usize) -> Direction {
        let c = self
            .instruction
            .chars()
            .nth(index % self.instruction.len())
            .unwrap();
        c.into()
    }

    fn node_iter<'a>(&'a self, start: &'a Node, end: fn(&Node) -> bool) -> MapNodeIter<'a> {
        MapNodeIter {
            map: self,
            current: start,
            step: 0,
            is_end: end,
        }
    }

    fn nodes_iter(&self, end: fn(&Node) -> bool) -> NodesIterator<'_> {
        NodesIterator {
            cur: self
                .nodes
                .iter()
                .filter(|n| n.is_start_node())
                .map(|n| self.node_iter(n, end))
                .collect(),
        }
    }
}

fn part_two_is_end(node: &Node) -> bool {
    node.is_end_node()
}

fn part_one_is_end(node: &Node) -> bool {
    node.id == "ZZZ"
}

impl Parse for Map {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (next, inst) = take_till(|c: char| c.is_ascii_whitespace())(s)?;
        let (next, _) = take_till(|c: char| !c.is_ascii_whitespace())(next)?;
        let (next, nodes) = many1(Node::parse)(next)?;
        Ok((
            next,
            Map {
                instruction: inst.to_owned(),
                nodes: HashSet::from_iter(nodes),
            },
        ))
    }
}

impl Parse for Node {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (next, id) = take_till(|c: char| c.is_ascii_whitespace())(s)?;
        let (next, _) = tag(" = ")(next)?;
        let (next, inner) = delimited(tag("("), take_while1(|c: char| c != ')'), tag(")"))(next)?;
        let (_, (left, right)) = separated_pair(alpha1, tag(", "), alpha1)(inner)?;
        let (next, _) = take_till(|c: char| !c.is_ascii_whitespace())(next)?;
        Ok((
            next,
            Node {
                id: id.to_owned(),
                children: NodeChildren {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            },
        ))
    }
}

struct NodesIterator<'a> {
    cur: Vec<MapNodeIter<'a>>,
}

impl<'a> NodesIterator<'a> {
    fn lcm(self) -> usize {
        self.cur.into_iter().map(|n| n.count()).fold(1, lcm)
    }
}

fn part_one(s: &str) -> usize {
    let out = Map::parse(s);
    let (_, map) = out.unwrap();
    let start = map.nodes.get("AAA").unwrap();
    map.node_iter(start, part_one_is_end).count()
}

fn part_two(s: &str) -> usize {
    let out = Map::parse(s);
    let (_, map) = out.unwrap();
    map.nodes_iter(part_two_is_end).lcm()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_example() {
        assert_eq!(part_one(include_str!("../test")), 6)
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(include_str!("../test2")), 6)
    }
}

