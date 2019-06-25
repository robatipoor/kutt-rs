use crate::constants::*;
use crate::errors::{Error, Result};
use crate::utils::get_id_from_link;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListLinks {
    pub list: Vec<Link>,
    #[serde(rename = "countAll")]
    pub count_all: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    count: u32,
    #[serde(rename = "createdAt")]
    created_at: String,
    id: String,
    target: String,
    password: bool,
    #[serde(rename = "shortUrl")]
    short_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kutt {
    target: String,
    customurl: Option<String>,
    password: Option<String>,
    reuse: bool,
}

impl Kutt {
    pub fn target_url(target: &str) -> Self {
        Kutt {
            target: target.to_owned(),
            customurl: None,
            password: None,
            reuse: false,
        }
    }
    pub fn custom_url<S: Into<String>>(mut self, customurl: S) -> Self {
        self.customurl = Some(customurl.into());
        self
    }
    pub fn password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = Some(password.into());
        self
    }
    pub fn reuse(mut self) -> Self {
        self.reuse = true;
        self
    }

    pub fn create_short_link(&self) -> Result<String> {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            #[serde(rename = "createdAt")]
            created_at: String,
            id: String,
            target: String,
            password: bool,
            #[serde(rename = "shortUrl")]
            short_url: String,
        }
        let request = serde_json::to_string(self).unwrap();
        let response: Response = Client::new()
            .post(&format!("{}/{}", BASE_URL, "api/url/submit"))
            .header("X-API-Key", &*KUTT_API_KEY.as_str())
            .header("Content-Type", "application/json")
            .body(request)
            .send()
            .map_err(|e| {
                eprintln!("{}", e);
                Error::SendRequsetError
            })?
            .json()
            .map_err(|e| {
                eprintln!("{}", e);
                Error::ParseJsonError
            })?;
        Ok(response.short_url)
    }
    pub fn delete_link(short_link: &str) -> Result {
        let body = match get_id_from_link(short_link) {
            Ok(id) => r#"{"id":""#.to_owned() + id.as_str() + r#"","domain":null}"#,
            Err(e) => return Err(e),
        };
        let resp: reqwest::Response = Client::new()
            .post(&format!("{}/{}", BASE_URL, "api/url/deleteurl"))
            .header("X-API-Key", &*KUTT_API_KEY.as_str())
            .header("Content-Type", "application/json")
            .body(body.clone())
            .send()
            .map_err(|e| {
                eprintln!("{}", e);
                Error::SendRequsetError
            })?;
        if resp.status() == 200 {
            Ok(())
        } else {
            Err(Error::UnsuccessResponseError)
        }
    }
    #[allow(dead_code)]
    pub fn list_links() -> Result<ListLinks> {
        let resp: ListLinks = Client::new()
            .get(&format!("{}/{}", BASE_URL, "api/url/geturls"))
            .header("X-API-Key", &*KUTT_API_KEY.as_str())
            .send()
            .map_err(|e| {
                eprintln!("{}", e);
                Error::SendRequsetError
            })?
            .json()
            .map_err(|e| {
                eprintln!("{}", e);
                Error::ParseJsonError
            })?;
        Ok(resp)
    }
}
