extern crate reqwest;
extern crate serde_derive;

use reqwest::blocking;
use reqwest::blocking::Client as EClient;
use reqwest::blocking::Response;
use reqwest::Error;
use serde_derive::Deserialize;
use std::collections::HashMap;

use log::{info, warn};

#[derive(Debug, Deserialize)]
struct Token {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
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

    pub fn get(&self, path: &str) -> String {
        let uri = format!("{}/{}", self.base_uri, path);
        let client = EClient::new();
        let response = client.get(uri).send();
        println!("{:#?}", response);
        format!("{}", "nada")
    }

    pub fn post(&self, path: &str, json: &HashMap<&str, &str>) -> Result<Response, Error> {
        let uri = format!("{}/{}", self.base_uri, path);
        let client = EClient::new();
        client
            .post(uri)
            .header("Content-Type", "Application/json")
            .header("Accept", "Application/json")
            .json(json)
            .send()
    }
}
