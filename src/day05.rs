use std::collections::HashMap;

use crate::*;

pub struct Day05;
impl Day for Day05 {
    const DAY_NUM: u8 = 5;
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
        let dict = Map::combine_maps(dict);
        let dict = {
            let mut m = HashMap::new();
            m.insert(Kind::Seed, dict);
            m
        };
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
                    .map(|seed_pos| {
                        // println!("Starting to map Seed pos to location: {}", seed_pos);
                        let target = map_seed_to_position(seed_pos, &dict);
                        // println!("Got pos from seed: {}=>{}", seed_pos, target);
                        dbg!(target);
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

    for (x, lines) in (&data.group_by(|x| x.len() > 0)).into_iter()
    //.filter(|(x, _)| *x)
    {
        let lines = lines.collect::<Vec<_>>();
        dbg!(&x, &lines);
        if !x {
            continue;
        }
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
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Map {
        pub from: Kind,
        pub to: Kind,
        pub ranges: Vec<Range>,
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
            source
        }
        pub fn combine_maps(maps: HashMap<Kind, Map>) -> Map {
            let mut maps = maps;
            dbg!("combining following maps to a single one: ", &maps);
            let mut current: Map = maps.remove(&Kind::Seed).unwrap();
            let mut kind = current.to;
            while kind != Kind::Location {
                dbg!("current kind:", &kind);
                let to = maps.get(&kind).unwrap();
                current = Map::combine_two_maps(&current, to);
                kind = current.to;
            }
            current
        }
        pub fn combine_two_maps(from: &Map, to: &Map) -> Map {
            dbg!("combining these two maps to one:", from, to);
            let from_kind = from.from;
            let to_kind = to.to;
            let mut ranges = Vec::<Range>::new();
            'from_ranges: for range in &from.ranges {
                let mut map_start = range.destination_start;
                let mut map_end = range.length + map_start - 1;
                let mut map_length = range.length;

                'to_ranges: for to_range in &to.ranges {
                    let to_source_start = to_range.source_start;
                    let to_source_len = to_range.length;
                    let to_range_end = to_source_start + to_source_len - 1;
                    let to_source_end = map_start + map_length;
                    if to_source_start <= map_start {
                        let offset_from_this_range = map_start - to_source_start;
                        if to_range_end < to_source_end {
                            dbg!(
                                "from range fits fully into target range",
                                offset_from_this_range
                            );
                            let new_range = Range {
                                destination_start: to_range.destination_start
                                    + offset_from_this_range,
                                source_start: range.source_start,
                                length: map_length,
                            };
                            ranges.push(new_range);
                            continue 'from_ranges;
                        } else if to_range_end < map_start {
                            dbg!("from range is not inside this range at all");

                            continue 'to_ranges;
                        } else {
                            dbg!("from range fits partially into this range");

                            let length = 0;
                            let new_range = Range {
                                destination_start: to_range.destination_start
                                    + offset_from_this_range,
                                source_start: range.source_start,
                                length,
                            };
                            map_start += offset_from_this_range;
                            map_length -= length;
                            ranges.push(new_range);
                            dbg!("There was a range that could not completely be mapped, so only the start got mapped...", &range, &to_range);
                            continue 'to_ranges;
                        }
                    } else if to_range.source_start <= map_end {
                        dbg!("from range fits partially into this range");

                        let offset = map_end - to_range.source_start;
                        let new_range = Range {
                            destination_start: to_range.destination_start + offset,
                            source_start: to_range.source_start,
                            length: offset,
                        };
                        ranges.push(new_range);
                        dbg!("There was a range that could not completely be mapped, so only the end got mapped...", &range, &to_range);
                    }
                }
            }
            let map = Self {
                from: from_kind,
                to: to_kind,
                ranges,
            };
            dbg!(
                "combined two maps into one:",
                from,
                to,
                "were combined into: ",
                &map
            );
            map
        }
    }
    impl<T> From<T> for Map
    where
        T: AsRef<str>,
    {
        fn from(value: T) -> Self {
            let value = value.as_ref();
            dbg!(&value);
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
        pub destination_start: usize,
        pub source_start: usize,
        pub length: usize,
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
        dbg!("Test data:", &data);
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        dbg!("targets: {}", target_seeds);
        parse_to_maps(data)
    }
    fn get_sample_map_real() -> HashMap<Kind, Map> {
        let data: String = read_input(5);
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        println!("targets: {}", target_seeds);
        parse_to_maps(data)
    }
    #[bench]
    fn bench_mapping(b: &mut Bencher) {
        let mapper = get_sample_map();
        b.iter(|| {
            (0..10000).for_each(|x| {
                map_seed_to_position(x, &mapper);
            })
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
        let data: String = read_input(5);
        let mut data = data.lines().map(|x| x.trim());
        let target_seeds = data.next().unwrap();
        println!("targets: {}", target_seeds);
        b.iter(|| parse_to_maps(data.clone()))
    }
    #[test]
    fn test_parsing_combined() {
        let maps = get_sample_map();
        let map = Map::combine_maps(maps);
        dbg!(map);
        assert!(false);
    }

    #[test]
    fn test_parsing_combined2() {
        let maps = {
            let mut d = HashMap::new();
            d.insert(
                Kind::Seed,
                Map {
                    from: Kind::Seed,
                    to: Kind::Light,
                    ranges: vec![
                        Range {
                            source_start: 10,
                            destination_start: 50,
                            length: 5,
                        },
                        Range {
                            source_start: 60,
                            destination_start: 20,
                            length: 5,
                        },
                    ],
                },
            );
            d.insert(
                Kind::Light,
                Map {
                    from: Kind::Light,
                    to: Kind::Location,
                    ranges: vec![
                        Range {
                            source_start: 50,
                            destination_start: 150,
                            length: 5,
                        },
                        Range {
                            source_start: 20,
                            destination_start: 210,
                            length: 5,
                        },
                    ],
                },
            );
            d
        };
        let expected = Map {
            from: Kind::Seed,
            to: Kind::Location,
            ranges: vec![
                Range {
                    source_start: 10,
                    destination_start: 150,
                    length: 5,
                },
                Range {
                    source_start: 60,
                    destination_start: 210,
                    length: 5,
                },
            ],
        };
        let map = Map::combine_maps(maps);
        dbg!("result of the combining: ", &map);

        assert_eq!(map, expected);
        panic!("i want to see the results");
    }

    #[test]
    fn test_parsing_combined3() {
        let maps = get_sample_map();

        let map = Map::combine_maps(maps.clone());
        dbg!("result of the combining: ", &map);
        (0..1000).for_each(|x| {
            let normal = map_seed_to_position(x, &maps);
            let combined = map.get_mapped(x);
            assert_eq!(normal, combined);
        });
    }

    #[bench]
    fn bench_parsing_combined(b: &mut Bencher) {
        b.iter(|| {
            let maps = get_sample_map();
            let map = Map::combine_maps(maps);

            dbg!(map);
        })
    }
}
