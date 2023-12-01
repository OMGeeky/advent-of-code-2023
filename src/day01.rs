use crate::{Day, DayPart2};

pub struct Day01;
impl Day for Day01 {
    const DAY_NUM: u8 = 01;

    type Input = String;

    type Output = usize;

    fn get_test_data() -> Vec<Self::Input> {
        vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ]
    }

    fn get_test_result() -> Self::Output {
        142
    }

    fn run(data: Vec<Self::Input>) -> Self::Output {
        let mut sum = 0;
        for line in data {
            let mut digits = line.chars().filter(|c| c.is_digit(10));
            let first = digits.next().unwrap_or(' ');
            let last = digits.last().unwrap_or(first);
            let num = format!("{}{}", first, last)
                .trim()
                .parse::<usize>()
                .unwrap();
            println!(
                "{} + {} = {}; total: {} => {}",
                first,
                last,
                num,
                sum,
                sum + num
            );
            sum += num;
        }
        sum
    }
}
impl DayPart2 for Day01 {
    fn get_test_data_part2() -> Vec<String> {
        vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ]
    }
    fn get_test_result_part2() -> usize {
        281
    }

    fn run_part2(data: Vec<String>) -> usize {
        let mut sum = 0;
        for line in data {
            let mut l = String::new();
            for i in 0..line.len() {
                let part = &line[i..];
                // println!("{}", part);
                let c = part.chars().next().unwrap();
                if c.is_digit(10) {
                    l.push(c);
                }
                if part.starts_with("one") {
                    l.push('1')
                } else if part.starts_with("two") {
                    l.push('2')
                } else if part.starts_with("three") {
                    l.push('3')
                } else if part.starts_with("four") {
                    l.push('4')
                } else if part.starts_with("five") {
                    l.push('5')
                } else if part.starts_with("six") {
                    l.push('6')
                } else if part.starts_with("seven") {
                    l.push('7')
                } else if part.starts_with("eight") {
                    l.push('8')
                } else if part.starts_with("nine") {
                    l.push('9')
                }
            }
            let mut digits = l.chars().filter(|c| c.is_digit(10));
            let first = digits.next().unwrap_or(' ');
            let last = digits.last().unwrap_or(first);
            // println!("'{}' '{}'", first, last);
            let num = format!("{}{}", first, last)
                .trim()
                .parse::<usize>()
                .unwrap();
            println!(
                "{} + {} = {}; total: {} => {}",
                first,
                last,
                num,
                sum,
                sum + num
            );
            sum += num;
        }
        sum
    }
}
