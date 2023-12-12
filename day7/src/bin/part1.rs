use anyhow::Result;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, multispace0, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use std::iter::zip;
use std::ops::Range;
use std::{fs, iter};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Card {
    strength: u8,
}

impl Card {
    fn new(card: char) -> Card {
        let strength = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => 0,
        };

        Card { strength }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl HandType {
    fn value(&self) -> usize {
        match *self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::Pair => 2,
            HandType::HighCard => 1,
        }
    }
}

fn calculate_type(mut cards: Vec<Card>) -> HandType {
    cards.sort_by_key(|card| card.strength);

    let mut unique_cards = cards.clone();
    unique_cards.dedup();

    let mut index = 0;
    let mut card_matches = std::iter::from_fn(move || {
        index += 1;

        if index - 1 < unique_cards.len() {
            Some(
                cards
                    .iter()
                    .filter(|&card| card == unique_cards.get(index - 1).unwrap())
                    .count(),
            )
        } else {
            None
        }
    })
    .collect::<Vec<_>>();
    card_matches.sort();

    if card_matches == [5] {
        HandType::FiveOfAKind
    } else if card_matches == [1, 4] {
        HandType::FourOfAKind
    } else if card_matches == [2, 3] {
        HandType::FullHouse
    } else if card_matches == [1, 1, 3] {
        HandType::ThreeOfAKind
    } else if card_matches == [1, 2, 2] {
        HandType::TwoPair
    } else if card_matches == [1, 1, 1, 2] {
        HandType::Pair
    } else {
        HandType::HighCard
    }
}

fn parse_line(input: &str) -> IResult<&str, (Hand, u64)> {
    let (input, (cards, bid)) = tuple((
        complete::alphanumeric1.map(|c| c),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;
    let hand = cards.chars().map(|c| Card::new(c)).collect::<Vec<_>>();

    Ok((
        input,
        (
            Hand {
                cards: hand.clone(),
                hand_type: calculate_type(hand),
            },
            bid,
        ),
    ))
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test_log::test]
    fn test_process() -> Result<()> {
        let input = fs::read_to_string("test.txt").unwrap();
        let lines = input.lines();

        let mut hands: Vec<(Hand, u64)> = lines
            .into_iter()
            .map(|line| {
                let (_, (hand, bid)) = parse_line(line).unwrap();
                (hand, bid)
            })
            .collect();
        hands.sort_by_key(|(hand, _)| hand.hand_type.value());
        dbg!(hands);
        Ok(())
    }
}
