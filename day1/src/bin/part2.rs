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

    println!("{:#?}", parsed_input);
    // let digit_lines: Vec<String> = parsed_input
    //     .into_iter()
    //     .map(|line| {
    //         line.chars()
    //             .into_iter()
    //             .filter(|char| char.is_numeric())
    //             .collect()
    //     })
    //     .collect();

    let sum = add_lines(parsed_input)?;
    println!("{:#?}", sum);
    Ok(())
}

fn parse_input(input: Vec<String>) -> Vec<String> {
    input
        .into_iter()
        .map(|line| {
            // println!("Line before: {}", line);
            let mut index = 0;
            let line_iter = std::iter::from_fn(move || {
                let slice = &line[index..];
                let result = if slice.starts_with("one") {
                    Some('1')
                } else if slice.starts_with("two") {
                    Some('2')
                } else if slice.starts_with("three") {
                    Some('3')
                } else if slice.starts_with("four") {
                    Some('4')
                } else if slice.starts_with("five") {
                    Some('5')
                } else if slice.starts_with("six") {
                    Some('6')
                } else if slice.starts_with("seven") {
                    Some('7')
                } else if slice.starts_with("eight") {
                    Some('8')
                } else if slice.starts_with("nine") {
                    Some('9')
                } else {
                    slice.chars().next()
                };
                index += 1;

                result
            });

            let mut it = line_iter.filter_map(|character| character.to_digit(10));
            let first = it.next().expect("should be a number");
            match it.last() {
                Some(value) => format!("{first}{value}"),
                None => format!("{first}{first}"),
            }
        })
        .collect::<Vec<_>>()
}

fn add_lines(lines: Vec<String>) -> Result<i32> {
    let digits: Vec<i32> = lines
        .into_iter()
        .map(|line| {
            // println!("Line before: {}", line);
            // if line == "" {
            //     println!("This line is empty: {:#?}", line);
            //     return 0;
            // }
            // let nums = format!(
            //     "{}{}",
            //     line.chars().nth(0).unwrap(),
            //     line.chars().nth(line.len() - 1).unwrap()
            // );
            // println!("Line before: {}", nums.clone());
            line.parse::<i32>().unwrap()
        })
        .collect();
    Ok(digits.iter().sum())
}
