use reqwest::blocking::Client;
use std::env;
use std::str::FromStr;

fn get_data(cookie: String) -> Result<String, reqwest::Error> {
    let url = "https://adventofcode.com/2022/day/4/input";
    let client = Client::new();

    let resp = client.get(url).header("cookie", cookie).send();
    resp?.text()
}

fn check_full_overlap(a: Vec<u8>, b: Vec<u8>) -> bool {
    if a.len() == 0 || b.len() == 0 {
        return false;
    }

    let a_ = if a.len() > b.len() { &b } else { &a };
    let b_ = if a.len() > b.len() { &a } else { &b };

    for x in a_ {
        if !b_.contains(&x) {
            return false;
        }
    }
    true
}

#[test]
fn test_check_full_overlap() {
    assert_eq!(check_full_overlap(vec![1, 2, 3], vec![4, 5, 6]), false);
    assert_eq!(check_full_overlap(vec![1, 2, 3], vec![1]), true);
    assert_eq!(check_full_overlap(vec![1, 2, 3], vec![1, 2, 3]), true);
    assert_eq!(
        check_full_overlap(vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3]),
        true
    );
    assert_eq!(check_full_overlap(vec![1], vec![]), false);
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
                    for x in l..r + 1 {
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

fn check_line(input: &str) -> bool {
    let (a, b) = input.split_at(input.find(',').unwrap());
    check_full_overlap(vector_from_input(a), vector_from_input(&b[1..]))
}

#[test]
fn test_check_line() {
    assert_eq!(check_line("48-50,48-49"), true);
    assert_eq!(check_line("5-89,5-5"), true);
    assert_eq!(check_line("17-57,55-96"), false);
}

fn part1(lines: &[&str]) -> u32 {
    let mut counter = 0;
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        }
        if check_line(&line.trim()) {
            counter += 1;
        }
    }
    counter
}

fn check_partial_overlap(a: Vec<u8>, b: Vec<u8>) -> bool {
    if a.len() == 0 || b.len() == 0 {
        return false;
    }

    let a_ = if a.len() > b.len() { &b } else { &a };
    let b_ = if a.len() > b.len() { &a } else { &b };

    for x in a_ {
        if b_.contains(&x) {
            return true;
        }
    }
    false
}

fn check_line_2(input: &str) -> bool {
    let (a, b) = input.split_at(input.find(',').unwrap());
    check_partial_overlap(vector_from_input(a), vector_from_input(&b[1..]))
}

fn part2(lines: &[&str]) -> u32 {
    let mut counter = 0;
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        }
        if check_line_2(&line.trim()) {
            counter += 1;
        }
    }
    counter
}

struct ExpectedResult {
    part1: u32,
    part2: u32,
}

fn main() {
    let expected_result = ExpectedResult {
        part1: 530,
        part2: 903,
    };

    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() == 1);
    let cookie = format!("session={}", args[0]);

    let data = get_data(cookie);

    match data {
        Ok(value) => {
            let lines = value.split("\n").collect::<Vec<&str>>();
            assert_eq!(part1(&lines), expected_result.part1);
            assert_eq!(part2(&lines), expected_result.part2);
        }
        Err(e) => panic!("{}", format!("Invalid data {:?}", e)),
    }
}
