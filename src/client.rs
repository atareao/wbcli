extern crate reqwest;

use reqwest::blocking;
use reqwest::blocking::Client as EClient;
use reqwest::blocking::Response;

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

    pub fn post(&self, path: &str, msg: &'static str) -> String {
        let uri = format!("{}/{}", self.base_uri, path);
        let client = EClient::new();
        let response = client.post(uri).body(msg).send();
        println!("{:#?}", response);
        format!("{}", "nada")
    }
}
