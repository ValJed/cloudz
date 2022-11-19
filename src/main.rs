mod config_utils;
mod structs;

use chrono::{DateTime, NaiveDateTime, Utc};
use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use config_utils::get_config;
use regex::Regex;
use std::collections::HashMap;
use std::env;

use structs::{ApiCoordinates, ApiHourlyForecast, ApiResponse};

const OW_URL: &str = "https://api.openweathermap.org/data/2.5";
const OW_GEOCODING_URL: &str = "https://api.openweathermap.org/geo/1.0/direct";
const UNITS: &str = "metric";
// const LANG: &str = "fr";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config_keys = ["OPEN_WEATHER".to_string(), "CITY".to_string()];

    // Get config file content
    let config_keys = get_config(config_keys);

    let api_key = config_keys
        .get("OPEN_WEATHER")
        .expect("Error when getting api key from config");

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

    let daily_forecasts = group_forecast_by_day(forecast);

    print_weather(daily_forecasts, &city);

    Ok(())
}

fn group_forecast_by_day(
    forecast: ApiHourlyForecast,
) -> (Vec<String>, HashMap<String, Vec<(String, ApiResponse)>>) {
    let mut days_order = vec![];

    let grouped: HashMap<String, Vec<(String, ApiResponse)>> =
        forecast
            .list
            .iter()
            .fold(HashMap::new(), |mut acc, hourly_wheather| {
                let [day, hour] = format_date(&hourly_wheather.dt);

                match acc.get(&day) {
                    Some(val) => {
                        let mut clone = val.to_owned();

                        let tup = (hour, hourly_wheather.to_owned());
                        clone.push(tup);

                        acc.insert(day, clone);
                    }
                    None => {
                        days_order.push(day.clone());
                        acc.insert(day, vec![(hour, hourly_wheather.to_owned())]);
                    }
                }

                acc
            });

    (days_order, grouped)
}

fn format_date(date: &i64) -> [String; 2] {
    let dt = NaiveDateTime::from_timestamp(*date, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(dt, Utc);
    let regex = Regex::new(r"\s+").unwrap();
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
        .expect("Error when getting hourly forecast")
        .json()
        .await
        .expect("Error when deserializing hourly forecast");

    body
}

async fn get_coordinates(city: &String, api_key: String) -> Option<ApiCoordinates> {
    let url = format!("{OW_GEOCODING_URL}?q={city}&appid={api_key}");

    let body: Vec<ApiCoordinates> = reqwest::get(url)
        .await
        .expect("Error when getting coordinates")
        .json()
        .await
        .expect("Error when deserializing coordinates");

    body.into_iter().nth(0)
}

fn print_weather(
    (days, daily_forecasts): (Vec<String>, HashMap<String, Vec<(String, ApiResponse)>>),
    city: &String,
) {
    println!(
        "{}",
        format!("{} {}", "Meteo forecast", city)
            .bold()
            .truecolor(201, 40, 45)
            .to_string()
    );

    for day in days {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new(&day)
                    .fg(Color::Red)
                    .add_attribute(Attribute::Bold),
                Cell::new("Weather")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new("Temperature")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new("Feeling")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new("Pressure")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new("Humididy")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new("Wind")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
            ]);

        let forecast = daily_forecasts.get(&day).unwrap();

        let mut count = 0;
        for (hour, weather) in forecast {
            count = count + 1;
            let data = format_data(&hour, weather.to_owned());

            table.add_row(data);
        }

        println!("{table}");
    }
}

fn format_data(hour: &str, weather: ApiResponse) -> Vec<Cell> {
    let desc = &weather.weather[0].description;
    let temp = weather.main.temp;
    // let temp_min = weather.main.temp_min;
    // let temp_max = weather.main.temp_max;
    let feels = weather.main.feels_like;
    let pressure = weather.main.pressure;
    let humidity = weather.main.humidity;
    let wind = (weather.wind.speed * 3.6 * 100.0).round() / 100.0;

    vec![
        Cell::new(hour).add_attribute(Attribute::Bold),
        Cell::new(desc),
        Cell::new(temp),
        Cell::new(feels),
        Cell::new(pressure),
        Cell::new(humidity),
        Cell::new(wind),
    ]
}
