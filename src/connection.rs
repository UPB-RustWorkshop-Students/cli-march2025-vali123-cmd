use chrono::{DateTime, Local};
use std::env;
use dotenv::dotenv;
use reqwest;
use serde_json::Value;
use std::error::Error;

#[derive(Debug)]
pub struct CityInfo {
    pub name: String,
    pub country: String,
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
}

fn getAPIkey() -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    Ok(env::var("API_KEY").map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?)
}

pub fn get_data(city: String) -> Result<CityInfo, Box<dyn Error + Send + Sync>> {
    let api_key = getAPIkey()?;
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric");
    
    let response = reqwest::blocking::get(&url)?;
    
    if response.status().is_success() {
        let data = response.json::<serde_json::Value>()?;
        let city_info = CityInfo {
            name: data["name"].as_str().ok_or("Missing name")?.to_string(),
            country: data["sys"]["country"].as_str().ok_or("Missing country")?.to_string(),
            temperature: data["main"]["temp"].as_f64().ok_or("Missing temperature")?,
            humidity: data["main"]["humidity"].as_f64().ok_or("Missing humidity")?,
            wind_speed: data["wind"]["speed"].as_f64().ok_or("Missing wind speed")?,
        };
        Ok(city_info)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get data: {}", response.status())
        )))
    }
}
