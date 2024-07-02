extern crate reqwest;
extern crate serde;
extern crate wifiscanner;
extern crate dirs;
extern crate serde_yaml;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wifiscanner::Wifi;
use dirs::config_dir;
use std::{fs::{DirBuilder, File}, io::Write};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GpsLocation {
    pub accuracy: f64,
    pub location: Location,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}
// Define a struct for the request body
#[derive(Debug, Serialize)]
pub struct WifiAccessPoint {
    pub macAddress: String,
    pub signalStrength: i32,
}

const BASE_URL: &str = "https://www.googleapis.com/geolocation/v1/geolocate?key=";

pub fn get_networks() -> Vec<Wifi> {
    wifiscanner::scan().unwrap()
}

pub fn read_apikey() -> String {
    let config_dir = config_dir().unwrap();
    let config_file = config_dir.join("wifi-locator").join("config.yaml");
    if !config_file.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(config_file.parent().unwrap())
            .expect("Unable to create directory");
        let mut file = File::create(&config_file).expect("Unable to create file");
        file.write_all(b"api_key: my-gcloud-api-key").expect("Unable to write to file");
        panic!("Please add your Google Cloud API key to the config file at {:?}", &config_file);
    }

    let contents = std::fs::read_to_string(&config_file).unwrap();
    
    let yaml: Value =
        serde_yaml::from_str(&contents).expect("Unable to parse yaml from config file");
    let apikey = yaml["api_key"].as_str().unwrap();
    if apikey == "my-gcloud-api-key" {
        panic!("Please add your Google Cloud API key to the config file at {:?}", config_file);
    }
    println!("API Key:{}",apikey.to_string());
    apikey.to_string()
}

/// Return GPS location using a Vec of wifiscanner::Wifi. Uses Google's GPS location service
pub async fn get_location(networks: Vec<Wifi>) -> Result<Vec<GpsLocation>, reqwest::Error> {

    let api_key = read_apikey();
    let mut url = BASE_URL.to_string();
    url = url + &api_key ;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = reqwest::Client::new();

    let mut gps_locations: Vec<GpsLocation> = Vec::new();
    
    for network in networks {
        let wifi_access_points = WifiAccessPoint {
            macAddress: network.mac,
            signalStrength: -90
        };
        let payload = json!({
            "wifiAccessPoints": [
                wifi_access_points
            ]
        });
        let gps: GpsLocation = client.post(&url).headers(headers.clone()).json(&payload).send().await?.json::<GpsLocation>().await?;
        gps_locations.push(gps);
    }

    Ok(gps_locations)
}
