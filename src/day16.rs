use std::{collections::VecDeque, fmt::Display};

use itertools::Itertools;

use crate::*;

pub struct Day16;
impl Day for Day16 {
    const DAY_NUM: u8 = 16;
    type Input = String;
    type Output = usize;

    fn get_test_data() -> Self::Input {
        vec![
            ".|...\\....",
            "|.-.\\.....",
            ".....|-...",
            "........|.",
            "..........",
            ".........\\",
            "..../.\\\\..",
            ".-.-/..|..",
            ".|....-|.\\",
            "..//.|....",
        ]
        .join("\n")
    }

    fn get_test_result() -> Self::Output {
        46
    }

    fn run(data: Self::Input) -> Self::Output {
        let lines = data
            .lines()
            .map(str::chars)
            .map(Itertools::collect_vec)
            .collect_vec();
        let length = lines.first().expect("Need at least one line").len();
        let height = lines.len();

        let starting_ray = Ray {
            pos: (0, 0),
            direction: Direction::Right,
        };
        check_heat(length, height, starting_ray, lines)
    }
}
impl DayPart2 for Day16 {
    fn run_part2(data: Self::Input) -> Self::Output {
        let lines = data
            .lines()
            .map(str::chars)
            .map(Itertools::collect_vec)
            .collect_vec();
        let length = lines.first().expect("Need at least one line").len();
        let height = lines.len();

        let mut max = 0;
        for y in 0..height {
            let starting_ray = Ray {
                pos: (0, y as isize),
                direction: Direction::Right,
            };
            let x = check_heat(length, height, starting_ray, lines.clone());
            if max < x {
                max = x;
            }
            let starting_ray = Ray {
                pos: ((length - 1) as isize, y as isize),
                direction: Direction::Left,
            };
            let x = check_heat(length, height, starting_ray, lines.clone());
            if max < x {
                max = x;
            }
        }
        for x in 0..length {
            let starting_ray = Ray {
                pos: (x as isize, 0),
                direction: Direction::Down,
            };
            let x = check_heat(length, height, starting_ray, lines.clone());
            if max < x {
                max = x;
            }
            let starting_ray = Ray {
                pos: (x as isize, (height - 1) as isize),
                direction: Direction::Up,
            };
            let x = check_heat(length, height, starting_ray, lines.clone());
            if max < x {
                max = x;
            }
        }
        max
    }

    fn get_test_result_part2() -> Self::Output {
        51
    }

    fn get_test_data_part2() -> Self::Input {
        Self::get_test_data()
    }
}

fn check_heat(length: usize, height: usize, starting_ray: Ray, lines: Vec<Vec<char>>) -> usize {
    let mut grid = Grid::new(length, height);
    let mut rays: VecDeque<Ray> = VecDeque::new();
    rays.push_back(starting_ray);

    let mut last_was_energized = false;
    while rays.len() > 0 {
        // println!("");
        // dbg!(&rays);
        let ray = &mut rays[0];
        if !grid.is_in_bounds(ray.pos) {
            // println!("Ray is out of bounds: {ray:?}");
            rays.pop_front();
            continue;
        }
        let c = lines[ray.pos.1 as usize][ray.pos.0 as usize];
        grid.set_highlight(ray.pos);
        // println!("{ray:?}");
        // println!(
        //     "Char: '{c}' line: '{}'",
        //     lines[ray.pos.1 as usize]
        //         .iter()
        //         .map(|x| x.to_string())
        //         .join("")
        // );
        // println!("{grid}");
        if grid.has_been_enerigzed_from_this_dir(ray.pos, ray.direction) {
            if last_was_energized {
                last_was_energized = false;
                // println!("Already been here");
                rays.pop_front();
                continue;
            } else {
                last_was_energized = true;
            }
        }
        grid.set_energized(ray.pos, ray.direction);
        match c {
            '.' => (),
            '\\' => match ray.direction {
                Direction::Up => ray.direction = Direction::Left,
                Direction::Down => ray.direction = Direction::Right,
                Direction::Left => ray.direction = Direction::Up,
                Direction::Right => ray.direction = Direction::Down,
            },
            '/' => match ray.direction {
                Direction::Up => ray.direction = Direction::Right,
                Direction::Down => ray.direction = Direction::Left,
                Direction::Left => ray.direction = Direction::Down,
                Direction::Right => ray.direction = Direction::Up,
            },
            '-' => match ray.direction {
                Direction::Up | Direction::Down => {
                    ray.direction = Direction::Left;
                    let mut second_ray = Ray {
                        direction: Direction::Right,
                        pos: ray.pos,
                    };
                    tick_ray(ray);
                    tick_ray(&mut second_ray);
                    rays.push_back(second_ray);
                    continue;
                }
                Direction::Left | Direction::Right => {}
            },
            '|' => match ray.direction {
                Direction::Left | Direction::Right => {
                    ray.direction = Direction::Up;
                    let mut second_ray = Ray {
                        direction: Direction::Down,
                        pos: ray.pos,
                    };
                    tick_ray(ray);
                    tick_ray(&mut second_ray);

                    rays.push_back(second_ray);
                    continue;
                }
                Direction::Up | Direction::Down => {}
            },
            other => println!("Invalid char: '{}'", other),
        }
        tick_ray(ray);
    }
    grid.reset_highlight();
    // println!("DONE");
    // println!("{grid}");

    grid.get_heat()
}

