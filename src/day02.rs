use std::{collections::HashMap, fmt::Display};

use crate::*;

pub struct Day02;
impl Day for Day02 {
    const DAY_NUM: u8 = 02;
    type Input = String;
    type Output = usize;

    fn get_test_data() -> Vec<Self::Input> {
        vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ]
    }

    fn get_test_result() -> Self::Output {
        8
    }

    fn run(data: Vec<Self::Input>) -> Self::Output {
        let mut result = 0;
        for line in data {
            let game = Game::from(line.clone());
            println!("{}", &game);
            if game.is_possible() {
                result += game.id;
            }
        }
        result
    }
}
impl DayPart2 for Day02 {
    fn run_part2(data: Vec<Self::Input>) -> Self::Output {
        let mut result = 0;
        for line in data {
            let game = Game::from(line.clone());
            println!("{}", &game);
            result += game.get_game_power();
        }
        result
    }

    fn get_test_result_part2() -> Self::Output {
        2286
    }

    fn get_test_data_part2() -> Vec<Self::Input> {
        Self::get_test_data()
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        for reveals in self.reveals.iter() {
            for (color, count) in reveals.iter() {
                let max: &usize = &match &color {
                    Color::Red => 12,
                    Color::Green => 13,
                    Color::Blue => 14,
                };
                if count > max {
                    return false;
                }
            }
        }
        true
    }
    fn get_game_power(&self) -> usize {
        let mut min_needed: HashMap<Color, usize> = HashMap::new();
        for reveal in &self.reveals {
            for (color, count) in reveal {
                let x = min_needed.get_mut(color);
                match x {
                    Some(previous) => {
                        if *previous < *count {
                            *previous = *count;
                        }
                    }
                    None => {
                        min_needed.insert(*color, *count);
                    }
                }
            }
        }
        println!("Min power needed: {:?}", min_needed);
        min_needed.into_values().product()
    }
}

//region data structure
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Red => "red",
                Color::Green => "green",
                Color::Blue => "blue",
            }
        )
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    reveals: Vec<HashMap<Color, usize>>,
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game {}: {}",
            self.id,
            self.reveals
                .iter()
                .map(|x| x
                    .iter()
                    .map(|r| Reveal {
                        color: *r.0,
                        count: *r.1
                    }
                    .to_string())
                    .collect::<Vec<String>>()
                    .join(", "))
                .collect::<Vec<String>>()
                .join("; ")
        )
    }
}
impl From<String> for Game {
    fn from(value: String) -> Self {
        let (id, reveals) = value
            .strip_prefix("Game ")
            .unwrap()
            .split_once(": ")
            .unwrap();
        let id: usize = id.parse().unwrap();
        let reveals = reveals
            .split(';')
            .map(|x| {
                let colors: Vec<Reveal> = x.split(',').map(|r| r.to_string().into()).collect();
                let mut hash: HashMap<Color, usize> = HashMap::new();
                for reveal in colors.into_iter() {
                    hash.insert(reveal.color, reveal.count);
                }
                hash
            })
            .collect();
        Self { id, reveals }
    }
}

#[derive(Debug)]
struct Reveal {
    count: usize,
    color: Color,
}
impl Display for Reveal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.count, self.color)
    }
}
impl From<String> for Reveal {
    fn from(value: String) -> Self {
        let (count, color) = value.trim().split_once(' ').unwrap();
        let count = count.parse().unwrap();

        let color = match color {
            "green" => Color::Green,
            "red" => Color::Red,
            "blue" => Color::Blue,

            _ => panic!("this case should never happen with valid input data"),
        };

        Self { count, color }
    }
}
//endregion
