// use serde::{Serialize, Deserialize};

mod structs;
mod config_utils;

use config_utils::get_config;
use structs::ApiResponse;
use colored::Colorize;
// use std::collections::HashMap;
use std::env;

const OPEN_WEATHER_URL: &str = "https://api.openweathermap.org/data/2.5";
const UNITS: &str = "metric";
const LANG: &str = "fr";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config_keys = [
        "OPEN_WEATHER".to_string(),
        "CITY".to_string()
    ];

    // Get config file content
    let config_keys = get_config(config_keys);

    let api_key = config_keys.get("OPEN_WEATHER")
        .expect("Get api key in config");

    // Gets city name from command line argument or from config file
    let city = if args.len() > 1 { args[1].to_string() } else {
        let city_from_config = config_keys.get("CITY")
        .expect("Should find a default city in config if none is passed as argument.");

        city_from_config.to_string()
    };

    println!("city {:?}", city);

    let url = format!(
        "{}/weather?q={}&appId={}&units={}&lang={}",
        OPEN_WEATHER_URL,
        city,
        api_key,
        UNITS,
        LANG
    );

    let body: structs::ApiResponse = reqwest::get(url).await?
        .json().await?;

    let res = print_weather(body);

    Ok(())
}

// fn extractResData (res: ApiResponse) {
//     let datas = HashMap::new();

//     datas.insert("City", res.name);
//     datas.insert("Weather", res.name);
//     datas.insert("City", res.name);
//     datas.insert("City", res.name);
//     datas.insert("City", res.name);
//     datas.insert("City", res.name);
// }

fn print_weather (current: ApiResponse) {
    println!(
        r#"
        |{}| {}
        |{}| {}
        |{}| {}
        |{}| lon {}, lat {}
        "#,
        green("City"),
        // format!("City").bold().green(),
        current.name,
        green("Weather"),
        &current.weather.first().unwrap().description,
        green("Temperature"),
        current.main.temp,
        green("Coordinates"),
        current.coord.lon,
        current.coord.lat,
    );

    fn green (name: &str) -> String {
        format!("{}", name).bold().green().to_string()
    }
}