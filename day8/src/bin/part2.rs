use anyhow::Result;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, char, line_ending, multispace0, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use std::iter::zip;
use std::ops::Range;
use std::{fs, iter};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Direction::Left,
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn go_left(&self, nodes: Vec<Node>) -> Node {
        let node: Node = nodes
            .into_iter()
            .filter(|node| node.id == self.left)
            .next()
            .unwrap();

        node
    }

    fn go_right(&self, nodes: Vec<Node>) -> Node {
        let node: Node = nodes
            .into_iter()
            .filter(|node| node.id == self.right)
            .next()
            .unwrap();

        node
    }
}

#[derive(Clone, Debug)]
struct Map {
    maps: Vec<Node>,
}

fn line(input: &str) -> IResult<&str, Node> {
    let (input, (id, (left, right))) = tuple((
        complete::alpha1.map(|c| c),
        delimited(
            char('('),
            tuple((
                complete::alpha1.map(|c| c),
                complete::alpha1.map(|c| c).preceded_by(tag(", ")),
            )),
            char(')'),
        )
        .preceded_by(tag(" = ")),
    ))(input)?;

    Ok((
        input,
        Node {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, (Vec<Direction>, Map)> {
    let (input, directions) = complete::alpha1.map(|c| c).parse(input)?;
    dbg!(&directions);
    let (input, nodes) = tag("\n")
        .precedes(many1(line_ending.precedes(line)).map(|maps| Map { maps }))
        .parse(input)?;
    let dirs: Vec<Direction> = directions.chars().map(|c| Direction::new(c)).collect();
    Ok((input, (dirs, nodes)))
}

fn process(input: String) -> usize {
    let (_, (directions, nodes)) = parse_map(&input).unwrap();
    dbg!((&directions, &nodes));

    let mut the_zs: Vec<Node> = nodes
        .maps
        .clone()
        .into_iter()
        .filter(|node| node.id.ends_with("A"))
        .collect();

    dbg!(the_zs.clone());
    let mut count = 0;
    let mut index = 0;
    while the_zs.iter().any(|node| !node.id.ends_with("Z")) {
        if index == directions.len() {
            index = 0;
        }

        let direction = directions.get(index).unwrap();
        match direction {
            Direction::Left => {
                index += 1;
                count += 1;
                the_zs = the_zs
                    .into_iter()
                    .map(|node| node.go_left(nodes.maps.clone()))
                    .collect();
            }
            Direction::Right => {
                index += 1;
                count += 1;
                the_zs = the_zs
                    .into_iter()
                    .map(|node| node.go_right(nodes.maps.clone()))
                    .collect();
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").unwrap();
    let count = process(input);

    println!("{count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test_log::test]
    fn test_one() -> Result<()> {
        let input = fs::read_to_string("test3.txt").unwrap();
        let count = process(input);

        assert_eq!(count, 6);
        Ok(())
    }
}
