use std::ops::RangeInclusive;
use std::usize;

use cached::proc_macro::cached;
use cached::SizedCache;

use crate::*;

pub struct Day04;
impl Day for Day04 {
    const DAY_NUM: u8 = 04;
    type Input = String;
    type Output = usize;

    fn get_test_data() -> Self::Input {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string()
    }

    fn get_test_result() -> Self::Output {
        13
    }

    fn run(data: Self::Input) -> Self::Output {
        data.lines()
            .map(|x| x.trim())
            .map(|line| {
                let card = Card::from(line);
                // dbg!(&card);

                let numbers = card
                    .own_numbers
                    .iter()
                    .filter(|x| card.winning_numbers.contains(x))
                    .count();

                println!("{:>4} from {:?}", numbers, card);
                match numbers > 0 {
                    true => 2usize.pow(numbers as u32 - 1),
                    false => 0,
                }
            })
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    own_numbers: Vec<usize>,
}
impl<'a, T> From<T> for Card
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let value: &str = value.as_ref();
        // dbg!(&value);
        let value = value
            .strip_prefix("Card")
            .expect("line did not start with 'Card '");
        let (id, value) = value.split_once(':').expect("Id was not followed by ':'");
        let id = id.trim().parse().unwrap();

        let (winning_numbers, own_numbers) = value.split_once('|').unwrap();
        let winning_numbers = winning_numbers
            .trim()
            .split(' ')
            .filter_map(|x| x.parse().ok());
        // dbg!(&winning_numbers);
        // let winning_numbers = winning_numbers.map(|x| x.parse().unwrap()).collect();
        let winning_numbers = winning_numbers.collect();
        let own_numbers = own_numbers.trim().split(' ').filter_map(|x| x.parse().ok());
        // dbg!(&own_numbers);
        // let own_numbers = own_numbers.map(|x| x.parse().unwrap()).collect();
        let own_numbers = own_numbers.collect();

        Self {
            id,
            winning_numbers,
            own_numbers,
        }
    }
}

impl DayPart2 for Day04 {
    fn run_part2(data: Self::Input) -> Self::Output {
        let data = data
            .lines()
            .map(|x| x.trim())
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        let len = data.len();
        let mut dict = Vec::with_capacity(len + 1);
        dict.push(0);
        data.iter().for_each(|line| {
            let card = Card::from(line);
            // dbg!(&card);

            let numbers = card
                .own_numbers
                .iter()
                .filter(|x| card.winning_numbers.contains(x))
                .count();

            println!("{:>4} from {:?}", numbers, card);
            dict.push(numbers);
        });
        println!("{:?}", &dict.iter().enumerate().collect::<Vec<(_, _)>>());
        get_for_range(1, dict.len() - 1, &dict, 0)
    }

    fn get_test_result_part2() -> Self::Output {
        30
    }

    fn get_test_data_part2() -> Self::Input {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string()
    }
}

#[cached(
    type = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(500) }",
    convert = r#"{ format!("{:?}=>{:?}", range, dict) }"#
)]

// #[cached(
//     type = "SizedCache<String, usize>",
//     create = "{ SizedCache::with_size(100) }",
//     convert = r#"{ format!("{}{}", a, b) }"#
// )]
fn get_amount_from_range(range: RangeInclusive<usize>, dict: &Vec<usize>) -> usize {
    println!("getting from range: {:?}", range);
    let sum = range
        .clone()
        .map(|x| get_amount_from_id(x, &dict))
        // .inspect(|x|println!("got {}",x))
        .sum();
    println!("got {} from range: {:?}", sum, range);
    sum
}
#[cached(
    type = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(200) }",
    convert = r#"{ format!("{}:{:?}", id, dict) }"#
)]
fn get_amount_from_id(id: usize, dict: &Vec<usize>) -> usize {
    let e = dict[id];
    println!("{:>4}=>{:>4}", id, e);
    if e == 0 {
        println!("reached last layer");
        1
    } else {
        let i1 = id + 1;
        let e1 = i1 + e;
        println!("going deeper: {}..{}", i1, e1);
        get_amount_from_range(i1..=e1, dict)
    }
}

// #[cached(
//     type = "SizedCache<String, usize>",
//     create = "{ SizedCache::with_size(500) }",
//     convert = r#"{ format!("[{}]{:?}=>{:?}", layer, id, dict) }"#
// )]
fn get_for_id(id: usize, dict: &Vec<usize>, layer: usize) -> usize {
    let own_amount = dict[id];
    if own_amount == 0 {
        // p!(layer, "- {id} no subs");
        return 1;
    }
    let first = id + 1;
    let last = id + own_amount;
    let others = get_for_range(first, last, dict, layer);
    // p!(layer, "- {id} subs: {}", others);
    return others + 1;
}

// #[cached(
//     type = "SizedCache<String, usize>",
//     create = "{ SizedCache::with_size(500) }",
//     convert = r#"{ format!("[{}]({}..={})=>{:?}", layer, first, last, dict) }"#
// )]
fn get_for_range(first: usize, last: usize, dict: &Vec<usize>, layer: usize) -> usize {
    // p!(layer, "getting for range: {first}..={last}");
    let others: usize = (first..=last).map(|i| get_for_id(i, dict, layer + 1)).sum();
    // p!(layer, "got for range: {first}..={last}=> {others}");
    others
}
#[macro_export]
macro_rules! p {
    () => {
        $crate::print!("\n")
    };
    ($level:ident,$($arg:tt)*) => {{
        println!("{:indent$}{}", "", format!($($arg)*), indent=$level*5);
    }};
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}
