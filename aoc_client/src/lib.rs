use reqwest::{blocking, Error};

pub fn get_data(cookie: String, day: u8) -> Result<String, Error> {
    let client = blocking::Client::new();

    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let resp = client.get(&url).header("cookie", cookie).send();
    resp?.text()
}
