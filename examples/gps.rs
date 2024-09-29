#[tokio::main]
async fn main() {
    println!("{:?}", wifilocate::get_locations().await.ok());
}
