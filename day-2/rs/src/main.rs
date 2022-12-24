use std::env;

fn get_data(cookie: String) -> Result<String, reqwest::Error> {
    let url = "https://adventofcode.com/2022/day/2/input".to_string();
    let client = reqwest::blocking::Client::new();

    let resp = client.get(url).header("cookie", cookie).send();
    resp?.text()
}

fn same_value(they: char, me: char) -> bool {
    if they == 'A' && me == 'X' {
        return true;
    } else if they == 'B' && me == 'Y' {
        return true;
    } else if they == 'C' && me == 'Z' {
        return true;
    }
    false
}

fn a_beats_b(a: char, b: char) -> bool {
    if a == 'A' && b == 'Z' {
        return true;
    } else if a == 'B' && b == 'X' {
        return true;
    } else if a == 'C' && b == 'Y' {
        return true;
    }
    false
}

fn get_points_for_input(a: char) -> u32 {
    match a {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid char"),
    }
}

fn get_points(they: &char, me: &char) -> u32 {
    let mut outcome = 3;
    if !same_value(*they, *me) {
        if a_beats_b(*they, *me) {
            outcome = 0;
        } else {
            outcome = 6;
        }
    }
    let my_points = get_points_for_input(*me);
    outcome + my_points
}

fn get_first_char(value: &str) -> char {
    value.chars().next().unwrap()
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

        points += get_points(&char_they, &char_me);
    }
    points
}
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() == 1);
    let cookie = format!("session={}", args[0]);

    let data = get_data(cookie);

    match data {
        Ok(value) => {
            let lines = value.split("\n").collect::<Vec<&str>>();
            println!("{}", part1(&lines));
        }
        Err(e) => panic!("{}", format!("Invalid data {:?}", e)),
    }
}
