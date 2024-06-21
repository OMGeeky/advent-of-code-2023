#![feature(test)]
#![feature(iter_map_windows)]

pub fn read_input<T>(day: u8) -> T
where
    T: From<String>,
{
    let filename = format!("input/day{:02}.txt", day);
    println!("Reading input from {}", filename);
    let input = std::fs::read_to_string(filename).unwrap();
    utils::day!();
    input.into()
}
mod day01;
pub use day01::*;
mod day02;
pub use day02::*;
mod day03;
pub use day03::*;
mod day04;
pub use day04::*;
mod day05;
pub use day05::*;
mod day06;
pub use day06::*;
mod day07;
pub use day07::*;

mod day09;
pub use day09::*;

mod day10;
pub use day10::*;

mod day14;
pub use day14::*;

mod day19;
pub use day19::*;

pub mod utils {
    #[macro_export]
    macro_rules! day {
        ($num: expr) => {
            paste::item! {
                fn [< run_test_day $num >] () {
                    use [< day $num>];
                    type D = [< Day $num>];
                    let test_res = D::run(D::get_test_data());
                    println!("Day {}: {:?}", $num, test_res);
                    assert_eq!(D::get_test_result(), test_res);
                }
                fn [< run_day $num >] () {
                    let test_res = [< day $num>]::run(read_input($num));
                    println!("Day {} result: \n{:?}", $num, test_res);
                }
                fn [< run_test_day $num _part2>] () {
                    let test_res = [< day $num>]::run_part2([< day $num>]::get_test_data_part2());
                    println!("Day {} part2 test: {:?}", $num, test_res);
                    assert_eq!([< day $num>]::get_test_result_part2(), test_res);
                }
                fn [< run_day $num _part2>] () {
                    let test_res = [< day $num>]::run_part2(read_input($num));
                    println!("Day {} part 2 result: \n{:?}", $num, test_res);
                }
            }
        };
        () => {
            //hi
        };
    }
    pub use day;
    #[macro_export]
    macro_rules! run {
        // ($num: expr) => {
        //     paste::item! {
        //         [< run_test_day $num >] ();
        //         [< run_day $num >] ();
        //         [< run_test_day $num _part2 >] ();
        //         [< run_day $num _part2>] ();
        //     }
        // };
        () => {
            // run!(01);
            // run!(02);
            // run!(03);
            // run!(04);
            // run!(05);
            // run!(06);
            // run!(07);
            // run!(08);
            // run!(09);
        };
    }
    pub use run;
}
pub trait Day
where
    <Self as Day>::Input: std::fmt::Debug + From<String>,
    <Self as Day>::Output: std::fmt::Debug + PartialEq,
{
    const DAY_NUM: u8;
    type Input;
    type Output;

    fn get_test_data() -> Self::Input;
    fn get_test_result() -> Self::Output;
    fn get_multiple_test_data() ->  Box<impl Iterator<Item=Self::Input>>{
        Box::new([Self::get_test_data()].into_iter())
    }
    fn get_multiple_test_result() -> Box<impl Iterator<Item=Self::Output>>{
        Box::new([Self::get_test_result()].into_iter())
    }
    fn run(data: Self::Input) -> Self::Output;
}
pub trait DayPart2: Day {
    fn run_part2(data: Self::Input) -> Self::Output;
    fn get_test_result_part2() -> Self::Output;
    fn get_test_data_part2() -> Self::Input;
    fn get_multiple_test_data_part2() ->  Box<impl Iterator<Item=Self::Input>>{
        Box::new([Self::get_test_data_part2()].into_iter())
    }
    fn get_multiple_test_result_part2() -> Box<impl Iterator<Item=Self::Output>>{
        Box::new([Self::get_test_result_part2()].into_iter())
    }
}
pub trait DayConvenience: Day {
    fn run_day_tests(){
        let test_inputs = Self::get_multiple_test_data();
        let expected_results = Self::get_multiple_test_result();
        (test_inputs).zip(expected_results).enumerate().for_each(|(i,(input, expected))|{
            let test_res = Self::run(input);
            println!("Day {} test {i}: {:?}", Self::DAY_NUM, test_res);
            assert_eq!(expected, test_res);
        })
    }
    fn run_day() {
        let test_res = Self::run(read_input(Self::DAY_NUM));
        println!("Day {} result: \n{:?}", Self::DAY_NUM, test_res);
    }
    fn part1() {
        Self::run_day_tests();
        Self::run_day();
    }
}
impl<T> DayConvenience for T where T: Day {}

pub trait DayPart2Convenience: DayPart2 + DayConvenience {
    fn run_day_part2_tests(){
        let test_inputs = Self::get_multiple_test_data_part2();
        let expected_results = Self::get_multiple_test_result_part2();
        (test_inputs).zip(expected_results).enumerate().for_each(|(i,(input, expected))|{
            let test_res = Self::run_part2(input);
            println!("Day {} part 2 test {i}: {:?}", Self::DAY_NUM, test_res);
            assert_eq!(expected, test_res);
        })
    }
    fn run_day_part2() {
        let test_res = Self::run_part2(read_input(Self::DAY_NUM));
        println!("Day {} part 2 result: \n{:?}", Self::DAY_NUM, test_res);
    }
    fn part2(){
        Self::run_day_part2_tests();
        Self::run_day_part2();
    }
    fn run_all() {
        Self::part1();
        Self::part2();
    }
}
impl<T> DayPart2Convenience for T where T: DayPart2 {}
