use std::{fmt::Display, iter::Sum};

use itertools::Itertools;

use crate::*;

pub struct Day14;
impl Day for Day14 {
    const DAY_NUM: u8 = 14;
    type Input = String;
    type Output = usize;

    fn get_test_data() -> Self::Input {
        r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            .to_string()
    }

    fn get_test_result() -> Self::Output {
        136
    }

    fn run(data: Self::Input) -> Self::Output {
        part1::run(data)
    }
}
impl DayPart2 for Day14 {
    fn run_part2(data: Self::Input) -> Self::Output {
        part2::run(data)
    }

    fn get_test_result_part2() -> Self::Output {
        64
    }

    fn get_test_data_part2() -> Self::Input {
        Self::get_test_data()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Object {
    Stone,
    Empty,
    CubeStone,
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Stone => "O",
                Object::Empty => ".",
                Object::CubeStone => "#",
            }
        )
    }
}
impl Sum<Object> for usize {
    fn sum<I: Iterator<Item = Object>>(iter: I) -> Self {
        iter.map(|o| match o {
            Object::Stone => 1,
            Object::Empty => 0,
            Object::CubeStone => 0,
        })
        .sum()
    }
}
impl<'a> Sum<&'a Object> for usize {
    fn sum<I: Iterator<Item = &'a Object>>(iter: I) -> Self {
        iter.map(|o| *o).sum()
    }
}
fn display_grid<T: ToString>(data: &Vec<Vec<T>>) {
    println!("----------------------------");
    for row in data.iter() {
        let line = row.iter().map(ToString::to_string).join("");
        println!("{}", line);
    }
    println!("----------------------------");
}
mod part1 {
    use super::*;

    pub fn run(data: String) -> usize {
        let mut data = parse_lines(data);
        display_grid(&data);
        shift_stones(&mut data, Direction::North);
        display_grid(&data);
        evaluate_wheights(&data)
    }
}
mod part2 {
    use indicatif::{ProgressIterator as _, ProgressStyle};

    use super::*;
    pub fn run(data: String) -> usize {
        let mut data = parse_lines(data);
        display_grid(&data);
        let iterator = 0..1_000_000_000;
        // let iterator = 0..1000; //for testing
        for _ in iterator.progress().with_style(ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:.cyan/blue} eta:[{eta_precise}] at {per_sec:15} {human_pos:>13}/{human_len:13} {percent_precise}%").unwrap()) {
            shift_stones(&mut data, Direction::North);
            shift_stones(&mut data, Direction::West);
            shift_stones(&mut data, Direction::South);
            shift_stones(&mut data, Direction::East);
            // display_grid(&data);
        }
        evaluate_wheights(&data)
    }
}
fn parse_lines(data: String) -> Vec<Vec<Object>> {
    data.lines()
        // .rev()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => Object::Stone,
                    '#' => Object::CubeStone,
                    '.' => Object::Empty,
                    _ => panic!("Invalid character: {}", c),
                })
                .collect()
        })
        .collect()
}
fn evaluate_wheights(data: &Vec<Vec<Object>>) -> usize {
    data.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) * x.iter().sum::<usize>())
        .sum()
}
fn shift_stones(data: &mut Vec<Vec<Object>>, direction: Direction) {
    let (horizontal_active, vertical_active) = match direction {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
        Direction::East => (1, 0),
    };
    let width = data[0].len();
    let mut count_stones: isize;
    let height = data.len();
    let range = match direction {
        Direction::South => num::range_step(0, height as isize, 1),
        Direction::North => num::range_step(height as isize - 1, 0 - 1, -1),
        Direction::West => num::range_step(width as isize - 1, 0 - 1, -1),
        Direction::East => num::range_step(0, width as isize, 1),
    };
    // println!("Shifting stones to {direction} v:{vertical_active} h:{horizontal_active}");

    if vertical_active != 0 {
        for x in 0..width {
            count_stones = 0;
            // println!("entering column: {x}");
            for y in range.clone() {
                // println!("entering row: {y}");
                // display_grid(data);
                move_object(
                    data,
                    y as usize,
                    x as usize,
                    &mut count_stones,
                    vertical_active,
                    horizontal_active,
                );
            }
        }
    } else {
        for y in 0..height {
            // println!("entering row: {y}");
            count_stones = 0;
            for x in range.clone() {
                // println!("entering column: {x}");
                // display_grid(data);
                move_object(
                    data,
                    y as usize,
                    x as usize,
                    &mut count_stones,
                    vertical_active,
                    horizontal_active,
                );
            }
        }
    }
}

fn move_object(
    data: &mut Vec<Vec<Object>>,
    y: usize,
    x: usize,
    count_stones: &mut isize,
    vertical_active: isize,
    horizontal_active: isize,
) {
    let obj = data[y][x];
    match obj {
        Object::Stone => {
            *count_stones += 1;
        }
        Object::CubeStone => {
            *count_stones = 0;
        }
        Object::Empty => {
            let c = *count_stones;
            if c > 0 {
                let vertical_factor = c * vertical_active;
                let origin_y = y as isize - vertical_factor;
                let horizontal_factor = c * horizontal_active;
                let origin_x = x as isize - horizontal_factor;
                // println!("moving stone from {origin_x}x{origin_y}y to {x}x{y}y with factors: h:{horizontal_factor} v:{vertical_factor}");
                data[origin_y as usize][origin_x as usize] = Object::Empty;
                data[y][x] = Object::Stone;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "North",
                Direction::South => "South",
                Direction::West => "West",
                Direction::East => "East",
            }
        )
    }
}
