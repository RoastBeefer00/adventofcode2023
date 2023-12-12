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

fn parse_times(input: &str) -> Result<u64> {
    let new_input: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    Ok(new_input.parse::<u64>()?)
}

fn parse_distances(input: &str) -> Result<u64> {
    let new_input: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    Ok(new_input.parse::<u64>()?)
}

fn process_race(time: u64, distance: u64) -> Result<u64> {
    let range = 0..time;
    let distances: Vec<u64> = range
        .into_iter()
        .map(|seconds| seconds * (time - seconds))
        .collect();

    Ok(distances
        .into_iter()
        .filter(|&x| x > distance)
        .collect::<Vec<_>>()
        .len() as u64)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let time = parse_times(&lines.next().unwrap()).unwrap();
    let distance = parse_distances(&lines.last().unwrap()).unwrap();

    let wins = process_race(time, distance).unwrap();
    println!("{wins}");
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
        let time = parse_times(&lines.next().unwrap()).unwrap();
        let distance = parse_distances(&lines.last().unwrap()).unwrap();

        let wins = process_race(time, distance).unwrap();

        assert_eq!(71503, wins);
        Ok(())
    }
}
