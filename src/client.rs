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
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl Client {
    pub fn new(base_uri: &str) -> Client {
        Client {
            base_uri: String::from(base_uri),
            access_token: None,
            refresh_token: None,
        }
    }
    pub fn new_with_token(base_uri: &str, access_token: &str, refresh_token: &str) -> Client {
        Client {
            base_uri: String::from(base_uri),
            access_token: Some(String::from(access_token)),
            refresh_token: Some(String::from(refresh_token)),
        }
    }

    pub fn get(&self, path: &str) -> Response {
        let uri = format!("{}/{}", self.base_uri, path);
        let client = EClient::new();
        if self.access_token != None {
            client
                .get(&uri)
                .header(
                    "Authorization",
                    format!("Bearer {}", &self.access_token.as_ref().unwrap()),
                )
                .send()
                .unwrap()
        } else {
            client.get(&uri).send().unwrap()
        }
    }

    pub fn post(&self, path: &str, json: &HashMap<&str, &str>) -> Response {
        let uri = format!("{}/{}", self.base_uri, path);
        let client = EClient::new();
        if self.access_token != None {
            client
                .post(uri)
                .header("Content-Type", "Application/json")
                .header("Accept", "Application/json")
                .header(
                    "Authorization",
                    format!("Bearer {}", &self.access_token.as_ref().unwrap()),
                )
                .json(json)
                .send()
                .unwrap()
        } else {
            client
                .post(uri)
                .header("Content-Type", "Application/json")
                .header("Accept", "Application/json")
                .json(json)
                .send()
                .unwrap()
        }
    }
}
