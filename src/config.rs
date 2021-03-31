use log::{info, warn};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::Result;

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub version: String,
    pub url: String,
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
    pub debug: bool,
}

pub struct Config {
    path: String,
}

impl Config {
    pub fn init() -> Config {
        info!("Init {}", "init");
        let mut path = dirs::config_dir().unwrap();
        path.push(env!("CARGO_PKG_NAME"));
        match fs::create_dir_all(&path) {
            Ok(resultado) => println!("Directorio creado {}", "ok"),
            Err(e) => panic!("Adios"),
        };
        path.push(String::from(CONFIG_FILENAME));
        Config {
            path: String::from(path.to_str().unwrap()),
        }
    }

    pub fn delete(&self) -> Result<()> {
        fs::remove_file(&self.path)
    }

    pub fn read(&self) -> Settings {
        confy::load_path(&self.path).unwrap()
    }

    pub fn save(&self, settings: &Settings) {
        confy::store_path(&self.path, settings);
    }
}
