use std::ascii;

use crate::*;

pub struct Day15;
impl Day for Day15 {
    const DAY_NUM: u8 = 15;
    type Input = String;
    type Output = usize;

    fn get_test_data() -> Self::Input {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".into()
    }
    fn get_multiple_test_result() -> Box<impl Iterator<Item = Self::Output>> {
        Box::new([52, 30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231].into_iter())
    }
    fn get_multiple_test_data() -> Box<impl Iterator<Item = Self::Input>> {
        Box::new(
            [
                "HASH".into(),
                "rn=1".into(),
                "cm-".into(),
                "qp=3".into(),
                "cm=2".into(),
                "qp-".into(),
                "pc=4".into(),
                "ot=9".into(),
                "ab=5".into(),
                "pc-".into(),
                "pc=6".into(),
                "ot=7".into(),
            ]
            .into_iter(),
        )
    }

    fn get_test_result() -> Self::Output {
        1320
    }

    fn run(data: Self::Input) -> Self::Output {
        run_part1(&data)
    }
}
fn run_part1(data: &str) -> usize {
    assert!(!data.contains('\n'));

    data.split(',')
        .into_iter()
        .filter_map(str::as_ascii)
        .map(algorythm)
        .sum()
}
fn algorythm(data: &[std::ascii::Char]) -> usize {
    assert!(!data.contains(&ascii::Char::Comma));
    let mut result = 0;

    for x in data
        .into_iter()
        // .inspect(|x| print!("{x}:"))
        .map(get_ascii_value)
    {
        result = ((result + x) * 17) % 256;
        // println!("{result}");
    }
    result
}
fn get_ascii_value(c: &std::ascii::Char) -> usize {
    c.to_u8() as usize
}
