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
        println!("ok success login !");
        return;
    }
    if let Some(link) = app.delete {
        match Kutt::delete_link(link.as_str()) {
            Ok(_) => println!("ok deleted !"),
            Err(e) => fatal!(e),
        }
        return;
    }
    if app.target_url.is_some() && app.custom_url.is_some() && app.password.is_some() {
        match Kutt::target_url(app.target_url.unwrap().as_str())
            .reuse()
            .custom_url(app.custom_url.unwrap().as_str())
            .password(app.password.unwrap().as_str())
            .create_short_link()
        {
            Ok(o) => println!("{}", o),
            Err(e) => fatal!(e),
        }
        return;
    } else if app.target_url.is_some() && app.custom_url.is_some() {
        match Kutt::target_url(app.target_url.unwrap().as_str())
            .reuse()
            .custom_url(app.custom_url.unwrap().as_str())
            .create_short_link()
        {
            Ok(o) => println!("{}", o),
            Err(e) => fatal!(e),
        }
        return;
    } else if app.target_url.is_some() {
        match Kutt::target_url(app.target_url.unwrap().as_str())
            .reuse()
            .create_short_link()
        {
            Ok(o) => println!("{}", o),
            Err(e) => fatal!(e),
        }
        return;
    }
    match read_from_stdin() {
        Ok(link) => {
            if app.custom_url.is_some() {
                match Kutt::target_url(link.as_str())
                    .reuse()
                    .custom_url(app.custom_url.unwrap())
                    .create_short_link()
                {
                    Ok(o) => println!("{}", o),
                    Err(e) => fatal!(e),
                }
            } else {
                match Kutt::target_url(link.as_str()).reuse().create_short_link() {
                    Ok(o) => println!("{}", o),
                    Err(e) => fatal!(e),
                }
            }
        }
        Err(e) => fatal!(e),
    }
}
