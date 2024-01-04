use std::collections::HashMap;

use itertools::{Itertools, repeat_n};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::*;

pub struct Day19;
impl Day for Day19 {
    const DAY_NUM: u8 = 19;
    type Input = String;
    type Output = usize;

    fn get_test_data() -> Self::Input {
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            .to_string()
    }

    fn get_test_result() -> Self::Output {
        19114
    }

    fn run(data: Self::Input) -> Self::Output {
        let data = Game::from_str_p1(data);
        dbg!(&data);
        let mut sum = 0;
        for element in &data.elements {
            let result = data.get_result_p1(element);
            sum += match result {
                RuleResult::Accepted => {
                    println!("Part {:?} has been accepted", element);
                    element.a + element.m + element.s + element.x
                }
                RuleResult::Rejected => {
                    println!("Part {:?} has been rejected", element);

                    0
                }
                other => todo!("How did we get here: {:?}", other),
            };
        }
        sum.try_into().unwrap()
    }
}
impl DayPart2 for Day19 {
    fn run_part2(data: Self::Input) -> Self::Output {
        let data = Game::from_str_p1(data);

        let limit = 4000;
        let result = calculate_accepted_amount(limit, &data);
        result
    }

    fn get_test_result_part2() -> Self::Output {
        167409079868000
    }

    fn get_test_data_part2() -> Self::Input {
        Self::get_test_data()
    }
}

