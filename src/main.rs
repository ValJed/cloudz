// use serde::{Serialize, Deserialize};

mod config_utils;
mod structs;

use chrono::{DateTime, NaiveDateTime, Utc};
use colored::Colorize;
use comfy_table::Table;
use config_utils::get_config;
use regex::Regex;
use std::collections::HashMap;
use std::env;
// use structs::DataToPrint;

use structs::{ApiCoordinates, ApiHourlyForecast, ApiResponse};

const OW_URL: &str = "https://api.openweathermap.org/data/2.5";
const OW_GEOCODING_URL: &str = "https://api.openweathermap.org/geo/1.0/direct";
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
        .expect("Error when getting api key in config");

    // Gets city name from command line argument or from config file
    let city = if args.len() > 1 {
        args[1].to_string()
    } else {
        let city_from_config = config_keys
            .get("CITY")
            .expect("Should find a default city in config if none is passed as argument.");

        city_from_config.to_string()
    };

    let coord_infos = match get_coordinates(&city, (&api_key).to_string()).await {
        Some(infos) => infos,
        None => {
            println!("No coordinates found for {city}");
            return Ok(());
        }
    };

    let forecast = get_forecast(&api_key, coord_infos).await;

    // for time in forecast.list {
    //     let list_time = format_date(&time.dt);
    //
    // }

    let formatted: HashMap<String, String> =
        forecast
            .list
            .iter()
            .fold(HashMap::new(), |acc, hourly_wheather| {
                let [day, hour] = format_date(&hourly_wheather.dt);

                println!("{:?}", day);
                println!("{}", hour);
                println!("{:?}", hourly_wheather);
                // println!(time[0]);
                acc
            });

    // let url = format!(
    //     "{}/weather?q={}&appId={}&units={}&lang={}",

    //     OW_URL, city, api_key, UNITS, LANG
    // );

    // let body: ApiResponse = reqwest::get(url).await?.json().await?;

    // let data_to_print = extract_data(body);

    // print_weather(data_to_print);

    Ok(())
}

fn format_date(date: &i64) -> [String; 2] {
    let dt = NaiveDateTime::from_timestamp(*date, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(dt, Utc);
    let regex = Regex::new(r"\s+").unwrap();
    // let day = t.day()
    let time = datetime.format("%H:%M").to_string();
    let _day = datetime.format("%a %b %e").to_string();

    let day = regex.replace_all(&*_day, " ").to_string();

    [day, time]
}

async fn get_forecast(api_key: &str, infos: ApiCoordinates) -> ApiHourlyForecast {
    let lon = infos.lon.to_string();
    let lat = infos.lat.to_string();

    let url = format!("{OW_URL}/forecast?lat={lat}&lon={lon}&appid={api_key}&units={UNITS}");

    let body: ApiHourlyForecast = reqwest::get(url)
        .await
        .expect("Error when getting hourly forecat")
        .json()
        .await
        .expect("Error when deserializing hourly forecast");

    body
}

async fn get_coordinates(city: &String, api_key: String) -> Option<ApiCoordinates> {
    let url = format!("{}?q={}&appid={}", OW_GEOCODING_URL, city, api_key);

    let body: Vec<ApiCoordinates> = reqwest::get(url)
        .await
        .expect("Error when getting coordinates")
        .json()
        .await
        .expect("Error when deserializing coordinates");

    body.into_iter().nth(0)
}

// fn extract_data(data: ApiResponse) -> [String; 4] {
//     return [
//         format!(
//             "{} | {}\n",
//             color(data.name, Color::title),
//             color(data.sys.country, Color::title)
//         ),
//         format!(
//             "{}°, {}",
//             color(data.main.temp.to_string(), Color::degrees),
//             data.weather.first().unwrap().description.to_string()
//         ),
//         format!(
//             "min: {}°",
//             color(data.main.temp_min.to_string(), Color::degrees)
//         ),
//         format!(
//             "max: {}°",
//             color(data.main.temp_max.to_string(), Color::degrees)
//         ),
//     ];
// }

fn print_weather(data: [String; 4]) {
    let mut table = Table::new();

    for (i, line) in data.iter().enumerate() {
        if i == 0 {
            table.set_header(vec![line]);
        } else {
            table.add_row(vec![line]);
        }
    }

    println!("{table}");
    // let to_print: String = data.iter().fold("".to_string(), |acc, text| {
    //     return format!("{}  {}\n", acc, text);
    // });

    // println!("{}", to_print);
}

fn color(text: String, color: Color) -> String {
    match color {
        Color::title => format!("{}", text).bold().red().to_string(),
        Color::degrees => format!("{}", text).purple().to_string(),
    }
}
