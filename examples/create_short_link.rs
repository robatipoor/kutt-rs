extern crate dotenv;
extern crate kutt;

use dotenv::dotenv;
use kutt::Kutt;

fn main() {
    dotenv().ok(); // read KUTT_API_KEY in .env file/
    let slink = Kutt::target_url("https://addr...")
        .custom_url("custom-url")
        .create_short_link()
        .unwrap();
    println!("{}", slink);
}
