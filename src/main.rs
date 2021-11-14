use std::{fs};
use std::io::{BufReader, BufRead};
use fs::{File};
use regex::Regex;
// use serde::{Serialize, Deserialize};

mod structs;

const OPEN_WEATHER_URL: &str = "https://api.openweathermap.org/data/2.5";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get env file content
    let api_key = read_env_file("OPEN_WEATHER".to_string());

    let city = "lyon".to_string();

    let url = format!("{}/weather?q={}&appId={}", OPEN_WEATHER_URL, city, api_key);

    let body: structs::ApiResponse = reqwest::get(url).await?
        .json().await?;
        // .json::<HashMap<String, String>>().await?;
        // .json::<HashMap<String, String>>()
        // .await?;

        println!("body {:?}", body);

    Ok(())
}


fn read_env_file (env_key: String) -> String {
    let env_file = File::open(".env")
        .expect("Error when opening .env file");

    let reader = BufReader::new(env_file);
    let formatted_regex = format!(r"^{}", env_key);
    let regex = Regex::new(&formatted_regex)
        .expect("Regex could not be generated");

    let lines = reader
        .lines()
        .map(|l| l.unwrap());

    lines
        // .find(|line| regex.is_match(&line)).unwrap();
        .fold(String::new(), |acc, line| {
            if regex.is_match(&line) {
                let splitted: Vec<&str> = line.split('=').collect();

                return splitted[1].to_string();
            }

            acc
        })
}