use crate::*;

pub struct Day03;
impl Day for Day03 {
    const DAY_NUM: u8 = 03;
    type Input = String;
    type Output = i32;

    fn get_test_data() -> Self::Input {
 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string()
    }

    fn get_test_result() -> Self::Output {
        4361
    }

    fn run(data: Self::Input) -> Self::Output {
        let data = data.lines().map(|x|x.to_string()).collect::<Vec<_>>();
        let mut result = 0;
        dbg!(&data);
        for y in 0..data.len() {
            let mut x: isize = -1;
            let vertical_len = data.len() as i32;
            let horizontal_len = data[y].len() as i32;
            let p = split(&data[y]);
            println!("{:?}", &p);

            for e in &p {
                if e.len() == 0 {
                    x += 1;
                } else {
                    x += 1;
                    if e.chars().all(|c| c.is_digit(10)) {
                        if check_number(&data, e, x, y, horizontal_len, vertical_len) {
                            let num: i32 = e.parse().unwrap();
                            println!("{e:>4} adjacent => {result} + {num} => {}", result + num);
                            result += num;
                        } else {
                            println!("{e:>4} not adjacent")
                        }
                    } else if e.chars().any(|c| c.is_digit(10)) {
                        println!("{e:>4} not all chars are numbers...");
                    }
                    x += e.len() as isize - 1;
                }
            }
        }
        result
    }
}

fn check_number(
    data: &Vec<String>,
    e: &str,
    x: isize,
    y: usize,
    horizontal_len: i32,
    vertical_len: i32,
) -> bool {
    let len = e.len();
    'horizontal: for x_offset in (-1)..(len as i32 + 1) {
        'vertical: for y_offset in (-1)..2 {
            let x1 = x as i32 + x_offset;
            let y1 = y as i32 + y_offset;
            if x1 < 0 || x1 >= horizontal_len {
                continue 'horizontal;
            }
            if y1 < 0 || y1 >= vertical_len {
                continue 'vertical;
            }
            let checked_char = data[y1 as usize].chars().collect::<Vec<char>>()[x1 as usize];
            if y_offset == 0 && x_offset >= 0 && x_offset < len as i32 {
                println!("checked position is inside own element: {:>3}x{:>3}y + {:>3}x{:>3}y=> {:>2}x{:>2}y => {:>4}",
                    x_offset, y_offset, x, y, x1, y1, checked_char);
                continue;
            }
            if checked_char != '.' {
                println!(
                    "{:>3}x{:>3}y + {:>3}x{:>3}y=> {:>2}x{:>2}y => {:>4}",
                    x_offset, y_offset, x, y, x1, y1, checked_char
                );
                return true;
            }
        }
    }
    false
}

fn split(data: &String) -> Vec<String> {
    let mut res = vec![];
    let mut current = String::new();
    let mut last = data.chars().next().unwrap();
    for c in data.chars() {
        if c.is_ascii_digit() {
            current.push(c);
        } else {
            if last.is_digit(10) {
                res.push(current);
                current = String::new();
            }
            if c != '.' {
                current.push(c);
            }
            res.push(current);
            current = String::new();
        }
        last = c;
    }
    if last.is_digit(10) {
        res.push(current);
    }
    res
}
#[test]
fn test_split1() {
    let x = split(&"467..114..".to_string());
    assert_eq!(vec!["467", "", "", "114", "", ""], x);
}

#[test]
fn test_split2() {
    let x = split(&"...*......".to_string());
    assert_eq!(vec!["", "", "", "*", "", "", "", "", "", "",], x);
}

#[test]
fn test_split3() {
    let x = split(&"..35..633.".to_string());
    assert_eq!(vec!["", "", "35", "", "", "633", "",], x);
}
