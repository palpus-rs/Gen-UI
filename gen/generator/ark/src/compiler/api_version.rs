use std::{process::Command, str::FromStr};

use gen_utils::{
    compiler::Version,
    error::{CompilerError, Errors},
};
use which::which;

const JDK_MISSING: &str = r#"❗Can not find JDK in your system, please install it!
    1. Download JDK from official website: https://www.oracle.com/java/technologies/javase-jdk17-downloads.html
    2. Install JDK and set JAVA_HOME environment variable;"#;

/// # ArkTs API Version
/// - **V9: HarmonyOS ArkTS API 9**
///     - JDK 17
///     - Node.js 14.19.1<= version <= 17.0.0
///     - Hvigor verison >= 2.0.0
///     - compile_plugin version >= 2.0.0
///     - sdkmgr version >= 1.0.0.100
/// - **V8: HarmonyOS ArkTS API 8**
///     - JDK 17
///     - Node.js 14.19.1<= version <= 17.0.0
///     - Hvigor verison >= 1.0.6
///     - compile_plugin version >= 1.0.6
///     - sdkmgr version >= 1.0.0.100
/// > This feature is suitable for building applications/services with API Version 8-9 and above.
/// >
/// > API 9 uses a new way of using hvigorw to execute commands related to hvigor, which is different from the way API 8 uses npm script for execution.
pub enum ApiVersion {
    V9,
    V8,
}

impl ApiVersion {
    /// ## Check API ToolChain Version
    pub fn check(&self) -> Result<(), Errors> {
        match self {
            ApiVersion::V9 => todo!(),
            ApiVersion::V8 => {
                // check JDK version is 17 --------------------------------------------------
                if !self.jdk_is(17)? {
                    return Err(Errors::CompilerError(CompilerError::env_check(
                        "JDK",
                        "JDK version should be 17",
                        None,
                    )));
                }
                //check Node.js version is 14.19.1<= version <= 17.0.0 -----------------------
                

                Ok(())
            }
        }
    }
    fn jdk_is(&self, version: u32) -> Result<bool, Errors> {
        let jdk_version: Version = self.jdk_version()?;
        Ok(jdk_version.match_major(version) == 0)
    }
    /// ## Get JDK version
    /// Get JDK version from the system if is installed
    /// ### Return
    /// return `Result<Version, Errors>`
    /// - `Ok(Version)` if JDK is installed and config is correct
    /// - `Err(Errors)` if JDK is not installed or config is incorrect
    fn jdk_version(&self) -> Result<Version, Errors> {
        match which("java") {
            Ok(_) => {
                // get version ----------------------------------------------------------
                let output = Command::new("java")
                    .arg("--version")
                    .output()
                    .map_err(|e| {
                        Errors::CommandError(format!("Failed to execute command: {}", e))
                    })?;

                let output_str = String::from_utf8(output.stdout).unwrap();
                Version::from_str(
                    output_str
                        .trim()
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .trim_matches('"'),
                )
            }
            Err(_) => {
                return Err(Errors::CompilerError(CompilerError::env_check(
                    "JDK",
                    JDK_MISSING,
                    None,
                )))
            }
        }
    }
}

#[cfg(test)]
mod test_api_version {
    use super::*;

    #[test]
    fn test_check_jdk() {
        let api = ApiVersion::V8;
        let res = api.jdk_version();
        assert!(res.is_ok());
    }
}
