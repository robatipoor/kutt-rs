#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

mod api;
mod apikey;
mod args;
mod constants;
mod errors;
mod utils;
use crate::api::Kutt;
use crate::apikey::ApiKey;
use crate::args::AppArgs;
use crate::utils::read_from_stdin;

fn main() {
    let app = AppArgs::get_app_args();
    if let Some(key) = app.login {
        ApiKey::set(&*key).unwrap();
        println!("{}", "ok success login");
        return;
    }
    if let Some(link) = app.delete {
        Kutt::delete_link(link.as_str());
        println!("{}", "ok success delete");
        return;
    }
    if app.target_url.is_some() && app.custom_url.is_some() && app.password.is_some() {
        println!("{}", "ok success delete");
        return;
    } else if app.target_url.is_some() && app.custom_url.is_some() {

    } else if app.target_url.is_some() {

    }
    println!(
        "{:?}",
        Kutt::target_url(read_from_stdin().unwrap().as_str()).create_short_link()
    );
}
