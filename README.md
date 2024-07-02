# wifilocate

A crate to return your GPS location using WiFi hotspots.

## Usage

This crate is [on crates.io](https://crates.io/crates/wifilocation) and can be
used by adding `wifilocation` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
wifilocate = "0.1"
```

This crate requires an API key from [Google](https://developers.google.com/maps/documentation/geolocation/get-api-key) to use the Geolocation API.

You will be prompted to set the key when you use the crate.

## Example

```rust

use wifilocate;

#[tokio::main]
async fn main(){
    println!( "{:?}",
        wifilocate::get_location(wifilocate::get_networks()).await.ok()
     );
}
```

## Changelog

- 0.1.0 - Initial release

## Copyright

Copyright 2024 [Brian Vuku]

see [LICENSE](/LICENSE)
