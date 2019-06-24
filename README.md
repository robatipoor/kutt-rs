# client short link Kutt
[![Crates.io](https://img.shields.io/crates/v/kutt.svg?style=plastic)](http://crates.io/crates/kutt)
[![Build Status](https://travis-ci.org/robatipoor/kutt-rs.svg?branch=master)](https://travis-ci.org/robatipoor/kutt-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/g1sii8fby1it2jja/branch/master?svg=true)](https://ci.appveyor.com/project/robatipoor/kutt-rs/branch/master)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
### work in progress
Command line tool for [kutt.it](https://kutt.it)

**install**

```sh
cargo install kutt
```
## Usage command line

```sh
USAGE:
    kutt [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --custom-url <DOMAIN>    Set a domain name
    -d, --delete <URL>           Set a url
    -l, --login <API_KEY>        Set a apikey
    -p, --password <PASSWORD>    Set a password
    -t, --target-url <URL>       Set a url
```

## How to use crate
```sh
cargo add kutt
```
```rs
extern crate dotenv;
extern crate kutt;

use dotenv::dotenv;
use kutt::Kutt;

fn main() {
    dotenv().ok(); // read KUTT_API_KEY in .env file/
    let slink = Kutt::target_url("https://addr-example...")
        .custom_url("custom-url")
        .create_short_link()
        .unwrap();
    println!("{}", slink);
}
```