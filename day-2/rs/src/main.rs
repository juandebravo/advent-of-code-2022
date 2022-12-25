use reqwest::blocking::Client;
use std::env;

fn get_data(cookie: String) -> Result<String, reqwest::Error> {
    let url = "https://adventofcode.com/2022/day/2/input";
    let client = Client::new();

    let resp = client.get(url).header("cookie", cookie).send();
    resp?.text()
}

fn get_first_char(value: &str) -> char {
    value.chars().next().unwrap()
}

fn same_value(they: char, me: char) -> bool {
    match (they, me) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => true,
        _ => false,
    }
}

fn a_beats_b(a: char, b: char) -> bool {
    match (a, b) {
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => true,
        _ => false,
    }
}

fn get_points_for_input(a: &char) -> u32 {
    match a {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid char"),
    }
}

fn get_points_part_1(they: &char, me: &char) -> u32 {
    let mut outcome = 3;
    if !same_value(*they, *me) {
        if a_beats_b(*they, *me) {
            outcome = 0;
        } else {
            outcome = 6;
        }
    }
    let my_points = get_points_for_input(me);
    outcome + my_points
}

fn part1(lines: &[&str]) -> u32 {
    let valid_chars: [char; 6] = ['A', 'B', 'C', 'X', 'Y', 'Z'];
    let mut points = 0;

    for &line in lines {
        if line.trim().len() == 0 {
            continue;
        }
        let l_vector = line.split_whitespace().collect::<Vec<&str>>();
        let (they, me) = (l_vector[0], l_vector[1]);

        let char_they = get_first_char(&they);
        let char_me = get_first_char(&me);

        assert!(valid_chars.contains(&char_they));
        assert!(valid_chars.contains(&char_me));

        points += get_points_part_1(&char_they, &char_me);
    }
    points
}

fn get_points_part_2(they: &char, me: &char) -> u32 {
    let outcome;
    let my_shape;
    match me {
        'X' => {
            outcome = 0;
            my_shape = match they {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => panic!("Invalid value"),
            }
        }
        'Y' => {
            outcome = 3;
            my_shape = match they {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => panic!("Invalid value"),
            }
        }
        'Z' => {
            outcome = 6;
            my_shape = match they {
                'A' => 2,
                'B' => 3,
                'C' => 1,
                _ => panic!("Invalid value"),
            }
        }
        _ => panic!("Invalid value"),
    }
    outcome + my_shape
}

fn part2(lines: &[&str]) -> u32 {
    let valid_chars: [char; 6] = ['A', 'B', 'C', 'X', 'Y', 'Z'];
    let mut points = 0;

    for &line in lines {
        if line.trim().len() == 0 {
            continue;
        }
        let l_vector = line.split_whitespace().collect::<Vec<&str>>();
        let (they, me) = (l_vector[0], l_vector[1]);

        let char_they = get_first_char(&they);
        let char_me = get_first_char(&me);

        assert!(valid_chars.contains(&char_they));
        assert!(valid_chars.contains(&char_me));

        points += get_points_part_2(&char_they, &char_me);
    }
    points
}

struct ExpectedResult {
    part1: u32,
    part2: u32,
}

fn main() {
    let expected_result = ExpectedResult {
        part1: 11873,
        part2: 12014,
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
