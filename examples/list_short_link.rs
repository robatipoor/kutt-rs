extern crate dotenv;
extern crate kutt;

use dotenv::dotenv;
use kutt::{Kutt, ListLinks};

fn main() {
    dotenv().ok(); // save KUTT_API_KEY to .env file
    let result: ListLinks = Kutt::list_links().unwrap();
    for link in result.list {
        println!("{:#?}", link);
    }
    // println!("{:#?}", result);
}
