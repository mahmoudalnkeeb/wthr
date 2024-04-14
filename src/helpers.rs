use ureq;

use crate::models::Wthr;

pub fn json_req<T>(url: &str) -> Result<T, ureq::Error>
where
    T: serde::de::DeserializeOwned,
{
    let response: T = ureq::get(&url).call()?.into_json()?;
    Ok(response)
}

pub fn text_req(url: &str) -> Result<String, ureq::Error> {
    let response: String = ureq::get(&url).call()?.into_string()?;
    Ok(response)
}

pub fn print_weather_info(wthr: Wthr) {
    let red = "\x1b[31m";
    let green = "\x1b[32m";
    let purple = "\x1b[35m";
    let blue = "\x1b[36m";
    let reset = "\x1b[0m";

    println!(" {}Weather Information", red);
    println!("---------------------{}", reset);

    // Extract weather information
    let max_temp = wthr.main.temp_max;
    let min_temp = wthr.main.temp_min;
    let temp = wthr.main.temp;
    let humidity = wthr.main.humidity;
    let wind_speed = wthr.wind.speed;
    let weather_status = &wthr.weather[0].main;
    let weather_description = &wthr.weather[0].description;

    // Print weather information with colors
    println!("Max Temperature: {}{}°C{}", blue, max_temp, reset);
    println!("Min Temperature: {}{}°C{}", blue, min_temp, reset);
    println!("Temperature: {}{}°C{}", blue, temp, reset);
    println!("Humidity: {}{}%{}", green, humidity, reset);
    println!("Wind Speed: {}{}km/h{}", green, wind_speed, reset);
    println!("Weather Status: {}{}{}", purple, weather_status, reset);
    println!(
        "Weather Description: {}{}{}",
        purple, weather_description, reset
    );
}
