# Cloudz

A very basic meteo CLI written in Rust.

## Usage:

cloudz [city]

## Configuration

It uses Open Weather API and provides an hourly forecast for the next 4 days straight in your terminal.

You will need to create an account [here](https://openweathermap.org) but the route that we use doesn't require any payment method.

Running the bin a first time will generate a config file, on linux distributions at `~/.config/cloudz/config.toml`.

You must edit this file and provide: 

* `ow_api_key`: Your Open Weather API key.

* `default_city`: A default city (optional if you specify a city when running the app).

* `units_system`: The units system, by default the metric one is used, if anything is added here the imperial one will be used.

* `lang`: The lang if you want to get localized data from Open Weather (english by default).
