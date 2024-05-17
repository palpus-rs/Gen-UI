use ansi_term::Color;
use env_logger::{Builder, Env};
use log::{error, info, warn};
use std::io::Write;

use crate::msg::{LOGO, LOG_INIT};

pub fn init_log() -> () {
    // init log env
    let env = Env::default()
        .filter_or("GENUI_LOG_LEVEL", "info")
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

    info!("{}", Color::RGB(255, 112, 67).paint(LOGO));
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
