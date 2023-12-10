use anyhow::Result;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs;
use std::ops::Range;
use tracing::info;
// #[derive(Debug)]
// struct SeedId(u64);

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));
        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}

fn seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| SeedMap { mappings }))
        .parse(input)
}

#[tracing::instrument]
fn parse_seedmaps(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, tag(" "), complete::u64)
                .map(|(start, offset)| start..(start + offset)),
        ))
        .parse(input)?;
    let (input, maps) = many1(seed_map)(input)?;

    Ok((input, (seeds, maps)))
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, (seeds, maps)) = parse_seedmaps(input).expect("a valid parse");

    let locations = seeds
        .into_par_iter()
        .flat_map(|range| range.clone())
        .map(|seed| maps.iter().fold(seed, |seed, map| map.translate(seed)))
        .collect::<Vec<u64>>();

    Ok(locations
        .iter()
        .min()
        .expect("should have a minimum location value")
        .to_string())
}

#[tracing::instrument]
fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = process(&input)?;

    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test_log::test]
    fn test_process() -> Result<()> {
        let input = fs::read_to_string("test.txt").unwrap();

        assert_eq!("46", process(&input)?);
        Ok(())
    }

    // #[test]
    // fn test_entire_input() {
    //     let input = fs::read_to_string("test.txt").unwrap();

    //     let lines: Vec<String> = input.lines().map(String::from).collect();
    //     let games: Vec<Card> = lines.into_iter().map(|line| process_line(line)).collect();

    //     let valid_game_ids: Vec<i32> = games
    //         .into_iter()
    //         .filter(|game| game.is_valid())
    //         .map(|game| game.id)
    //         .collect();
    //     let result: i32 = valid_game_ids.iter().sum();
    //     let answer = 8;
    //     assert_eq!(result, answer);
    // }
}
