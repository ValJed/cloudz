mod config_utils;
mod structs;

use chrono::{DateTime, NaiveDateTime, Utc};
use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;

use structs::{ApiCoordinates, ApiHourlyForecast, ApiResponse, Config};

const OW_URL: &str = "https://api.openweathermap.org/data/2.5";
const OW_GEOCODING_URL: &str = "https://api.openweathermap.org/geo/1.0/direct";
// const LANG: &str = "fr";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config: Config = confy::load("cloudz").expect("Error when trying to access config file");

    if config.ow_api_key.is_empty() {
        println!("You need to specify your open weather api key in your confg file");
        return Ok(());
    }

    if args.len() <= 1 && config.default_city.is_empty() {
        println!("You need to define a default city in your config file if you don't provide one");
        return Ok(());
    }

    let units_system: String = if !config.units_system.is_empty() {
        print!("heeere");
        config.units_system
    } else {

        println!("tooo");
        "metric".into()
    };

    // Gets city name from command line argument or from config file
    let city = if args.len() > 1 {
        args[1].to_string()
    } else {
        config.default_city
    };

    let coord_infos = match get_coordinates(&city, &config.ow_api_key).await {
        Some(infos) => infos,
        None => {
            println!("No coordinates found for {city}");
            return Ok(());
        }
    };

    let forecast = get_forecast(&config.ow_api_key, coord_infos, &units_system).await;

    let daily_forecasts = group_forecast_by_day(forecast);

    print_weather(daily_forecasts, &city, &units_system);

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

async fn get_forecast(api_key: &str, infos: ApiCoordinates, units_system: &String) -> ApiHourlyForecast {
    let lon = infos.lon.to_string();
    let lat = infos.lat.to_string();

    let url = format!("{OW_URL}/forecast?lat={lat}&lon={lon}&appid={api_key}&units={units_system}");

    let body: ApiHourlyForecast = reqwest::get(url)
        .await
        .expect("Error when getting hourly forecast")
        .json()
        .await
        .expect("Error when deserializing hourly forecast");

    body
}

async fn get_coordinates(city: &String, api_key: &String) -> Option<ApiCoordinates> {
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
    units_system: &String
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

        for (hour, weather) in forecast {
            let data = format_data(&hour, weather.to_owned(), &units_system);

            table.add_row(data);
        }

        println!("{table}");
    }
}

fn format_data(hour: &str, weather: ApiResponse, units_system: &String) -> Vec<Cell> {
    let temp_symbol = if units_system == "metric" {
        "°"
    } else {
        " °F"
    };
    let desc = &weather.weather[0].description;
    let temp = format!("{}{}",  weather.main.temp, temp_symbol);
    // let temp_min = weather.main.temp_min;
    // let temp_max = weather.main.temp_max;
    let feels = format!("{}{}", weather.main.feels_like, temp_symbol); 
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
