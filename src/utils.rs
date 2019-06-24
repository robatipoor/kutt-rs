use crate::constants::BASE_URL;
use crate::errors::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! fatal {
    ($msg:tt) => {{
        eprintln!("{} in file {} line {}", $msg, file!(), line!());
        std::process::exit(1)
    }};
}
#[allow(dead_code)]
pub fn read_from_stdin() -> Result<String> {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map_err(|e| {
            eprintln!("{}", e);
            Error::StdinError
        })
}

pub fn home_dir() -> Option<PathBuf> {
    dirs::home_dir()
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file: File = File::open(path.as_ref()).map_err(|e| {
        eprintln!("{}", e);
        Error::OpenFileError
    })?;;
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(|e| {
        eprintln!("{}", e);
        Error::ReadToStringError
    })?;;;
    Ok(buf)
}
#[allow(dead_code)]
pub fn write_file<P: AsRef<Path>>(path: P, s: &str) -> Result {
    let mut file: File = File::create(path).map_err(|e| {
        eprintln!("{}", e);
        Error::CreateFileError
    })?;
    file.write_all(s.as_bytes()).map_err(|e| {
        eprintln!("{}", e);
        Error::WriteToFileError
    })?;
    Ok(())
}

pub fn get_id_from_link(short_link: &str) -> Result<String> {
    let split_link = short_link.split(BASE_URL).collect::<Vec<&str>>();
    if split_link.len() == 2 {
        let id = split_link[1];
        // remove / form id
        Ok(id[1..].to_owned())
    } else {
        Err(Error::InvalidLinkError)
    }
}
