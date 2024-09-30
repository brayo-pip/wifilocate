# wifilocate

A crate to return your GPS location using WiFi network mac addresses.

## Usage

This crate is [on crates.io](https://crates.io/crates/wifilocate) and can be
used by adding `wifilocate` to the dependencies in your project's `Cargo.toml`.

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

Note that this script requires elevated privileges to work. It's preferable that you build the binary then either use `chown`  or run with `sudo`.

Alterna

## Changelog

- 0.1.0 - Initial release
- 0.1.1 - Small bug fixes
- 0.1.7 - Added GeoCoding support

## Copyright

Copyright 2024 [Brian Vuku]

see [LICENSE](/LICENSE)
