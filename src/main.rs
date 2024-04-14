mod api;
mod helpers;
mod models;

use crate::{
    api::{fetch_weather, get_ip, get_ip_loc},
    helpers::print_weather_info,
};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("wthr")
        .about("get your local weather as temp , temp_max and temp_min")
        .version("0.1.0")
        .arg(
            Arg::new("ip")
                .short('i')
                .long("ip")
                .required(false)
                .num_args(1),
        )
        .get_matches();
    let ip = match matches.get_one::<String>("ip") {
        Some(ip) => ip.to_string(),
        None => get_ip(),
    };
    // println!("IP Address: {}", ip);

    let ip_loc = get_ip_loc(ip).unwrap();
    println!(
        "Latitude: {}, Longitude: {}",
        ip_loc.latitude, ip_loc.longitude
    );

    let wthr = fetch_weather(&ip_loc.latitude, &ip_loc.longitude).unwrap();

    print_weather_info(wthr)
}
