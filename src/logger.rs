extern crate log;
extern crate log4rs;

use log::{error, info, warn, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config as LogConfig, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;

pub fn configure_log(debug: bool) -> Handle {
    let levelFilter = if debug {
        LevelFilter::Info
    } else {
        LevelFilter::Off
    };
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();

    let config = LogConfig::builder()
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", levelFilter),
        )
        .build(
            Root::builder()
                .appender("requests")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap()
}
