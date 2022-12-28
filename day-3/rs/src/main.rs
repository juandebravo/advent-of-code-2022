use reqwest::blocking::Client;
use std::env;

fn get_data(cookie: String) -> Result<String, reqwest::Error> {
    let url = "https://adventofcode.com/2022/day/3/input";
    let client = Client::new();

    let resp = client.get(url).header("cookie", cookie).send();
    resp?.text()
}

fn split_in_two(value: &str) -> (&str, &str) {
    assert!(value.len() % 2 == 0, "Invalid input");
    value.split_at(value.len() / 2)
}

#[test]
fn test_split_in_two() {
    assert_eq!(split_in_two("abcdef"), ("abc", "def"));
}

fn find_duplicates(item_a: &str, item_b: &str) -> Vec<char> {
    let vect_a = item_a.chars().collect::<Vec<char>>();
    let vect_b = item_b.chars().collect::<Vec<char>>();
    let mut values: Vec<char> = vec![];

    for ch in vect_a {
        if vect_b.contains(&ch) && !values.contains(&ch) {
            values.push(ch)
        }
    }
    values
}

#[test]
fn test_find_duplicates() {
    assert_eq!(find_duplicates("abc", "xyza"), vec!['a']);
    assert_eq!(find_duplicates("abc", "xyz"), vec![]);
    assert_eq!(find_duplicates("abc", "xyzab"), vec!['a', 'b']);
    assert_eq!(find_duplicates("xyzabc", "c"), vec!['c']);
}

fn find_duplicates_part_2(item_a: &str, item_b: &str, item_c: &str) -> Vec<char> {
    let first_duplicates = find_duplicates(item_a, item_b);
    let next_duplicates = find_duplicates(item_b, item_c);
    let last_duplicates = find_duplicates(item_a, item_c);
    let mut duplicates: Vec<char> = vec![];
    for ch in first_duplicates {
        if next_duplicates.contains(&ch) && last_duplicates.contains(&ch) {
            duplicates.push(ch);
        }
    }
    duplicates
}

#[test]
fn test_find_duplicates_part_2() {
    assert_eq!(find_duplicates_part_2("abc", "aio", "pla"), ['a']);
    assert_eq!(find_duplicates_part_2("5bc", "aio", "pla"), []);
    assert_eq!(find_duplicates_part_2("abc", "b8ca", "b8cr"), ['b', 'c']);
}

fn get_priority(a: char) -> u32 {
    let value = a as u32;
    if value < 97 {
        value - 38
    } else {
        value - 96
    }
}

#[test]
fn test_get_priority() {
    assert_eq!(get_priority('a'), 1);
    assert_eq!(get_priority('z'), 26);
    assert_eq!(get_priority('A'), 27);
    assert_eq!(get_priority('Z'), 52);
}

fn get_priorities(lines: &[&str]) -> u32 {
    let mut count = 0;

    for &line in lines {
        let parts = split_in_two(&line.trim());
        let chars = find_duplicates(parts.0, parts.1);
        for c in chars {
            count += get_priority(c);
        }
    }
    count
}

#[test]
fn test_get_priorities() {
    let value = "vJrwpWtwJgWrhcsFMMfFFhFp
  jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
  PmmdzqPrVvPwwTWBwg
  wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
  ttgJtRGJQctTZtZT
  CrZsJsPPZsGzwwsLwLmpwMDw";
    let lines = value.split("\n").collect::<Vec<&str>>();
    assert_eq!(get_priorities(&lines), 157);
}

fn get_priorities_part_2(lines: &[&str]) -> u32 {
    let chunks = lines.chunks(3);

    let mut count = 0;

    for chunk in chunks {
        if chunk.len() != 3 {
            continue;
        }
        let dup = find_duplicates_part_2(
            chunk.get(0).unwrap(),
            chunk.get(1).unwrap(),
            chunk.get(2).unwrap(),
        );
        for c in dup {
            count += get_priority(c);
        }
    }
    count
}

struct ExpectedResult {
    part1: u32,
    part2: u32,
}

fn main() {
    let expected_result = ExpectedResult {
        part1: 7878,
        part2: 2760,
    };
    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() == 1);
    let cookie = format!("session={}", args[0]);

    let data = get_data(cookie);

    match data {
        Ok(value) => {
            let lines = value.split("\n").collect::<Vec<&str>>();
            assert_eq!(get_priorities(&lines), expected_result.part1);
            assert_eq!(get_priorities_part_2(&lines), expected_result.part2);
        }
        Err(e) => panic!("{}", format!("Invalid data {:?}", e)),
    }
}
