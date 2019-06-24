use crate::apikey::ApiKey;
use crate::utils::home_dir;
use std::path::PathBuf;

pub const BASE_URL: &str = "https://kutt.it";

#[cfg(not(target_os = "windows"))]
pub const API_KEY_FILE_NAME: &str = ".kutt-api-key.txt";
#[cfg(target_os = "windows")]
pub const API_KEY_FILE_NAME: &str = "kutt-api-key.txt";
lazy_static! {
    pub static ref PATH_FILE_API_KEY: PathBuf = home_dir().unwrap().join(API_KEY_FILE_NAME);
    pub static ref KUTT_API_KEY: String = ApiKey::get().unwrap();
}
