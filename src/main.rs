extern crate confy;
extern crate dirs;
extern crate log;
extern crate serde_derive;
extern crate syslog;

use confy::ConfyError;
use log::{LevelFilter, SetLoggerError};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use syslog::{BasicLogger, Facility, Formatter3164};

#[derive(Default, Debug, Serialize, Deserialize)]
struct Settings {
    version: String,
    url: String,
    secret: String,
    key: String,
}

fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "wbcli".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");

    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Debug));

    let mut config_file = dirs::config_dir().unwrap();
    config_file.push("wbcli");
    match fs::create_dir_all(&config_file) {
        Ok(resultado) => log::debug!("El directorio ha sido creado o existe {}", "ok"),
        Err(e) => panic!("Adios"),
    }
    config_file.push("config.json");
    println!("{}", &config_file.display());
    let mut settings: Settings = confy::load_path(&config_file).unwrap();
    settings.version = "0.1.0".into();
    confy::store_path(&config_file, settings);
    println!("{}", &config_file.display());
}
