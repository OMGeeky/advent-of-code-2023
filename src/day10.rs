use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::*;

pub struct Day10;
impl Day for Day10 {
    const DAY_NUM: u8 = 10;
    type Input = String;
    type Output = usize;

    fn get_test_data() -> Self::Input {
        todo!()
    }
    fn get_test_result() -> Self::Output {
        todo!()
    }

    fn get_multiple_test_result() -> Box<impl Iterator<Item = Self::Output>> {
        Box::new([4, 8].into_iter())
    }

    fn get_multiple_test_data() -> Box<impl Iterator<Item = Self::Input>> {
        Box::new(
            [
                ".....\n.S-7.\n.|.|.\n.L-J.\n.....".to_string(),
                "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...".to_string(),
            ]
            .into_iter(),
        )
    }

    fn run(data: Self::Input) -> Self::Output {
        assert!(Tile::from_char_part1('L')
            .connects_to_in_direction(Tile::from_char_part1('J'), Directions::East));
        assert!(!Tile::from_char_part1('L')
            .connects_to_in_direction(Tile::from_char_part1('L'), Directions::East));
        assert!(Tile::from_char_part1('L')
            .connects_to_in_direction(Tile::from_char_part1('-'), Directions::East));
        assert!(Tile::from_char_part1('S')
            .connects_to_in_direction(Tile::from_char_part1('-'), Directions::East));
        println!("Nice it worked");
        println!("Input: ");
        println!("{}", data);
        let mut index_data = data.clone();
        index_data.retain(|x|x!='\n');
        let startIndex = index_data.find('S').expect("Needs to have a start");
        let data: Vec<Vec<Tile>> = data
            .lines()
            .map(|l| l.chars().map(Tile::from_char_part1).collect())
            .collect();
        println!("");
        print_matrix(&data);
        let mut data: Vec<Vec<(Tile, usize)>> = data
            .into_iter()
            .map(|l| l.into_iter().map(|t| (t, usize::MAX)).collect())
            .collect();

        let width = data.first().unwrap().len();
        let height = data.len();
        let start_pos = (startIndex / width, startIndex % width);
        println!("Start at: {start_pos:?}");
        let start = get_at_position_mut(&mut data, start_pos);
        start.1 = 0;
        let steps = 1;
        process_neighbours(start.0, start_pos, (width, height), &mut data, steps);
        let visual_data = data
            .iter()
            .map(|l| {
                l.iter()
                    .map(|(_, i)| i)
                    .map(|i| {
                        let c;
                        if i < &usize::MAX {
                            c = i.to_string();
                        } else {
                            c = " ".to_string();
                        }
                        format!("{:1}", c)
                    })
                    .collect_vec()
            })
            .collect_vec();
        print_matrix(&visual_data);
        let result = data
            .iter()
            .map(|x| {
                x.iter()
                    .map(|(_, i)| if i < &usize::MAX { *i } else { 0 })
                    .max()
            })
            .max()
            .flatten()
            .unwrap_or(0);
        println!("Result is {result}");
        result
    }
}

fn process_neighbours(
    own_tile: Tile,
    own_pos: Position,
    size: Position,
    data: &mut Vec<Vec<(Tile, usize)>>,
    steps: usize,
) {
    println!("Processing neighbours for {own_pos:?}");
    for dir in own_tile.get_directions() {
        // println!("  Checking direction {dir:?} from {own_pos:?}");
        let pos = add_direction(own_pos, dir);
        if pos.is_none(){
            // println!("{pos:?} is out of bounds");
            continue;
        }
        let pos = pos.unwrap();
        if pos.0 >= size.0 || pos.1 >= size.1 {
            // println!("{pos:?} is out of bounds");
            continue;
        }
        let tile = get_at_position_mut(data, pos);
        if !own_tile.connects_to_in_direction(tile.0, dir) {
            // println!("{pos:?} does not connect to own {own_pos:?}");
            continue;
        }
        if tile.1 <= steps {
            // println!("{pos:?} already has a shorter path here");
            continue;
        }
        tile.1 = steps;
        process_neighbours(tile.0, pos, size, data, steps + 1);
    }
}

type Position = (usize, usize);

fn get_at_position_mut<T>(data: &mut Vec<Vec<T>>, pos: Position) -> &mut T {
    data.get_mut(pos.1).unwrap().get_mut(pos.0).unwrap()
}
fn add_direction(pos: Position, direction: Directions) -> Option<Position> {
    Some(match direction {
        Directions::North  if pos.1 > 0=> (pos.0, pos.1 - 1),
        Directions::South => (pos.0, pos.1 + 1),
        Directions::East => (pos.0 + 1, pos.1),
        Directions::West if pos.0 > 0=> (pos.0 - 1, pos.1),
        _=>{
            return None;
        }
    })
}
fn print_matrix<T>(matrix: &Vec<Vec<T>>)
where
    T: Display,
{
    for line in matrix {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
}
// #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// struct Position{
//     x: usize,
//     y: usize,
// }
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Directions {
    North = 0b0001,
    South = 0b0010,
    East = 0b0100,
    West = 0b1000,
}
impl Directions {
    fn invert(self) -> Directions {
        match self {
            Directions::North => Self::South,
            Directions::South => Self::North,
            Directions::East => Self::West,
            Directions::West => Self::East,
        }
    }
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tile {
    NorthSouth = Directions::North as isize | Directions::South as isize,
    EastWest = Directions::West as isize | Directions::East as isize,
    NorthWest = Directions::North as isize | Directions::West as isize,
    NorthEast = Directions::North as isize | Directions::East as isize,
    SouthWest = Directions::South as isize | Directions::West as isize,
    SouthEast = Directions::South as isize | Directions::East as isize,
    No = 0b0000,
    Start = 0b11111,
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char_part1())
    }
}

impl Tile {
    fn get_directions(self) -> Vec<Directions> {
        match self {
            Tile::NorthSouth => vec![Directions::North, Directions::South],
            Tile::EastWest => vec![Directions::East, Directions::West],
            Tile::NorthWest => vec![Directions::North, Directions::West],
            Tile::NorthEast => vec![Directions::North, Directions::East],
            Tile::SouthWest => vec![Directions::South, Directions::West],
            Tile::SouthEast => vec![Directions::South, Directions::East],
            Tile::No => Vec::new(),
            Tile::Start => vec![
                Directions::North,
                Directions::South,
                Directions::East,
                Directions::West,
            ],
        }
    }
    fn to_char_part1<'a>(self) -> char {
        match self {
            Tile::NorthSouth => '|',
            Tile::EastWest => '-',
            Tile::NorthWest => 'J',
            Tile::NorthEast => 'L',
            Tile::SouthWest => '7',
            Tile::SouthEast => 'F',
            Tile::No => '.',
            Tile::Start => 'S',
        }
    }
    fn from_char_part1<'a>(c: impl Into<char>) -> Self {
        match c.into() {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'J' => Tile::NorthWest,
            'L' => Tile::NorthEast,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::No,
            'S' => Tile::Start,
            _ => Tile::No,
        }
    }
    fn connects_to_in_direction(self, other: Self, direction: Directions) -> bool {
        self.has_flag(direction) && other.has_flag(direction.invert())
    }
    fn has_flag(self, direction: Directions) -> bool {
        let only_bits_in_direction = self as u8 & direction as u8;
        // println!(            "{:0>5b} & {:0>5b} => {:0>5b}",            self as u8, direction as u8, only_bits_in_direction        );
        only_bits_in_direction == direction as u8
    }
}
