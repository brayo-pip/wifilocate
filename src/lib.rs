extern crate dirs;
extern crate reqwest;
extern crate serde;
extern crate serde_yaml;
extern crate wifi_scanner;

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{DirBuilder, File},
    io::Write,
};
use wifi_scanner::Wifi;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GPSLocationWithAddress {
    pub address: String,
    pub gps_location: GpsLocation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpsLocation {
    pub accuracy: f64,
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}
// Define a struct for the request body
#[derive(Debug, Serialize)]
pub struct WifiAccessPoint {
    pub mac_address: String,
    pub signal_strength: i32,
}

const BASE_URL_GEOLOCATE: &str = "https://www.googleapis.com/geolocation/v1/geolocate?key=";
const BASE_URL_GEOCODE: &str = "https://maps.googleapis.com/maps/api/geocode/json?latlng=";

// Scan for wifi networks
pub fn get_networks() -> Vec<Wifi> {
    wifi_scanner::scan().unwrap()
    // TODO: handle this better
}

fn read_apikey() -> String {
    //TODO: Add caching for the apikey
    let config_dir = config_dir().unwrap();
    let config_file = config_dir.join("wifi-locator").join("config.yaml");
    if !config_file.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(config_file.parent().unwrap())
            .expect("Unable to create directory");
        let mut file = File::create(&config_file).expect("Unable to create file");
        file.write_all(b"apikey: my-gcloud-api-key")
            .expect("Unable to write to file");
        panic!(
            "Please add your Google Cloud API key to the config file at {:?}",
            &config_file
        );
    }

    let contents = std::fs::read_to_string(&config_file).unwrap();

    let yaml: Value =
        serde_yaml::from_str(&contents).expect("Unable to parse yaml from config file");
    let apikey = yaml["apikey"].as_str().unwrap();
    if apikey == "my-gcloud-apikey" {
        panic!(
            "Please add your Google Cloud API key to the config file at {:?}",
            config_file
        );
    }
    apikey.to_string()
}

// Scan for wifi networks and return GPS location
pub async fn get_locations() -> Result<Vec<GpsLocation>, reqwest::Error> {
    let networks = get_networks();
    get_location_from_vec(networks).await
}

// Scan for wifi networks and return GPS location with addr
pub async fn get_addresses() -> Result<Vec<GPSLocationWithAddress>, reqwest::Error> {
    let gps_locations = get_locations().await?;
    let apikey = read_apikey();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = reqwest::Client::new();

    let mut gps_locations_with_address: Vec<GPSLocationWithAddress> = Vec::new();

    for gps in gps_locations {
        let latlng = format!("{},{}", gps.location.lat, gps.location.lng);
        let url = format!("{}{}&key={}=", BASE_URL_GEOCODE, latlng, apikey);
        let response = client
            .get(&url)
            .headers(headers.clone())
            .send()
            .await?
            .json::<Value>()
            .await?;
        let address = response["results"][0]["formatted_address"]
            .as_str()
            .unwrap();
        gps_locations_with_address.push(GPSLocationWithAddress {
            address: address.to_string(),
            gps_location: gps,
        });
    }

    Ok(gps_locations_with_address)
}

// Return GPS location using a Vec of wifiscanner::Wifi. Uses Google's GPS location service
pub async fn get_location_from_vec(
    networks: Vec<Wifi>,
) -> Result<Vec<GpsLocation>, reqwest::Error> {
    let apikey = read_apikey();
    let mut url = BASE_URL_GEOLOCATE.to_string();
    url = url + &apikey;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = reqwest::Client::new();

    let mut gps_locations: Vec<GpsLocation> = Vec::new();

    for network in networks {
        let wifi_access_points = WifiAccessPoint {
            mac_address: network.mac,
            signal_strength: -90,
        };
        let payload = json!({
            "wifiAccessPoints": [
                wifi_access_points
            ]
        });
        let gps: GpsLocation = client
            .post(&url)
            .headers(headers.clone())
            .json(&payload)
            .send()
            .await?
            .json::<GpsLocation>()
            .await?;
        gps_locations.push(gps);
    }

    Ok(gps_locations)
}
