extern crate confy;
extern crate dirs;
extern crate serde_derive;

use serde_derive::{Deserialize, Serialize};
use std::fs;

const APP: &str = "wbcli";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub version: String,
    pub url: String,
    pub secret: String,
    pub key: String,
}

pub struct Config {
    path: String,
}

impl Config {
    pub fn init() -> Config {
        let mut path = dirs::config_dir().unwrap();
        path.push(String::from(APP));
        match fs::create_dir_all(&path) {
            Ok(resultado) => println!("Directorio creado {}", "ok"),
            Err(e) => panic!("Adios"),
        };
        path.push(String::from(CONFIG_FILENAME));
        Config {
            path: String::from(path.to_str().unwrap()),
        }
    }

    pub fn read(&self) -> Settings {
        confy::load_path(&self.path).unwrap()
    }

    pub fn save(&self, settings: &Settings) {
        confy::store_path(&self.path, settings);
    }
}
