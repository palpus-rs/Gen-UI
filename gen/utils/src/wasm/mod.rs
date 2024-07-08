use std::{any::Any, path::Path, process::Child};

use crate::error::Errors;

pub trait WasmImpl: Any + Clone {
    fn new() -> Self
    where
        Self: Default,
    {
        Self::default()
    }
    fn port(&mut self, port: u16) -> &mut Self;
    fn check(&mut self) -> &mut Self;
    fn no_fresh(&mut self) -> &mut Self;
    fn check_wasm(&self) -> Result<bool, Errors>;
    fn run<P>(&self, path: P) -> Result<Child, Errors>
    where
        P: AsRef<Path>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
