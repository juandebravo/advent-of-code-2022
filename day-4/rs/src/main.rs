use reqwest::blocking::Client;
use std::env;
use std::str::FromStr;

fn get_data(cookie: String) -> Result<String, reqwest::Error> {
    let url = "https://adventofcode.com/2022/day/4/input";
    let client = Client::new();

    let resp = client.get(url).header("cookie", cookie).send();
    resp?.text()
}

fn check_overlap(a: Vec<u8>, b: Vec<u8>, full: bool) -> bool {
    if a.len() == 0 || b.len() == 0 {
        return false;
    }

    let a_ = if a.len() > b.len() { &b } else { &a };
    let b_ = if a.len() > b.len() { &a } else { &b };

    for x in a_ {
        if !full && b_.contains(&x) {
            // if we only require partial overlap and x is contained,
            // return true
            return true;
        } else if full && !b_.contains(&x) {
            // if we require full overlap and one is missing, return false
            return false;
        }
    }
    full
}

#[test]
fn test_check_overlap() {
    assert_eq!(check_overlap(vec![1, 2, 3], vec![4, 5, 6], true), false);
    assert_eq!(check_overlap(vec![1, 2, 3], vec![1], true), true);
    assert_eq!(check_overlap(vec![1, 2, 3], vec![1, 2, 3], true), true);
    assert_eq!(
        check_overlap(vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3], true),
        true
    );
    assert_eq!(check_overlap(vec![1], vec![], true), false);

    assert_eq!(check_overlap(vec![1, 2, 3, 4], vec![4, 5, 6], false), true);
    assert_eq!(check_overlap(vec![1, 8], vec![1, 2, 3], false), true);
}

fn vector_from_input(input: &str) -> Vec<u8> {
    let mut value: Vec<u8> = vec![];

    match input.find('-') {
        Some(index) => {
            match (
                u8::from_str(&input[..index]),
                u8::from_str(&input[index + 1..]),
            ) {
                (Ok(l), Ok(r)) => {
                    for x in l..=r {
                        value.push(x);
                    }
                }
                _ => panic!("invalid value"),
            }
        }
        None => panic!("invalid input"),
    }
    value
}

#[test]
fn test_vector_from_input() {
    assert_eq!(vector_from_input("3-6"), vec![3, 4, 5, 6]);
    assert_eq!(vector_from_input("3-7"), vec![3, 4, 5, 6, 7]);
    assert_eq!(vector_from_input("3-3"), vec![3]);
}

fn check_line(input: &str, full: bool) -> bool {
    match input.find(',') {
        Some(index) => {
            let a = &input[..index];
            let b = &input[index + 1..];
            check_overlap(vector_from_input(a), vector_from_input(&b), full)
        }
        None => panic!("Invalid input"),
    }
}

#[test]
fn test_check_line() {
    assert_eq!(check_line("48-50,48-49", true), true);
    assert_eq!(check_line("5-89,5-5", true), true);
    assert_eq!(check_line("17-57,55-96", true), false);
}

fn part(lines: &[&str], full: bool) -> u32 {
    let mut counter = 0;
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        }
        if check_line(&line.trim(), full) {
            counter += 1;
        }
    }
    counter
}

fn part1(lines: &[&str]) -> u32 {
    part(&lines, true)
}

fn part2(lines: &[&str]) -> u32 {
    part(&lines, false)
}

struct ExpectedResult {
    part1: u32,
    part2: u32,
}

static EXPECTED_RESULT: ExpectedResult = ExpectedResult {
    part1: 530,
    part2: 903,
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() == 1);
    let cookie = format!("session={}", args[0]);

    let data = get_data(cookie);

    match data {
        Ok(value) => {
            let lines = value.split("\n").collect::<Vec<&str>>();
            assert_eq!(part1(&lines), EXPECTED_RESULT.part1);
            assert_eq!(part2(&lines), EXPECTED_RESULT.part2);
        }
        Err(e) => panic!("{}", format!("Invalid data {:?}", e)),
    }
}
