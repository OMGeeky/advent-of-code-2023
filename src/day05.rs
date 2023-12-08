use std::collections::HashMap;

use crate::*;

pub struct Day05;
impl Day for Day05 {
    const DAY_NUM: u8 = 05;
    type Input = String;
    type Output = usize;

    fn get_test_data() -> Self::Input {
        "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"
            .to_string()
    }

    fn get_test_result() -> Self::Output {
        35
    }

    fn run(data: Self::Input) -> Self::Output {
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        println!("targets: {}", target_seeds);
        let dict = parse_to_maps(data);
        let result = target_seeds
            .trim()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|seed_pos| {
                let seed_pos = seed_pos.parse().unwrap();
                let target = map_seed_to_position(seed_pos, &dict);
                dbg!(target);
                target
            })
            .min()
            .unwrap();

        result
    }
}
impl DayPart2 for Day05 {
    fn run_part2(data: Self::Input) -> Self::Output {
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        println!("targets: {}", target_seeds);
        let dict = parse_to_maps(data);
        let result = target_seeds
            .trim()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .chunks(2)
            .into_iter()
            .map(|mut seed_pos| {
                let start = seed_pos.next().unwrap().parse().unwrap();
                let end: usize = seed_pos.next().unwrap().parse().unwrap();
                let end = start + end;
                println!("targets: {}", target_seeds);
                println!("getting seed positions for range: {}..{}", start, end);
                println!("that is {} of positions to check", end-start);
                println!("=======================================================================================================");
                let min = (start..end)
                    .into_iter()
                    .map(|seed_pos| {
                        // println!("Starting to map Seed pos to location: {}", seed_pos);
                        let target = map_seed_to_position(seed_pos, &dict);
                        // println!("Got pos from seed: {}=>{}", seed_pos, target);
                        // dbg!(target);
                        target
                    })
                    .min()
                    .unwrap();
                println!("=======================================================================================================");
                min
            })
            .min()
            .unwrap();

        result
    }

    fn get_test_result_part2() -> Self::Output {
        46
    }

    fn get_test_data_part2() -> Self::Input {
        Self::get_test_data()
    }
}
fn parse_to_maps<'a>(data: impl Iterator<Item = &'a str>) -> HashMap<Kind, Map> {
    let mut dict = HashMap::new();

    for (_, lines) in (&data.group_by(|x| x.len() > 0))
        .into_iter()
        .filter(|(x, _)| *x)
    {
        let lines = lines.collect::<Vec<_>>();
        dbg!(&lines);
        let map = Map::from_vec(lines);
        dbg!(&map);
        dict.insert(map.from, map);
    }
    dict
}

fn map_seed_to_position(var_name: usize, dict: &HashMap<Kind, Map>) -> usize {
    let mut kind = Kind::Seed;
    let mut source = var_name;
    while kind != Kind::Location {
        let map = dict.get(&kind).unwrap();
        // println!("Mapping {:?} to {:?}", kind, map.to);
        kind = map.to;
        let get_mapped = map.get_mapped(source);
        // println!("Mapping {:?} to {:?}", source, get_mapped);
        source = get_mapped;
    }
    // println!("destination: {source}");
    source
}

use data::*;
use itertools::Itertools;
mod data {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Map {
        pub from: Kind,
        pub to: Kind,
        ranges: Vec<Range>,
    }

    impl Map {
        pub fn from_vec(value: Vec<&str>) -> Self {
            value.join("\n").into()
        }
        pub fn get_mapped(&self, source: usize) -> usize {
            for range in &self.ranges {
                if let Some(target) = range.get_mapped(source) {
                    return target;
                }
            }
            return source;
        }
    }
    impl<T> From<T> for Map
    where
        T: AsRef<str>,
    {
        fn from(value: T) -> Self {
            let value = value.as_ref();
            let mut values = value.trim().lines();
            let mut map_name = values
                .next()
                .unwrap()
                .strip_suffix(" map:")
                .unwrap()
                .split("-to-");
            let from = map_name.next().unwrap().into();
            let to = map_name.next().unwrap().into();
            let ranges = values.map(|x| x.into()).collect();
            Self { from, to, ranges }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Kind {
        Seed,
        Soil,
        Fertilizer,
        Water,
        Light,
        Temperature,
        Humidity,
        Location,
    }

    impl<T: AsRef<str>> From<T> for Kind {
        fn from(value: T) -> Self {
            match value.as_ref() {
                "soil" => Kind::Soil,
                "fertilizer" => Kind::Fertilizer,
                "humidity" => Kind::Humidity,
                "light" => Kind::Light,
                "location" => Kind::Location,
                "seed" => Kind::Seed,
                "temperature" => Kind::Temperature,
                "water" => Kind::Water,
                other => panic!("got an invalid kind to parse: {other}"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Range {
        destination_start: usize,
        source_start: usize,
        length: usize,
    }
    impl Range {
        fn get_mapped(&self, source: usize) -> Option<usize> {
            if source >= self.source_start && source < self.source_start + self.length {
                Some(self.destination_start + source - self.source_start)
            } else {
                None
            }
        }
    }
    impl<T: AsRef<str>> From<T> for Range {
        fn from(value: T) -> Self {
            let value = value.as_ref();
            let mut values = value.trim().split(' ');
            let destination_start = values.next().unwrap().parse().unwrap();
            let source_start = values.next().unwrap().parse().unwrap();
            let length = values.next().unwrap().parse().unwrap();
            Self {
                destination_start,
                source_start,
                length,
            }
        }
    }
}

#[cfg(test)]
mod my_tests {
    use super::*;
    extern crate test;
    use test::Bencher;
    fn get_sample_map() -> HashMap<Kind, Map> {
        let data: String = Day05::get_test_data();
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        println!("targets: {}", target_seeds);
        let dict = parse_to_maps(data);
        dict
    }
    fn get_sample_map_real() -> HashMap<Kind, Map> {
        let data: String = read_input(05);
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        println!("targets: {}", target_seeds);
        let dict = parse_to_maps(data);
        dict
    }
    #[bench]
    fn bench_mapping(b: &mut Bencher) {
        let mapper = get_sample_map();
        b.iter(|| {
            (0..10000)
                .into_iter()
                .for_each(|x| {map_seed_to_position(x, &mapper);})
        })
    }
    #[bench]
    fn bench_mapping2(b: &mut Bencher) {
        let mapper = get_sample_map();
        b.iter(|| map_seed_to_position(5, &mapper))
    }
    #[bench]
    fn bench_parsing(b: &mut Bencher) {
        b.iter(get_sample_map)
    }
    #[bench]
    fn bench_parsing2(b: &mut Bencher) {
        let data: String = Day05::get_test_data();
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        println!("targets: {}", target_seeds);
        b.iter(|| parse_to_maps(data.clone()))
    }
    #[bench]
    fn bench_parsing_real(b: &mut Bencher) {
        b.iter(get_sample_map_real)
    }
    #[bench]
    fn bench_parsing2_real(b: &mut Bencher) {
        let data: String = read_input(05);
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        println!("targets: {}", target_seeds);
        b.iter(|| parse_to_maps(data.clone()))
    }
}
