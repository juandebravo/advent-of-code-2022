use std::collections::HashSet;
use std::env;

fn is_unique(s: &str) -> bool {
    let mut set = HashSet::new();
    for c in s.chars() {
        if set.contains(&c) {
            return false;
        }
        set.insert(c);
    }
    true
}

fn parse_input(input: &str, num_characters: usize) -> Option<usize> {
    for i in 0..input.len() - num_characters + 1 {
        if is_unique(&input[i..i + num_characters]) {
            return Some(i + num_characters);
        }
    }
    None
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("aabbcdefg", 3), Some(6));
    assert_eq!(parse_input("aabbc", 3), None);
    assert_eq!(parse_input("aabbc", 1), Some(1));
    assert_eq!(parse_input("abbc", 2), Some(2));
    assert_eq!(parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
    assert_eq!(parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
    assert_eq!(
        parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
        Some(26)
    );
    assert_eq!(
        parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
        Some(29)
    );
}

struct ExpectedResult {
    part1: u32,
    part2: u32,
}

fn main() {
    let expected_result = ExpectedResult {
        part1: 1155,
        part2: 2789,
    };

    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() == 1);
    if args.len() != 1 {
        eprintln!("Usage: main <advent_of_code_cookie>");
        std::process::exit(1);
    }

    let cookie = format!("session={}", args[0]);

    let data = aoc_client::get_data(cookie, 6);
    match data {
        Ok(value) => {
            let lines = value.split("\n").collect::<Vec<&str>>();
            for line in lines {
                if line != "" {
                    // part1
                    match parse_input(line, 4) {
                        Some(value) => assert_eq!(value as u32, expected_result.part1),
                        None => panic!("invalid value"),
                    }

                    // part2
                    match parse_input(line, 14) {
                        Some(value) => assert_eq!(value as u32, expected_result.part2),
                        None => panic!("invalid value"),
                    }
                }
            }
        }
        Err(e) => panic!("{}", format!("Invalid data {:?}", e)),
    }
}
