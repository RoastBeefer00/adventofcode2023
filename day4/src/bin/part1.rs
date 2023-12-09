use anyhow::Result;
use std::fs;

struct Card {
    winning_numbers: Vec<i32>,
    score: i32,
}

fn update_score(score: i32) -> i32 {
    if score == 0 {
        1
    } else {
        score * 2
    }
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
    let card_split = line.split(": ");
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
    dbg!((winning_numbers_string, my_numbers_string));
    let (winning_numbers_split, my_numbers_split): (Vec<&str>, Vec<&str>) = (
        winning_numbers_string.split(" ").collect(),
        my_numbers_string.split(" ").collect(),
    );
    dbg!((winning_numbers_split.clone(), my_numbers_split.clone()));

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
        winning_numbers,
        score,
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    let cards: Vec<Card> = lines.into_iter().map(|line| process_line(line)).collect();
    let scores: Vec<i32> = cards.into_iter().map(|card| card.score).collect();

    let sum: i32 = scores.into_iter().sum();

    println!("Sum: {}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example_one() {
        let input = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let card = process_line(input);

        let result = card.score;
        let answer = 8;
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
