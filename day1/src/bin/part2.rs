use anyhow::Result;
use std::fs;

#[derive(Clone)]
struct Number {
    name: &'static str,
    value: &'static str,
}

impl Number {
    fn new(name: &'static str, value: &'static str) -> Self {
        Number { name, value }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let parsed_input = parse_input(lines.clone());

    println!("{:#?}", lines.clone().get(0));
    println!("{:#?}", parsed_input.clone().get(0));
    let digit_lines: Vec<String> = parsed_input
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .filter(|char| char.is_numeric())
                .collect()
        })
        .collect();

    // println!("{:#?}", digit_lines.clone().);
    let sum = add_lines(digit_lines)?;
    println!("{:#?}", sum);
    Ok(())
}

fn parse_input(input: Vec<String>) -> Vec<String> {
    let parsed_strings: Vec<String> = input;
    let letters = vec![
        "e",
        "f",
        "n",
        "o",
        "s",
        "t",
    ]
    let numbers: Vec<Number> = vec![
        Number::new("one", "1"),
        Number::new("two", "2"),
        Number::new("three", "3"),
        Number::new("four", "4"),
        Number::new("five", "5"),
        Number::new("six", "6"),
        Number::new("seven", "7"),
        Number::new("eight", "8"),
        Number::new("nine", "9"),
    ];

    parsed_strings
        .into_iter()
        .map(|line| {
            let mut parsed_string = line;

            for number in numbers.clone() {
                if parsed_string.contains(number.name) {
                    parsed_string = parsed_string.replace(number.name, number.value);
                }
            }

            parsed_string
        })
        .collect::<Vec<_>>()
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
