extern crate reqwest;
extern crate serde_derive;

use reqwest::blocking::Client as EClient;
use reqwest::blocking::Response;
use reqwest::Error;
use serde_derive::Deserialize;
use std::collections::HashMap;

use log::{info, warn};

#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    expires_in: i64,
    pub refresh_token: String,
    scope: Option<String>,
    token_type: String,
}
pub struct Client {
    base_uri: String,
}

impl Client {
    pub fn new(base_uri: &str) -> Client {
        Client {
            base_uri: String::from(base_uri),
        }
    }

    pub fn get(&self, path: &str) -> Response {
        let uri = format!("{}/{}", self.base_uri, path);
        let client = EClient::new();
        client.get(uri).send().unwrap()
    }

    pub fn post(&self, path: &str, json: &HashMap<&str, &str>) -> Response {
        let uri = format!("{}/{}", self.base_uri, path);
        let client = EClient::new();
        client
            .post(uri)
            .header("Content-Type", "Application/json")
            .header("Accept", "Application/json")
            .json(json)
            .send()
            .unwrap()
    }
}
