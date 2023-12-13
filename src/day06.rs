use crate::*;

pub struct Day06;
impl Day for Day06 {
    const DAY_NUM: u8 = 6;
    type Input = Scoreboard;
    type Output = u64;

    fn get_test_data() -> Self::Input {
        "Time:      7  15   30
Distance:  9  40  200"
            .to_string()
            .into()
    }

    fn get_test_result() -> Self::Output {
        288
    }

    fn run(data: Self::Input) -> Self::Output {
        dbg!(&data);
        data.races
            .iter()
            .map(Race::get_amount_of_winning_accelarations)
            .product()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scoreboard {
    races: Vec<Race>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Race {
    duration: u64,
    distance: u64,
}
impl Race {
    fn get_amount_of_winning_accelarations(&self) -> <Day06 as Day>::Output {
        if self.distance == 0 {
            dbg!("This makes little sense");
            return self.duration;
        }
        let mut result = 0;
        for i in 0..self.duration {
            let time_left = self.duration - i;
            if i * time_left > self.distance{
                result += 1;
            }
        }
        result
    }
}

impl From<String> for Scoreboard {
    fn from(value: String) -> Self {
        let mut lines = value.lines();
        let durations = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split_ascii_whitespace();
        let distances = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Distance:")
            .unwrap()
            .trim()
            .split_ascii_whitespace();
        let mut races = vec![];
        for (duration, distance) in durations.zip(distances) {
            let duration: u64 = duration.parse().unwrap();
            let distance: u64 = distance.parse().unwrap();
            races.push(Race { duration, distance });
        }

        Self { races }
    }
}
