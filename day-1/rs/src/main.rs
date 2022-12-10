use std::env;

fn main() {
    let url: &str = "https://adventofcode.com/2022/day/1/input";
    let mut args = env::args().skip(1);

    let cookie = format!("session={}", args.next().expect("Cookie not found"));

    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).header("cookie", cookie).send();

    let input = resp.unwrap().text().expect("Invalid text");
    let lines = input.split("\n");

    let mut curr = 0;
    let mut max = 0;

    for line in lines {
        if line == "" {
            if curr > max {
                max = curr
            }
            curr = 0;
            continue;
        }
        let line_num = String::from(line).parse::<i32>();
        match line_num {
            Ok(x) => {
                curr += x;
            }
            Err(_) => panic!("Invalid number"),
        }
    }
    println!("{}", max);
}
