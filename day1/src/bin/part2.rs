use anyhow::Result;
use regex::Regex;
use std::fs;

#[derive(Clone)]
struct Number {
    name: String,
    value: String,
}

impl Number {
    fn new(name: String, value: String) -> Self {
        Number { name, value }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let parsed_input = parse_input(lines.clone());

    let digit_lines: Vec<String> = parsed_input
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

fn parse_input(input: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();

    let numbers: Vec<Number> = vec![
        Number::new("one".to_string(), "1".to_string()),
        Number::new("two".to_string(), "2".to_string()),
        Number::new("three".to_string(), "3".to_string()),
        Number::new("four".to_string(), "4".to_string()),
        Number::new("five".to_string(), "5".to_string()),
        Number::new("six".to_string(), "6".to_string()),
        Number::new("seven".to_string(), "7".to_string()),
        Number::new("eight".to_string(), "8".to_string()),
        Number::new("nine".to_string(), "9".to_string()),
    ];

    input
        .into_iter()
        .map(|line| {
            // println!("Line before: {}", line);
            let mut matches = Vec::new();
            let mut matches_string = String::new();
            for cap in re.captures_iter(&line) {
                matches.push(cap.get(0).unwrap().as_str().to_string());
            }
            if matches.is_empty() {
                panic!("This line is empty: {:#?}", line);
            }
            matches.iter_mut().for_each(|mat| {
                for number in numbers.clone() {
                    if mat == &number.name {
                        *mat = number.value;
                    }
                }
                matches_string += mat;
            });
            // println!("{:#?}", matches);
            // println!("Line after: {}", matches_string.clone());

            matches_string
        })
        .collect::<Vec<_>>()
}

fn add_lines(lines: Vec<String>) -> Result<i32> {
    let digits: Vec<i32> = lines
        .into_iter()
        .map(|line| {
            println!("Line before: {}", line);
            if line == "" {
                println!("This line is empty: {:#?}", line);
                return 0;
            }
            let nums = format!(
                "{}{}",
                line.chars().nth(0).unwrap(),
                line.chars().nth(line.len() - 1).unwrap()
            );
            println!("Line before: {}", nums.clone());
            nums.parse::<i32>().unwrap()
        })
        .collect();
    Ok(digits.iter().sum())
}
