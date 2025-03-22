use chrono::{DateTime, Local};
use std::env;
use dotenv::dotenv;
use reqwest;
use serde_json::Value;

//API key

struct CityInfo {
    // TODO: define elements in the structure
    name: String,
    country: String,
    temperature: f64,
    humidity: f64,
    wind_speed: f64,


}

/// Method that is handling the request to the OpenWeather api
/// and parsing the response
///
/// Returns weather details about a certain city
fn getAPIkey() -> String {
    dotenv().ok();

    env::var("APIKEY").expect("APIKEY not found in .env file")
}

pub fn get_data(city: String) {
    let api_key = getAPIkey();
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric");
    match reqwest::blocking::get(&url) {
        Ok(response) => {
            // TODO: Check status code and then parse response
            if response.status().is_success() {
                let data = response.json::<serde_json::Value>().unwrap();
                let city_info = CityInfo {
                    name: data["name"].as_str().unwrap().to_string(),
                    country: data["sys"]["country"].as_str().unwrap().to_string(),
                    temperature: data["main"]["temp"].as_f64().unwrap(),
                    humidity: data["main"]["humidity"].as_f64().unwrap(),
                    wind_speed: data["wind"]["speed"].as_f64().unwrap(),
                };

                };
        }
        Err(error) => {
            // TODO: Handle error
        eprintln!("{}", error);
        }
    }
    }



