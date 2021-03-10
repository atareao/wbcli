extern crate clap;
extern crate log;
extern crate read_input;

mod config;
mod logger;

use clap::{App, Arg, SubCommand};
use log::{error, info, warn, LevelFilter};
use read_input::prelude::*;

use config::Config;
use logger::configure_log;

fn main() {
    configure_log();
    info!("Test");

    let config = Config::init();
    let mut settings = config.read();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("dapcion")
                .short("d")
                .long("dapcion")
                .help("Configure wbcli")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("configure")
                .help("Configure wbcli")
                .takes_value(false),
        )
        .get_matches();
    if settings.url == "" || settings.secret == "" || settings.key == "" {
        if !matches.is_present("config") {
            println!("Error: wbcli is not configured. You must configure before");
        }
    }
    if matches.is_present("config") {
        println!("Please following data for configure wbcli");
        let default_url = String::from(&settings.url);
        let msg_url = if default_url != "" {
            format!("Url [{}]: ", &default_url)
        } else {
            String::from("Url: ")
        };
        let url: String = input()
            .msg(msg_url)
            .default(String::from(default_url))
            .get();
        let key: String = input().msg("Key: ").default(settings.key).get();
        let secret: String = input().msg("Secret: ").default(settings.secret).get();
        println!("{}", &url);
        println!("{}", &key);
        println!("{}", &secret);
        settings.url = url;
        settings.key = key;
        settings.secret = secret;
        println!("{:#?}", &settings);
        config.save(&settings);
    }
}
