use crate::{helpers::{json_req , text_req}, models::{Wthr , Geo}};

const IPIFY_URL: &str = "https://api.ipify.org";
const IP_GEO_URL: &str = "https://api.ipgeolocation.io/ipgeo";
// i know it's a bad practice to hardcode the api keys but NVM
const IP_GEO_API_KEY: &str = "c6c1951f85724116900b669c6ab4b100";
const OWTHR_URL: &str = "https://api.openweathermap.org/data/2.5/weather";
// const IP_GEO_ALLOWED_LANGS: [&str; 9] = ["en", "de", "ru", "ja", "fr", "cn", "es", "cs", "it"];
const OWTHR_API_KEY: &str = "4faea495b1e5a50fe3c8c32bab9c14f8";
// const OWTHR_ALLOWED_LANGS: [&str; 49] = [
//     "af", "al", "ar", "az", "bg", "ca", "cz", "da", "de", "el", "en", "eu", "fa", "fi", "fr", "gl",
//     "he", "hi", "hr", "hu", "id", "it", "ja", "kr", "la", "lt", "mk", "no", "nl", "pl", "pt",
//     "pt_br", "ro", "ru", "sv", "se", "sk", "sl", "sp", "es", "sr", "th", "tr", "ua", "uk", "vi",
//     "zh_cn", "zh_tw", "zu",
// ];


pub fn fetch_weather(lat: &str, lon: &str) -> Result<Wthr, ureq::Error> {
    let url = format!(
        "{}?lat={}&lon={}&appid={}&units=metric",
        OWTHR_URL, lat, lon, OWTHR_API_KEY
    );
    json_req(&url)
}

pub fn get_ip_loc(ip: String) -> Result<Geo, ureq::Error> {
    let url = format!("{}?apiKey={}&ip={}", IP_GEO_URL, IP_GEO_API_KEY, ip);
    json_req(&url)
}


pub fn get_ip() -> String {
    text_req(IPIFY_URL).expect("error happened while checking ip")
}