fn tick_ray(ray: &mut Ray) {
    match ray.direction {
        Direction::Up => ray.pos.1 -= 1,
        Direction::Down => ray.pos.1 += 1,
        Direction::Left => ray.pos.0 -= 1,
        Direction::Right => ray.pos.0 += 1,
    }
}

#[derive(Debug)]
struct Ray {
    pos: (isize, isize),
    direction: Direction,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Grid {
    highlighted_pos: Option<(isize, isize)>,
    size: (usize, usize),
    positions: Vec<(bool, bool, bool, bool)>,
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid<x:{},y:{}>:", self.size.0, self.size.1)?;
        write!(f, "- ")?;
        for x in 0..self.size.0 {
            write!(f, " {:4} ", x as u8)?;
        }
        write!(f, "")?;

        for y in 0..self.size.1 {
            writeln!(f, "")?;
            write!(f, "{} ", y)?;
            for x in 0..self.size.0 {
                // let c;
                let (l, u, r, d) = self.get_pos_energized_state((x as isize, y as isize));
                write!(f, " ")?;

                if l {
                    write!(f, "l")?;
                } else {
                    write!(f, "-")?;
                }
                if u {
                    write!(f, "u")?;
                } else {
                    write!(f, "-")?;
                }
                if r {
                    write!(f, "r")?;
                } else {
                    write!(f, "-")?;
                }
                if d {
                    write!(f, "d")?;
                } else {
                    write!(f, "-")?;
                }
                write!(f, " ")?;
            }
            if let Some((_, y_highlight)) = self.highlighted_pos {
                if y_highlight as usize == y {
                    write!(f, "<")?;
                }
            }
        }
        if self.highlighted_pos.is_some() {
            writeln!(f, "")?;
            write!(f, "  ")?;

            for x in 0..self.size.0 {
                if let Some((x_highlight, _)) = self.highlighted_pos {
                    if x_highlight as usize == x {
                        write!(f, "    ^ ")?;
                        continue;
                    }
                }
                write!(f, "      ")?;
            }
        }
        Ok(())
    }
}
impl Grid {
    fn reset_highlight(&mut self) {
        self.highlighted_pos = None;
    }
    fn set_highlight(&mut self, pos: (isize, isize)) {
        self.highlighted_pos = Some(pos);
    }
    fn has_been_enerigzed_from_this_dir(&self, pos: (isize, isize), direction: Direction) -> bool {
        let x = self.get_pos_energized_state(pos);
        match direction {
            Direction::Up => x.1,
            Direction::Down => x.3,
            Direction::Left => x.0,
            Direction::Right => x.2,
        }
    }

    fn get_pos_energized_state(&self, pos: (isize, isize)) -> (bool, bool, bool, bool) {
        let x = self.positions[pos_to_index(pos.0 as usize, pos.1 as usize, self.size.0)];
        x
    }
    fn set_energized(&mut self, pos: (isize, isize), from_direction: Direction) {
        let x = self
            .positions
            .get_mut(pos_to_index(pos.0 as usize, pos.1 as usize, self.size.0))
            .unwrap();
        match from_direction {
            Direction::Up => x.1 |= true,
            Direction::Down => x.3 |= true,
            Direction::Left => x.0 |= true,
            Direction::Right => x.2 |= true,
        }
    }
    fn get_energized(&self, x: usize, y: usize) -> bool {
        let x = self.positions[pos_to_index(x, y, self.size.0)];
        Self::is_pos_enerized(x)
    }
    fn is_pos_enerized(x: (bool, bool, bool, bool)) -> bool {
        x.0 | x.1 | x.2 | x.3
    }

    fn new(length: usize, height: usize) -> Self {
        Grid {
            highlighted_pos: None,
            size: (length, height),
            positions: vec![(false, false, false, false); length * height],
        }
    }
    fn get_heat(&self) -> usize {
        self.positions
            .iter()
            .filter(|x| Self::is_pos_enerized(**x))
            .count()
    }
    fn is_in_bounds(&self, pos: (isize, isize)) -> bool {
        pos.1 >= 0 && pos.0 >= 0 && pos.0 < self.size.0 as isize && pos.1 < self.size.1 as isize
    }
}

fn pos_to_index(x: usize, y: usize, x_size: usize) -> usize {
    x + y * x_size
}
