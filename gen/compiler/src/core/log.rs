use ansi_term::Color;
use env_logger::{Builder, Env};
use gen_utils::common::msg::{LOGO, LOG_INIT};
use log::{error, info, warn};
use std::io::Write;

use crate::CONF;

pub fn init_log() -> () {
    let conf = CONF.lock().unwrap();

    // get and read env GENUI_LOGO ------------------------------------------------------------------------
    let print_logo = std::env::var("GENUI_LOGO").map_or_else(
        |_| {
            // read gen toml file
            match conf.as_ref() {
                Ok(conf) => {
                    if let Some(val) = conf["compiler"]["logo"].as_value() {
                        val.as_bool().unwrap()
                    } else {
                        true
                    }
                }
                Err(e) => {
                    warn(e.to_string().as_str());
                    true
                }
            }
        },
        |flag| flag.parse::<bool>().unwrap(),
    );

    // GENUI_LOG_LEVEL --------------------------------------------------------------------------------------
    let log_level = std::env::var("GENUI_LOG_LEVEL").map_or_else(
        |_| {
            // read gen toml file
            match conf.as_ref() {
                Ok(conf) => {
                    if let Some(val) = conf["compiler"]["log_level"].as_value() {
                        val.as_str().unwrap().to_string()
                    } else {
                        "info".to_string()
                    }
                }
                Err(e) => {
                    warn(e.to_string().as_str());
                    "info".to_string()
                }
            }
        },
        |flag| flag,
    );

    // init log env -----------------------------------------------------------------------------------------
    let env = Env::default()
        .filter_or("GENUI_LOG_LEVEL", &log_level)
        .write_style_or("GENUI_LOG_STYLE", "always");

    Builder::from_env(env)
        .format(|buf, record| {
            let title = Color::RGB(255, 112, 67).paint("GenUI-Compiler");
            let timestamp = buf.timestamp();
            let timestamp = Color::Blue.paint(timestamp.to_string());

            writeln!(
                buf,
                "{} :: [{}] :: {} >>> {}",
                title,
                timestamp,
                record.level(),
                record.args()
            )
        })
        .init();

    if print_logo {
        info!("{}", Color::RGB(255, 112, 67).paint(LOGO));
    }
    // log serve start
    info(LOG_INIT);
}

pub fn info(msg: &str) -> () {
    info!("{}", Color::Green.paint(msg));
}

pub fn warn(msg: &str) -> () {
    warn!("{}", Color::Yellow.paint(msg));
}

pub fn error(msg: &str) -> () {
    error!("{}", Color::Red.paint(msg));
}

pub fn error_and_exit(msg: &str) -> ! {
    error!("{}", Color::Red.paint(msg));
    std::process::exit(1)
}
