mod client;
mod config;
mod logger;

use clap::{App, Arg, SubCommand};
use log::{error, info, warn, LevelFilter};
use read_input::prelude::*;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::thread;

use client::{Client, Token};
use config::Config;
use logger::configure_log;

fn main() {
    let config = Config::init();
    let mut settings = config.read();
    configure_log(settings.debug);
    info!("Test");

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("clean")
                .short("d")
                .long("clean configuration")
                .help("remove and clean existing configuration")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("entries")
                .short("e")
                .long("entries")
                .help("Retrieve all entries")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("Create an entry")
                .help("Create an entry")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .help("Test connection with wbcli")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("refresh")
                .short("r")
                .long("refresh")
                .help("Refresh wallabag token")
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
    if matches.is_present("clear") {
        config.delete();
        return;
    }
    if settings.url == "" || settings.access_token == "" || settings.refresh_token == "" {
        if !matches.is_present("config") {
            println!("Error: wbcli is not configured. You must configure before");
        }
    }
    if matches.is_present("config") {
        println!("Please following data for configure wbcli");
        let msg_url = if &settings.url != "" {
            format!("Url [{}]: ", &settings.url)
        } else {
            String::from("Url: ")
        };
        let url: String = input()
            .msg(msg_url)
            .default(String::from(&settings.url))
            .get();
        let client_id: String = input().msg("Client ID: ").get();
        let client_secret: String = input().msg("Client secret: ").get();
        let username: String = input().msg("User name: ").get();
        let password: String = input().msg("Password: ").get();
        let mut json = HashMap::new();
        json.insert("grant_type", "password");
        json.insert("client_id", &client_id);
        json.insert("client_secret", &client_secret);
        json.insert("username", &username);
        json.insert("password", &password);
        let client = Client::new(&url);
        let response = client.post("oauth/v2/token", &json);
        if &response.status() == &StatusCode::OK {
            let token: Token = response.json().unwrap();
            println!("{:#?}", &token);
            settings.url = url;
            settings.client_id = client_id;
            settings.client_secret = client_secret;
            settings.access_token = token.access_token;
            settings.refresh_token = token.refresh_token;
            println!("{:#?}", &settings);
            config.save(&settings);
            return;
        }
    }
    if matches.is_present("refresh") {
        let mut json = HashMap::new();
        json.insert("grant_type", "refresh_token");
        json.insert("refresh_token", &settings.refresh_token);
        json.insert("client_id", &settings.client_id);
        json.insert("client_secret", &settings.client_secret);
        let client = Client::new(&settings.url);
        let response = client.post("oauth/v2/token", &json);
        if &response.status() == &StatusCode::OK {
            let token: Token = response.json().unwrap();
            println!("{:#?}", &token);
            settings.access_token = token.access_token;
            settings.refresh_token = token.refresh_token;
            println!("{:#?}", &settings);
            config.save(&settings);
            return;
        }
    }

    if matches.is_present("entries") {
        let client = Client::new_with_token(
            &settings.url,
            &settings.access_token,
            &settings.refresh_token,
        );
        let response = client.get("api/entries");
        if &response.status() == &StatusCode::OK {
            let text = response.text().unwrap();
            println!("{}", text);
        }
    }
    if matches.is_present("test") {
        let client = Client::new_with_token(
            &settings.url,
            &settings.access_token,
            &settings.refresh_token,
        );
        let response = client.get("api/info");
        println!("{:#?}", &response);
        if &response.status() == &StatusCode::OK {
            let text = response.text().unwrap();
            println!("{}", text);
        }
    }
    if matches.is_present("add") {
        let url = matches.value_of("add").unwrap();
        let client = Client::new_with_token(
            &settings.url,
            &settings.access_token,
            &settings.refresh_token,
        );
        let mut json = HashMap::new();
        json.insert("url", url);
        let response = client.post("api/entries", &json);
        println!("{:#?}", &response);
        if &response.status() == &StatusCode::OK {
            let text = response.text().unwrap();
            println!("{}", text);
        }
    }
}
