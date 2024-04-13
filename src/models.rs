#![allow(unused)]
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Deserialize)]
pub struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize)]
pub struct Main {
    pub   temp: f64,
    feels_like: f64,
    pub   temp_min: f64,
    pub   temp_max: f64,
    pressure: u32,
    humidity: u32,
    sea_level: u32,
    grnd_level: u32,
}

#[derive(Deserialize)]
pub struct Wind {
    speed: f64,
    deg: u32,
    gust: f64,
}

#[derive(Deserialize)]
pub struct Clouds {
    all: u32,
}

#[derive(Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    type_: u32,
    id: u32,
    country: String,
    sunrise: u32,
    sunset: u32,
}

#[derive(Deserialize)]
pub struct Wthr {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    pub  main: Main,
    visibility: u32,
    wind: Wind,
    clouds: Clouds,
    dt: u32,
    sys: Sys,
    timezone: i32,
    id: u32,
    name: String,
    cod: u32,
}

#[derive(Deserialize)]
pub struct Currency {
    code: String,
    name: String,
    symbol: String,
}

#[derive(Deserialize)]
pub struct TimeZone {
    name: String,
    offset: i32,
    offset_with_dst: i32,
    current_time: String,
    current_time_unix: f64,
    is_dst: bool,
    dst_savings: i32,
    dst_exists: bool,
    dst_start: DstPeriod,
    dst_end: DstPeriod,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DstPeriod {
    utc_time: String,
    duration: String,
    gap: bool,
    dateTimeAfter: String,
    dateTimeBefore: String,
    overlap: bool,
}

#[derive(Deserialize)]
pub struct Geo {
    ip: String,
    continent_code: String,
    continent_name: String,
    country_code2: String,
    country_code3: String,
    country_name: String,
    country_name_official: String,
    country_capital: String,
    state_prov: String,
    state_code: String,
    district: String,
    city: String,
    zipcode: String,
  pub  latitude: String,
  pub  longitude: String,
    is_eu: bool,
    calling_code: String,
    country_tld: String,
    languages: String,
    country_flag: String,
    geoname_id: String,
    isp: String,
    connection_type: String,
    organization: String,
    country_emoji: String,
    currency: Currency,
    time_zone: TimeZone,
}