use std::env;

fn get_max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

#[test]
fn test_get_max() {
    assert_eq!(get_max(1, 2), 2);
    assert_eq!(get_max(1, 0), 1);
}

fn get_data(cookie: String) -> Result<String, reqwest::Error> {
    let url = "https://adventofcode.com/2022/day/1/input".to_string();
    let client = reqwest::blocking::Client::new();

    let resp = client.get(url).header("cookie", cookie).send();
    resp?.text()
}

fn part1(lines: &Vec<&str>) -> u32 {
    let mut curr = 0;
    let mut max = 0;

    for &line in lines {
        if line == "" {
            max = get_max(max, curr);
            curr = 0;
        } else if let Ok(x) = String::from(line).parse::<u32>() {
            curr += x;
        } else {
            panic!("Invalid number")
        }
    }
    max
}

fn part2(lines: &[&str]) -> u32 {
    let mut curr = 0;
    let mut max = vec![0; 3];

    for &line in lines {
        if line == "" {
            max[0] = get_max(max[0], curr);
            max.sort();
            curr = 0;
        } else if let Ok(x) = String::from(line).parse::<u32>() {
            curr += x;
        } else {
            panic!("Invalid number")
        }
    }
    max.into_iter().sum()
}

struct ExpectedResult {
    part1: u32,
    part2: u32,
}

fn main() {
    let expected_result = ExpectedResult {
        part1: 71124,
        part2: 204639,
    };

    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() == 1);
    if args.len() != 1 {
        eprintln!("Usage: main <advent_of_code_cookie>");
        std::process::exit(1);
    }

    let cookie = format!("session={}", args[0]);

    let data = get_data(cookie);
    match data {
        Ok(value) => {
            let lines = value.split("\n").collect::<Vec<&str>>();

            assert_eq!(part1(&lines), expected_result.part1);
            assert_eq!(part2(&lines), expected_result.part2);

            println!(
                "{}",
                format!(
                    "Done! \n\tpart1: {}\n\tpart2: {}",
                    expected_result.part1, expected_result.part2
                )
            );
        }
        Err(e) => panic!("{}", format!("Invalid data {:?}", e)),
    }
}
