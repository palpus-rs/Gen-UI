pub mod api_version;

use api_version::ApiVersion;
use gen_utils::compiler::CompilerChecker;

/// # HarmonyOS ArkTS Compiler
/// Each compiler need to implement the `Compiler` trait.
pub struct Ark{
    api_version: ApiVersion,
}

impl CompilerChecker for Ark {
    fn check_env(&self) -> Result<(), gen_utils::error::Errors> {
        self.api_version.check()
    }
}