use anyhow::Result;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, multispace0, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use std::fs;
use std::iter::zip;
use std::ops::Range;

fn parse_times(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, values) = tag("Time: ")
        .precedes(separated_list1(space1, complete::u32.map(|num| num)).preceded_by(multispace0))
        .parse(input)?;

    Ok((input, values))
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, values) = tag("Distance: ")
        .precedes(separated_list1(space1, complete::u32.map(|num| num)).preceded_by(multispace0))
        .parse(input)?;

    Ok((input, values))
}

fn process_race(time: u32, distance: u32) -> Result<u32> {
    let range = 0..time;
    let distances: Vec<u32> = range
        .into_iter()
        .map(|seconds| seconds * (time - seconds))
        .collect();

    Ok(distances
        .into_iter()
        .filter(|&x| x > distance)
        .collect::<Vec<_>>()
        .len() as u32)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let (_, races) = parse_times(&lines.next().unwrap()).unwrap();
    let (_, distances) = parse_distances(&lines.last().unwrap()).unwrap();

    let races: Vec<(u32, u32)> = zip(races, distances).collect();
    let wins: Vec<u32> = races
        .into_iter()
        .map(|(time, distance)| process_race(time, distance).unwrap())
        .collect();

    let result = wins.into_iter().fold(1, |acc, win| acc * win);
    println!("{result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test_log::test]
    fn test_process() -> Result<()> {
        let input = fs::read_to_string("test.txt").unwrap();

        let mut lines = input.lines();
        let (_, races) = parse_times(&lines.next().unwrap()).unwrap();
        let (_, distances) = parse_distances(&lines.last().unwrap()).unwrap();

        let races: Vec<(u32, u32)> = zip(races, distances).collect();
        let wins: Vec<u32> = races
            .into_iter()
            .map(|(time, distance)| process_race(time, distance).unwrap())
            .collect();

        let result = wins.into_iter().fold(1, |acc, win| acc * win);
        assert_eq!(288, result);
        Ok(())
    }
}
