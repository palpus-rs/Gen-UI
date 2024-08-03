//! Environment variables used by the compiler.
//! # GenUI Env
//! Environment variables used by the compiler.
//!
//! - `GENUI_TARGET`: Set GenUI Compiler target
//! - `GENUI_LOGO`: Set Logo is print or not
//! - `GENUI_LOG_LEVEL`: Set GenUI Log Level
//!
//! ## Details
//!
//! |Env Name|Default Value|Type|Option Values|
//! |--|--|--|--|
//! |`GENUI_TARGET`|`makepad`|`String`|1. `makepad`|
//! |`GENUI_LOGO`|`true`|`Bool`|1. `true`<br /> 2. `false`|
//! |`GENUI_LOG_LEVEL`|`info`|`String`|1. `error`<br />2. `warn`<br />3. `info`<br />4. `debug`<br />5. `trace`|
//!
//! ## How to Set Env Variable
//!
//! ### Windows
//!
//! ```bash
//! setx GENUI_TARGET makepad
//! ```
//!
//! ### Mac/Linux
//!
//! ```bash
//! export GENUI_TARGET=makepad
//! ```


/// Set GenUI Compiler target
pub const GENUI_TARGET: &str = "GENUI_TARGET";
pub const GENUI_LOG_LEVEL: &str = "GENUI_LOG_LEVEL";
pub const GENUI_LOG_STYLE: &str = "GENUI_LOG_STYLE";
pub const GENUI_LOGO: &str = "GENUI_LOGO";