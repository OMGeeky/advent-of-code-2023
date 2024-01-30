use crate::*;

pub struct Day09;
impl Day for Day09 {
    const DAY_NUM: u8 = 09;
    type Input = String;
    type Output = i32;

    fn get_test_data() -> Self::Input {
        "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45".to_string()
    }

    fn get_test_result() -> Self::Output {
        114
    }

    fn run(data: Self::Input) -> Self::Output {
        data.lines()
        .map(|x|{
            x.trim().split(' ').map(|x|{
                // println!("{}",x);
                x.parse::<i32>().unwrap()
            })
        }).map(|x|{
           get_sub_rec(&x.collect())
        })
        .map(|x:i32|{
            x
        }).sum()
    }
}

fn get_sub_rec<'a>(x: &Vec<i32>) -> i32 {
    println!("{:?}", x);
    let x2 = get_sub(x.iter()).collect::<Vec<_>>();
    println!("=> {:?}", &x2);
    if x2.len() < 2 {
        dbg!(x2);
        todo!("What to do if we are all the way down?")
    }
    let last = *x.last().unwrap();
    if x2[0] == 0 && x2[1] == 0{
        return last;
    }
    let sub = get_sub_rec(&x2);
    dbg!(&sub, &last, sub + last);

    last + sub
}
fn get_sub<'a>(x: impl Iterator<Item = &'a i32> + 'a)-> Box<dyn Iterator<Item = i32> + 'a>{
    Box::new(x.map_windows(|[a,b]|{
        *b-(*a)
    }))
}


impl DayPart2 for Day09{
    fn run_part2(data: Self::Input) -> Self::Output { data.lines()
        .map(|x|{
            x.trim().split(' ').map(|x|{
                // println!("{}",x);
                x.parse::<i32>().unwrap()
            })
        }).map(|x|{
           get_sub_rec_part2(&x.collect())
        })
        .map(|x:i32|{
            x
        }).sum()
    }

    fn get_test_result_part2() -> Self::Output {
        2
    }

    fn get_test_data_part2() -> Self::Input {
        Self::get_test_data()
    }
}
fn get_sub_rec_part2<'a>(x: &Vec<i32>) -> i32 {
    println!("{:?}", x);
    let x2 = get_sub(x.iter()).collect::<Vec<_>>();
    println!("=> {:?}", &x2);
    if x2.len() < 2 {
        dbg!(x2);
        todo!("What to do if we are all the way down?")
    }
    let last = *x.first().unwrap();
    if x2[0] == 0 && x2[1] == 0{
        return last;
    }
    let sub = get_sub_rec_part2(&x2);
    dbg!(&sub, &last, sub - last);

    last - sub
}