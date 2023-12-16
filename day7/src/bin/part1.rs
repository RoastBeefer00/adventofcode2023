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

impl Hand {
    fn is_better_than(&self, hand: &Hand) -> bool {
        if self.hand_type.value() == hand.hand_type.value() {
            let mut i = 0;
            while i < self.cards.len() {
                let first = self.cards.get(i).unwrap();
                let second = hand.cards.get(i).unwrap();

                if first.strength == second.strength {
                    i += 1;
                } else if first.strength > second.strength {
                    return true;
                } else {
                    return false;
                }
            }
            false
        } else if self.hand_type.value() > hand.hand_type.value() {
            true
        } else {
            false
        }
    }
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

fn parse_line(input: &str) -> IResult<&str, (Hand, i64)> {
    let (input, (cards, bid)) = tuple((
        complete::alphanumeric1.map(|c| c),
        complete::i64.preceded_by(tag(" ")),
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

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let mut hands: Vec<(Hand, i64)> = lines
        .into_iter()
        .map(|line| {
            let (_, (hand, bid)) = parse_line(line).unwrap();
            (hand, bid)
        })
        .collect();
    hands.sort_by(|(a, _), (b, _)| a.is_better_than(b).cmp(&b.is_better_than(a)));

    let mut index = 0;
    let winnings = std::iter::from_fn(move || {
        index += 1;
        if index - 1 < hands.len() {
            let (_, bid) = hands.get(index - 1).unwrap();
            Some(index as i64 * bid)
        } else {
            None
        }
    })
    .collect::<Vec<_>>();

    let sum: i64 = winnings.into_iter().sum();
    println!("{sum}");
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

        let mut hands: Vec<(Hand, i64)> = lines
            .into_iter()
            .map(|line| {
                let (_, (hand, bid)) = parse_line(line).unwrap();
                (hand, bid)
            })
            .collect();
        hands.sort_by_key(|(hand, _)| hand.hand_type.value());

        let mut index = 0;
        while index < hands.len() - 1 {
            let (first_hand, _) = hands.get(index).unwrap();
            let (second_hand, _) = hands.get(index + 1).unwrap();
            if first_hand.hand_type == second_hand.hand_type {
                if first_hand.is_better_than(second_hand) {
                    hands.swap(index, index + 1);
                }
            }
            index += 1;
        }

        index = 0;
        let winnings = std::iter::from_fn(move || {
            index += 1;
            if index - 1 < hands.len() {
                let (_, bid) = hands.get(index - 1).unwrap();
                Some(index as i64 * bid)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

        let sum: i64 = winnings.into_iter().sum();

        assert_eq!(sum, 6440);
        Ok(())
    }
}