fn calculate_accepted_amount(limit: isize, data: &Game) -> usize {
    let result: usize = (1..=limit).par_bridge()
        // .into_par_iter()
        .map(|x| {
            (1..=limit).par_bridge()
                // .into_par_iter()
                .map(|m| {
                    (1..=limit).par_bridge()
                        // .into_par_iter()
                        .map(|a| {
                            (1..=limit).par_bridge()
                                // .into_par_iter()
                                .map(|s| {
                                    let element = Element { x, m, a, s };
                                    let result = data.get_result_p1(&element);
                                    match result {
                                        RuleResult::Accepted => 1,
                                        RuleResult::Rejected => 0,
                                        RuleResult::Workflow(_) => todo!(),
                                    }
                                })
                                .sum::<usize>()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    result
}

fn calculate_accepted_amount_v2(limit: isize, data: &Game) -> usize {
    // let total = limit * limit * limit*limit;
    let v = repeat_n(1..=limit, 4).multi_cartesian_product()
    // .collect::<Vec<_>>()
    ;
    // println!("limit: {limit}");
    // println!("should have {total} permutations");
    // println!("got {}", v.len());
    // dbg!(&v);
    // assert_eq!(total as usize, v.len());
    let result: usize = v.par_bridge()
    // .par_bridge()
        .map(|i| {
            let x = i[0];
            let m = i[1];
            let a = i [2];
            let s = i [3];
            // println!("{} {} {} {}", x, m, a, s);
            let element = Element { x, m, a, s };
            let result = data.get_result_p1(&element);
            match result {
                RuleResult::Accepted => 1,
                RuleResult::Rejected => 0,
                RuleResult::Workflow(_) => todo!(),
            }
        })
        .sum::<usize>();
    result
}
impl Game {
    fn from_str_p1(data: <Day19 as Day>::Input) -> Self {
        let mut data = data.lines().into_iter();
        let mut workflows = HashMap::new();
        for line in &mut data {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let workflow = Workflow::from_str_p1(line);
            workflows.insert(workflow.id.clone(), workflow);
        }
        let mut elements = vec![];
        for line in data {
            let element = Element::from_str_p1(line.trim());
            elements.push(element)
        }
        Self {
            elements,
            workflows,
        }
    }

    fn get_result_p1(&self, element: &Element) -> RuleResult {
        let mut current = self.workflows.get("in").unwrap();
        loop {
            'rules_loop: for rule in &current.rules {
                if let Some(x) = rule.matches_element(element) {
                    match x {
                        RuleResult::Accepted => return RuleResult::Accepted,
                        RuleResult::Rejected => return RuleResult::Rejected,
                        RuleResult::Workflow(workflow) => {
                            // println!("switching workflow from {} to {}", current.id, workflow);
                            // dbg!(&current);
                            // dbg!(&element);
                            current = self.workflows.get(&workflow).unwrap();
                            // dbg!(&current);
                            break 'rules_loop;
                        }
                    }
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct Game {
    pub elements: Vec<Element>,
    pub workflows: HashMap<String, Workflow>,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Workflow {
    pub rules: Vec<Rule>,
    pub id: String,
}
impl Workflow {
    pub fn from_str_p1(value: &str) -> Self {
        let (id, rules) = value.strip_suffix('}').unwrap().split_once('{').unwrap();
        let mut r = vec![];
        for rule in rules.split(',') {
            let rule = Rule::from_str_p1(rule);
            r.push(rule);
        }
        Self {
            id: id.to_string(),
            rules: r,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    Lt(Field, isize, RuleResult),
    Gt(Field, isize, RuleResult),
    Direct(RuleResult),
}
impl Rule {
    pub fn from_str_p1(value: &str) -> Self {
        let value = value.trim();
        let split = value.split_once(':');
        match split {
            Some((condition, rule_result)) => {
                let rule_result = RuleResult::from_str_p1(rule_result);
                match condition.split_once('<') {
                    Some((field, value)) => Rule::Lt(
                        Field::from_str_p1(field),
                        value.parse().unwrap(),
                        rule_result,
                    ),
                    None => {
                        let (field, value) = condition.split_once('>').unwrap();
                        Rule::Gt(
                            Field::from_str_p1(field),
                            value.parse().unwrap(),
                            rule_result,
                        )
                    }
                }
            }
            None => Rule::Direct(RuleResult::from_str_p1(value)),
        }
    }

    fn matches_element(&self, element: &Element) -> Option<RuleResult> {
        match self {
            Rule::Lt(field, value, result) => {
                let e_value = get_field_value(field, element);
                match e_value < *value {
                    true => Some(result.clone()),
                    false => None,
                }
            }
            Rule::Gt(field, value, result) => {
                let e_value = get_field_value(field, element);
                match e_value > *value {
                    true => Some(result.clone()),
                    false => None,
                }
            }
            Rule::Direct(result1) => Some(result1.clone()),
        }
    }
}

fn get_field_value(field: &Field, element: &Element) -> isize {
    let e_value = match field {
        Field::A => element.a,
        Field::M => element.m,
        Field::X => element.x,
        Field::S => element.s,
    };
    e_value
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Field {
    A,
    M,
    X,
    S,
}
impl Field {
    fn from_str_p1(field: &str) -> Field {
        match field {
            "a" => Field::A,
            "m" => Field::M,
            "x" => Field::X,
            "s" => Field::S,
            other => panic!("This value is not valid: {other}"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum RuleResult {
    Accepted,
    Rejected,
    Workflow(String),
}
impl RuleResult {
    fn from_str_p1(value: &str) -> RuleResult {
        match value {
            "A" => RuleResult::Accepted,
            "R" => RuleResult::Rejected,
            other => RuleResult::Workflow(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}
impl Element {
    pub fn from_str_p1(value: &str) -> Self {
        let value = value.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let mut d = HashMap::new();
        for v in value.split(',') {
            let (key, value) = v.split_once('=').unwrap();
            d.insert(key, value.parse().unwrap());
        }

        Self {
            x: *d.get("x").unwrap(),
            m: *d.get("m").unwrap(),
            a: *d.get("a").unwrap(),
            s: *d.get("s").unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn game() -> super::Game {
        let mut data = super::Game::from_str_p1(Day19::get_test_data());
        data.elements = vec![];

        data
    }

    #[rstest]
    #[case("{x=787,m=2655,a=1222,s=2876}", "A")]
    #[case("{x=1679,m=44,a=2067,s=496}", "R")]
    #[case("{x=2036,m=264,a=79,s=2244}", "A")]
    #[case("{x=2461,m=1339,a=466,s=291}", "R")]
    #[case("{x=2127,m=1623,a=2188,s=1013}", "A")]
    fn element(#[case] input: String, #[case] expected: String, game: super::Game) {
        let element = Element::from_str_p1(input.trim());
        let expected_result = RuleResult::from_str_p1(&expected);
        let actual_result = game.get_result_p1(&element);
        assert_eq!(actual_result, expected_result);
    }
    #[rstest]
    fn v1_v2(game: super::Game) {
        println!("Running v1");
        let result = calculate_accepted_amount(LIMIT, &game);
        println!("Running v2");
        let result2 = calculate_accepted_amount_v2(LIMIT, &game);
        println!("Got results.");
        dbg!(&result, &result2);
        assert_eq!(result, result2);
    }
    extern crate test;
    use test::Bencher;
    const LIMIT: isize = 25;
    #[bench]
    fn bench_mapping(b: &mut Bencher) {
        let game = game();
        b.iter(|| {
            let result = calculate_accepted_amount(LIMIT, &game);
            println!("{}", result);
        })
    }
    #[bench]
    fn bench_mapping_v2(b: &mut Bencher) {
        let game = game();
        b.iter(|| {
            let result = calculate_accepted_amount_v2(LIMIT, &game);
            println!("{}", result);
        })
    }
}
