use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let lines: Vec<String> = input.lines().map(String::from).collect();
    let digit_lines: Vec<String> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .filter(|char| char.is_numeric())
                .collect()
        })
        .collect();

    let sum = add_lines(digit_lines)?;
    println!("{:#?}", sum);

    Ok(())
}

fn add_lines(lines: Vec<String>) -> Result<i32> {
    let digits: Vec<i32> = lines
        .into_iter()
        .map(|line| {
            let nums = format!(
                "{}{}",
                line.chars().nth(0).unwrap(),
                line.chars().nth(line.len() - 1).unwrap()
            );
            nums.parse::<i32>().unwrap()
        })
        .collect();
    Ok(digits.iter().sum())
}
