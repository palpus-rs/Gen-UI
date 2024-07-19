use crate::error::Errors;

/// # Checker trait
/// CompilerChecker trait is used to check the compiler status
/// If the environment is not ready, it will return an error
pub trait Checker {
    /// check everything is ready
    fn check(&self) -> Result<(), Errors> {
        self.check_env().and_then(|_| Self::check_other())
    }
    /// check the environment
    fn check_env(&self) -> Result<(), Errors>;
    /// check other
    fn check_other() -> Result<(), Errors> {
        Ok(())
    }
}
