// use serde::{Serialize, Deserialize};

mod config_utils;
mod structs;

use colored::Colorize;
use config_utils::get_config;
use std::collections::HashMap;
use std::env;
use structs::ApiResponse;
use structs::DataToPrint;

const OPEN_WEATHER_URL: &str = "https://api.openweathermap.org/data/2.5";
const UNITS: &str = "metric";
const LANG: &str = "fr";

enum Color {
    title,
    degrees,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config_keys = ["OPEN_WEATHER".to_string(), "CITY".to_string()];

    // Get config file content
    let config_keys = get_config(config_keys);

    let api_key = config_keys
        .get("OPEN_WEATHER")
        .expect("Get api key in config");

    // Gets city name from command line argument or from config file
    let city = if args.len() > 1 {
        args[1].to_string()
    } else {
        let city_from_config = config_keys
            .get("CITY")
            .expect("Should find a default city in config if none is passed as argument.");

        city_from_config.to_string()
    };

    let url = format!(
        "{}/weather?q={}&appId={}&units={}&lang={}",
        OPEN_WEATHER_URL, city, api_key, UNITS, LANG
    );

    let body: ApiResponse = reqwest::get(url).await?.json().await?;

    let data_to_print = extract_data(body);

    print_weather(data_to_print);

    Ok(())
}

fn extract_data(data: ApiResponse) -> [String; 4] {
    return [
        format!(
            "{} | {}\n",
            color(data.name, Color::title),
            color(data.sys.country, Color::title)
        ),
        format!(
            "{}°, {}",
            color(data.main.temp.to_string(), Color::degrees),
            data.weather.first().unwrap().description.to_string()
        ),
        format!(
            "min: {}°",
            color(data.main.temp_min.to_string(), Color::degrees)
        ),
        format!(
            "max: {}°",
            color(data.main.temp_max.to_string(), Color::degrees)
        ),
    ];
}

fn print_weather(data: [String; 4]) {
    let to_print: String = data.iter().fold("".to_string(), |acc, text| {
        return format!("{}  {}\n", acc, text);
    });

    println!("{}", to_print);
}

fn color(text: String, color: Color) -> String {
    match color {
        Color::title => format!("{}", text).bold().red().to_string(),
        Color::degrees => format!("{}", text).purple().to_string(),
    }
}
