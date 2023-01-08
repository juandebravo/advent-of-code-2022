use aoc_client;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

type Table<'a> = HashMap<u8, Vec<&'a str>>;

#[derive(Debug, PartialEq)]
struct Input {
    quantity: u8,
    from: u8,
    to: u8,
}

fn parse_line(line: &str) -> Input {
    let re = Regex::new(r"^[^\d]*\b(\d*)\b[^\d]*\b(\d*)\b[^\d]*\b(\d*)").unwrap();

    fn unwrap_number(capture: Option<regex::Match>) -> u8 {
        match capture.map(|x| u8::from_str(x.as_str())) {
            Some(Ok(x)) => x,
            _ => panic!("Invalid number"),
        }
    }

    match re.captures(line) {
        Some(caps) => Input {
            quantity: unwrap_number(caps.get(1)),
            from: unwrap_number(caps.get(2)),
            to: unwrap_number(caps.get(3)),
        },
        _ => panic!("Invalid line"),
    }
}

#[test]
fn test_parse_line() {
    assert!(
        parse_line("move 3 from 5 to 2")
            == Input {
                quantity: 3,
                from: 5,
                to: 2,
            }
    );

    assert!(
        parse_line("move 10 from 1 to 4")
            == Input {
                quantity: 10,
                from: 1,
                to: 4,
            }
    );
}

fn handle_input(input: &Input, table: &mut Table, reverse: bool) -> () {
    let mut values: Vec<&str> = Vec::new();
    match table.get_mut(&input.from) {
        Some(from) => {
            for _ in 1..=input.quantity {
                match from.pop() {
                    Some(x) => {
                        values.push(&x);
                    }
                    _ => panic!("Invalid value"),
                }
            }
        }
        _ => panic!("invalid value"),
    }

    if reverse {
        values.reverse();
    }

    match table.get_mut(&input.to) {
        Some(to) => {
            for el in values {
                to.push(&el);
            }
        }
        _ => panic!("Invalid value"),
    }
}

#[test]
fn test_handle_input() {
    let mut table = Table::new();
    table.insert(1, vec!["a"]);
    table.insert(2, vec!["b", "b2"]);

    let input = Input {
        quantity: 2,
        from: 2,
        to: 1,
    };
    handle_input(&input, &mut table, false);

    assert_eq!(*table.get(&1).unwrap(), ["a", "b2", "b"]);
}

fn build_initial_table() -> Table<'static> {
    let mut table = Table::new();
    table.insert(1, vec!["B", "W", "N"]);
    table.insert(2, vec!["L", "Z", "S", "P", "T", "D", "M", "B"]);
    table.insert(3, vec!["Q", "H", "Z", "W", "R"]);
    table.insert(4, vec!["W", "D", "V", "J", "Z", "R"]);
    table.insert(5, vec!["S", "H", "M", "B"]);
    table.insert(6, vec!["L", "G", "N", "J", "H", "V", "P", "B"]);
    table.insert(7, vec!["J", "Q", "Z", "F", "H", "D", "L", "S"]);
    table.insert(8, vec!["W", "S", "F", "J", "G", "Q", "B"]);
    table.insert(9, vec!["Z", "W", "M", "S", "C", "D", "J"]);
    table
}

fn execute_with_reverse(lines: &[&str], reverse: bool) -> String {
    let mut table = build_initial_table();
    for line in lines {
        if line.starts_with("move") {
            let input = parse_line(line);
            handle_input(&input, &mut table, reverse);
        }
    }
    let mut val = String::new();
    for i in 1..=9 {
        let value = table.get(&i).unwrap();
        val.push_str(value.get(value.len() - 1).unwrap());
    }
    val
}

fn part1(lines: &[&str]) -> String {
    execute_with_reverse(lines, false)
}

fn part2(lines: &[&str]) -> String {
    execute_with_reverse(lines, true)
}

struct ExpectedResult<'a> {
    part1: &'a str,
    part2: &'a str,
}

static EXPECTED_RESULT: ExpectedResult = ExpectedResult {
    part1: "MQSHJMWNH",
    part2: "LLWJRBHVZ",
};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    assert!(args.len() == 1);

    let cookie = format!("session={}", args[0]);
    let data = aoc_client::get_data(cookie, 5);

    match data {
        Ok(value) => {
            let lines = value.split("\n").collect::<Vec<&str>>();
            assert_eq!(part1(&lines), EXPECTED_RESULT.part1);
            assert_eq!(part2(&lines), EXPECTED_RESULT.part2)
        }
        Err(e) => panic!("{}", format!("Invalid data {:?}", e)),
    }
}
