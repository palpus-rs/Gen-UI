//! # GenUI Logger
//!
//! ## Logo
//!
//! You can control whether the logo is printed using the system environment variable `GENUI_LOGO` or through the configuration file in TOML format.
//!
//! - For more details, see [GenUI Environment Setup](https://palpus-rs.github.io/Gen-UI.github.io/gen/tutorials/env.html).
//! - For configuration, see [GenUI Config TOML](https://palpus-rs.github.io/Gen-UI.github.io/gen/tutorials/conf.html).
//!
//! Example:
//!
//! ```rust
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>>
//!
//!      _/_/_/  _/_/_/_/  _/      _/  _/    _/  _/_/_/
//!   _/        _/        _/_/    _/  _/    _/    _/
//!  _/  _/_/  _/_/_/    _/  _/  _/  _/    _/    _/
//! _/    _/  _/        _/    _/_/  _/    _/    _/
//!  _/_/_/  _/_/_/_/  _/      _/    _/_/    _/_/_/
//!
//! ```
//!
//! ## Services
//!
//! The GenUI Logger provides detailed information about the state of various services. Here are some log examples:
//!
//! ```rust
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Log Service is starting... Log entries will be available after the `app event::Change` occurs!
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Source Generator Service started successfully!
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> ✅ Cache Service: Cache file written successfully!
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 App is running...
//! GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Watcher Service started successfully!
//! ```
//!
//! ## Compile Timing
//!
//! The logger also tracks and displays compile timings, helping you monitor the compilation process:
//!
//! ```rust
//! GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> File "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\gen_makepad_simple\\ui\\views\\root.gen" compiled successfully.
//! GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> ✅ Cache Service: Cache file written successfully!
//! GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> File "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\gen_makepad_simple\\ui\\views\\root.gen" compiled successfully.
//! ```

use ansi_term::Color;
use env_logger::{Builder, Env};
use gen_utils::common::{msg::{LOGO, LOG_INIT}, time::local_time_default};
use log::{error, info, warn};
use std::io::Write;
use crate::CONF;
use super::env::{GENUI_LOGO, GENUI_LOG_LEVEL, GENUI_LOG_STYLE};

/// # Init Log
/// init GenUI log service. It will read the system environment variable `GENUI_LOGO` and `GENUI_LOG_LEVEL` to set the log level and print the logo.
/// If the system environment variable is not set, it will read the configuration file in the project root path.
/// If the configuration file is not found, it will use the default value.
/// > This function should be called before any other service is started.
pub fn init_log() -> () {
    let conf = CONF.lock().unwrap();

    // get and read env GENUI_LOGO ------------------------------------------------------------------------
    let print_logo = std::env::var(GENUI_LOGO).map_or_else(
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
    let log_level = std::env::var(GENUI_LOG_LEVEL).map_or_else(
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
        .filter_or(GENUI_LOG_LEVEL, &log_level)
        .write_style_or(GENUI_LOG_STYLE, "always");

    Builder::from_env(env)
        .format(|buf, record| {
            let title = Color::RGB(255, 112, 67).paint("GenUI-Compiler");
            let timestamp = local_time_default();
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