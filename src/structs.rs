use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct ApiResponse {
  pub coord: Coord,
  pub weather: Vec<Weather>,
  pub base: String,
  pub main: Main,
  pub visibility: i64,
  pub wind: Wind,
  pub clouds: Clouds,
  pub dt: i64,
  pub sys: Sys,
  pub timezone: i64,
  pub id: i64,
  pub name: String,
  pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct Coord {
  pub lon: f64,
  pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct Weather {
  pub id: i64,
  pub main: String,
  pub description: String,
  pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct Main {
  pub temp: f64,
  pub feels_like: f64,
  pub temp_min: f64,
  pub temp_max: f64,
  pub pressure: i64,
  pub humidity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct Wind {
  pub speed: f64,
  pub deg: i64,
  pub gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct Clouds {
  pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct Sys {
  #[serde(rename = "type")]
  pub type_field: i64,
  pub id: i64,
  pub country: String,
  pub sunrise: i64,
  pub sunset: i64,
}
