extern crate dotenv;
extern crate kutt;

use dotenv::dotenv;
use kutt::Kutt;

fn main() {
    dotenv().ok(); // read KUTT_API_KEY in .env file
    if Kutt::delete_link("https://kutt.it/...").is_ok() {
        println!("ok delete !");
    }
}
