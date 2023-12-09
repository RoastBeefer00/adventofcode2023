use anyhow::Result;
use std::fs;

#[derive(Clone, Debug)]
struct Card {
    id: i32,
    score: i32,
    winning_numbers: Vec<i32>,
}

fn update_score(score: i32) -> i32 {
    score + 1
}

fn calculate_score(winning_numbers: Vec<i32>, my_numbers: Vec<i32>) -> i32 {
    let mut score = 0;
    for number in my_numbers {
        if winning_numbers.contains(&number) {
            score = update_score(score.clone());
        }
    }

    score
}

fn process_line(line: String) -> Card {
    let mut card_split = line.split(": ");
    let card_name = card_split.next().expect("should have found a name");
    let card_name_split = card_name.split(" ");
    let card_id = card_name_split
        .last()
        .expect("should have found a number")
        .parse::<i32>()
        .unwrap();
    let card_numbers = card_split.last().expect("should have found numbers");
    let mut numbers_split = card_numbers.split(" | ");
    let (winning_numbers_string, my_numbers_string) = (
        numbers_split
            .next()
            .expect("should have found first numbers"),
        numbers_split
            .last()
            .expect("should have found last numbers"),
    );
    let (winning_numbers_split, my_numbers_split): (Vec<&str>, Vec<&str>) = (
        winning_numbers_string.split(" ").collect(),
        my_numbers_string.split(" ").collect(),
    );

    let (winning_numbers, my_numbers): (Vec<i32>, Vec<i32>) = (
        winning_numbers_split
            .into_iter()
            .filter(|&num| num != "")
            .map(|num| num.parse::<i32>().unwrap())
            .collect(),
        my_numbers_split
            .into_iter()
            .filter(|&num| num != "")
            .map(|num| num.parse::<i32>().unwrap())
            .collect(),
    );

    let score = calculate_score(winning_numbers.clone(), my_numbers);

    Card {
        id: card_id,
        score,
        winning_numbers,
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut cards: Vec<Card> = lines.into_iter().map(|line| process_line(line)).collect();

    let adds: Vec<i32> = cards
        .clone()
        .into_iter()
        .map(|card| get_cards_added(card, cards.clone()))
        .collect();

    let mut result: i32 = adds.clone().into_iter().sum();
    result += cards.len() as i32;

    println!("Sum: {}", result);
    Ok(())
}

fn get_cards_added(card: Card, all_cards: Vec<Card>) -> i32 {
    println!("{}", card.clone().id);
    let mut result = card.score.clone();
    let mut score = card.score.clone();
    let id = card.id;

    while score > 0 {
        let won_card = all_cards
            .clone()
            .into_iter()
            .find(|card| card.id == score + id);
        result += match won_card {
            Some(card) => get_cards_added(card, all_cards.clone()),
            None => 0,
        };
        score -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example_one_day_two() {
        let input = fs::read_to_string("test.txt").unwrap();

        let lines: Vec<String> = input.lines().map(String::from).collect();
        let cards: Vec<Card> = lines.into_iter().map(|line| process_line(line)).collect();

        let adds: Vec<i32> = cards
            .clone()
            .into_iter()
            .map(|card| get_cards_added(card, cards.clone()))
            .collect();

        let mut result: i32 = adds.clone().into_iter().sum();
        result += cards.len() as i32;

        println!("{:#?}", adds);
        let answer = 30;
        assert_eq!(result, answer);
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
