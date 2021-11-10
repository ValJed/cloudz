use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 pub struct ApiResponse {
  coord: Coord,
  weather: Vec<Weather>,
  base: String,
  main: Main,
  visibility: i64,
  wind: Wind,
  clouds: Clouds,
  dt: i64,
  sys: Sys,
  timezone: i64,
  id: i64,
  name: String,
  cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Coord {
  lon: f64,
  lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Weather {
  id: i64,
  main: String,
  description: String,
  icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Main {
  temp: f64,
  feels_like: f64,
  temp_min: f64,
  temp_max: f64,
  pressure: i64,
  humidity: i64,
  sea_level: i64,
  grnd_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Wind {
  speed: f64,
  deg: i64,
  gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Clouds {
  all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Sys {
  #[serde(rename = "type")]
  type_field: i64,
  id: i64,
  country: String,
  sunrise: i64,
  sunset: i64,
}
