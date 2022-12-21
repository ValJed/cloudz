use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub default_city: String,
    pub ow_api_key: String,
    pub units_system: String,
    pub lang: String
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            default_city: "".into(),
            ow_api_key: "".into(),
            units_system: "".into(),
            lang: "".into()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse {
    // pub coord: Coord,
    pub weather: Vec<Weather>,
    // pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub dt_txt: String,
    // pub sys: Sys,
    // pub timezone: i64,
    // pub id: i64,
    // pub name: String,
    // pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiCoordinates {
    pub name: String,
    pub country: String,
    // #[serde(default = "unknown")]
    // pub state: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiHourlyForecast {
    pub list: Vec<ApiResponse>,
    pub city: City,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct City {
    pub id: i64,
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct ApiCoordinates {
//     pub coord: Coord,
//     pub weather: Vec<Weather>,
//     pub base: String,
//     pub main: Main,
//     pub visibility: i64,
//     pub wind: Wind,
//     pub clouds: Clouds,
//     pub dt: i64,
//     pub sys: Sys,
//     pub timezone: i64,
//     pub id: i64,
//     pub name: String,
//     pub cod: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct DataToPrint {
//     pub city: String,
//     pub weather: String,
//     pub temperature: String,
//     pub coordinates: String,
// }

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
    // pub gust: f64,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForecastSys {
    pod: String,
}
