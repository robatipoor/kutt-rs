use crate::constants::*;
use crate::errors::*;
use crate::utils::*;
use std::env;

pub struct ApiKey;

impl ApiKey {
    pub fn get() -> Result<String> {
        env::var("KUTT_API_KEY")
            .or(read_file(&*PATH_FILE_API_KEY))
            .map(|key| key.trim().to_owned())
    }
    pub fn set(key: &str) -> Result {
        write_file(&*PATH_FILE_API_KEY, key)
    }
}
