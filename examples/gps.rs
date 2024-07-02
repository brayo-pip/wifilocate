
#[tokio::main]
async fn main(){
	println!( "{:?}",
		wifilocate::get_location(wifilocate::get_networks()).await.ok()
	 );
}