use std::{env, str::Split};

fn get_max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

#[test]
fn test_get_max() {
    assert_eq!(get_max(1, 2), 2);
    assert_eq!(get_max(1, -2), 1);
}

fn get_data(cookie: String) -> Result<String, reqwest::Error> {
    let url = "https://adventofcode.com/2022/day/1/input";
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).header("cookie", cookie).send();
    resp?.text()
}

fn part1(lines: Split<&str>) -> i32 {
    let mut curr = 0;
    let mut max = 0;

    for line in lines {
        if line == "" {
            max = get_max(max, curr);
            curr = 0;
        } else {
            let line_num = String::from(line).parse::<i32>();
            match line_num {
                Ok(x) => {
                    curr += x;
                }
                Err(_) => panic!("Invalid number"),
            }
        }
    }
    max
}

fn main() {
    assert!(env::args().len() > 1);
    let mut args = env::args().skip(1);

    let cookie = format!(
        "session={}",
        args.next().expect("Cookie argument not found")
    );
    let data = get_data(cookie);
    match data {
        Ok(value) => {
            let lines = value.split("\n");
            println!("{}", part1(lines))
        }
        Err(e) => panic!("{}", format!("Invalid data {:?}", e)),
    }
}
