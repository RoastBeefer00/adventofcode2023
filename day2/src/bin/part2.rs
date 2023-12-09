use anyhow::Result;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: i32,
    pulls: Vec<Pull>,
    min_red: i32,
    min_blue: i32,
    min_green: i32,
    power: i32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            id: 0,
            pulls: Vec::new(),
            min_red: 0,
            min_blue: 0,
            min_green: 0,
            power: 0,
        }
    }
}

impl Game {
    fn new(id: i32) -> Self {
        Game {
            id,
            pulls: Vec::new(),
            min_red: 0,
            min_blue: 0,
            min_green: 0,
            power: 0,
        }
    }

    fn is_valid(&self) -> bool {
        for pull in self.pulls.clone() {
            if pull.red > 12 || pull.blue > 14 || pull.green > 13 {
                return false;
            }
        }

        true
    }

    fn get_min_red(&mut self) {
        self.min_red = self
            .pulls
            .clone()
            .into_iter()
            .map(|pull| pull.red)
            .max()
            .unwrap()
    }

    fn get_min_blue(&mut self) {
        self.min_blue = self
            .pulls
            .clone()
            .into_iter()
            .map(|pull| pull.blue)
            .max()
            .unwrap()
    }

    fn get_min_green(&mut self) {
        self.min_green = self
            .pulls
            .clone()
            .into_iter()
            .map(|pull| pull.green)
            .max()
            .unwrap()
    }

    fn get_power(&mut self) {
        self.power = self.min_red * self.min_blue * self.min_green;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pull {
    red: i32,
    blue: i32,
    green: i32,
}

impl Default for Pull {
    fn default() -> Self {
        Pull {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

impl Pull {
    fn new(red: i32, blue: i32, green: i32) -> Self {
        Pull { red, blue, green }
    }

    fn update_red(&mut self, red: i32) {
        self.red = red;
    }

    fn update_blue(&mut self, blue: i32) {
        self.blue = blue;
    }

    fn update_green(&mut self, green: i32) {
        self.green = green;
    }
}

fn get_game_id(game_str: String) -> i32 {
    let split = game_str.split(" ");
    let id = split
        .last()
        .expect("should have found an id")
        .parse::<i32>()
        .unwrap();

    id
}

fn process_line(line: String) -> Game {
    let mut processed_games = Vec::new();
    let mut split_game = line.split(": ");
    let game_id = get_game_id(
        split_game
            .next()
            .expect("should have found a game")
            .clone()
            .to_string(),
    );
    let games_string = match split_game.last() {
        Some(games) => games,
        None => panic!("unable to find games"),
    };
    let pulls: Vec<&str> = games_string.split("; ").collect();
    for pull in pulls {
        let mut pull_object = Pull::default();
        let cubes: Vec<&str> = pull.split(", ").collect();
        cubes.iter().for_each(|cube| {
            let mut info = cube.split(" ");
            let number_string = info.next().expect("should have found a number");
            let number = number_string.parse::<i32>().unwrap();
            let color = info.last().expect("should have found a color");

            match color {
                "red" => pull_object.update_red(number),
                "blue" => pull_object.update_blue(number),
                "green" => pull_object.update_green(number),
                _ => println!("no colors found"),
            }
        });
        processed_games.push(pull_object);
    }

    let mut result = Game {
        id: game_id,
        pulls: processed_games,
        min_red: 0,
        min_blue: 0,
        min_green: 0,
        power: 0,
    };
    result.get_min_red();
    result.get_min_blue();
    result.get_min_green();
    result.get_power();

    result
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    let games: Vec<Game> = lines.into_iter().map(|line| process_line(line)).collect();

    let game_powers: Vec<i32> = games.into_iter().map(|game| game.power).collect();
    let result: i32 = game_powers.iter().sum();

    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_power() {
        let input = String::from(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        let mut game = process_line(input);
        game.get_min_red();
        game.get_min_blue();
        game.get_min_green();
        game.get_power();

        assert_eq!(game.min_red, 20);
        assert_eq!(game.min_blue, 6);
        assert_eq!(game.min_green, 13);
        assert_eq!(game.power, 1560);
    }

    #[test]
    fn test_entire_input() {
        let input = fs::read_to_string("test.txt").unwrap();

        let lines: Vec<String> = input.lines().map(String::from).collect();
        let games: Vec<Game> = lines.into_iter().map(|line| process_line(line)).collect();

        let game_powers: Vec<i32> = games.into_iter().map(|game| game.power).collect();
        let result: i32 = game_powers.iter().sum();
        let answer = 2286;
        assert_eq!(result, answer);
    }
}
