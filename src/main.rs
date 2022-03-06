// use serde::{Serialize, Deserialize};

mod structs;
mod config_utils;

use config_utils::get_config;
use structs::ApiResponse;
use colored::Colorize;
use std::collections::HashMap;
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

    let data_to_print = extract_data(body);

    println!("data&_to_print {:?}", data_to_print);

    print_weather(data_to_print);

    Ok(())
}

fn extract_data (data: ApiResponse) -> HashMap<String, String> {
    let mut datas = HashMap::new();

    datas.insert("City".into(), data.name);
    datas.insert("Weather".into(), data.weather.first().unwrap().description.to_string());
    datas.insert("Temperature".into(), data.main.temp.to_string());
    datas.insert("Coordinates".into(), format!("lon {}, lat {}", data.coord.lon, data.coord.lat));

    datas
}

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