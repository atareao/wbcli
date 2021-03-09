extern crate clap;
extern crate log;
extern crate read_input;
extern crate syslog;

mod config;

use clap::{App, Arg, SubCommand};
use log::{LevelFilter, SetLoggerError};
use read_input::prelude::*;
use syslog::{BasicLogger, Facility, Formatter3164};

use config::Config;

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

    let config = Config::init();
    let mut settings = config.read();

    let matches = App::new("wbcli")
        .version("1.0")
        .author("Lorenzo Carbonell <a.k.a. atareao>")
        .about("Wallabag command line interface")
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
