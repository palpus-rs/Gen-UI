use std::{
    path::Path,
    process::{Child, Command},
};

use gen_utils::{error::Errors, wasm::WasmImpl};
use which::which;

const TOOLCHAIN_UNINSTALL: &str = "makepad toolchain not found, please install it!";
const TOOLCHAIN_INSTALL: &str = r#"
    makepad toolchain not found, please install it:
    1. 👍 install from makepad project branch `rik`(recommended): `cargo install --path=./tools/cargo_makepad`
    2. 👎 install from crate.io(not recommended): `cargo install cargo-makepad`
    the more information please visit: https://github.com/makepad/makepad/
"#;

#[derive(Debug, Clone)]
pub struct Wasm {
    /// 是否需要对makepad-wasm进行检查
    pub check: bool,
    /// 是否需要在每次Gen更新后重新编译
    pub fresh: bool,
    /// 默认端口 (默认8010)
    pub port: Option<u16>,
}

impl Default for Wasm {
    fn default() -> Self {
        Self {
            check: false,
            fresh: true,
            port: None,
        }
    }
}

impl WasmImpl for Wasm {
    fn new() -> Self {
        Self::default()
    }
    fn port(&mut self, port: u16) -> &mut Self {
        self.port.replace(port);
        self
    }
    fn check(&mut self) -> &mut Self {
        self.check = true;
        self
    }
    fn no_fresh(&mut self) -> &mut Self {
        self.fresh = false;
        self
    }
    /// check makepad wasm
    /// return true if makepad wasm is installed
    /// return false if makepad wasm not need to check
    /// return error if makepad wasm is not installed
    fn check_wasm(&self) -> Result<bool, Errors> {
        if self.check {
            // 表示需要检查makepad wasm是否工作
            // 其实是需要检查makepad toolchain是否安装, 如果没有安装则需要提示用户安装
            match which("cargo-makepad") {
                Ok(_) => Ok(true),
                Err(_) => Err(Errors::DepError(format!(
                    "{}\n{}",
                    TOOLCHAIN_UNINSTALL, TOOLCHAIN_INSTALL
                ))),
            }
        } else {
            // 表示不需要检查直接返回true
            Ok(false)
        }
    }
    /// path: project path for makepad
    fn run<P>(&self, path: P) -> Result<Child, Errors>
    where
        P: AsRef<Path>,
    {
        // let mut command = "cargo makepad wasm".to_string();
        let mut command = Command::new("cargo");
        command.args(["makepad", "wasm"]);
        if let Some(port) = self.port {
            command.args(["--port", &port.to_string()]);
        }
        command.args(["run", "-p", "src_gen", "--release"]);
        // command.push_str(" run -p src_gen --release");
        command
            .current_dir(path.as_ref())
            .spawn()
            .map_err(|e| Errors::CommandError(format!("makepad wasm run failed: {}", e)))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod test_wasm {
    use gen_utils::wasm::WasmImpl;

    #[test]
    fn check() {
        let mut wasm = super::Wasm::new();
        wasm.check();
        assert_eq!(wasm.check_wasm().is_ok(), true);
    }
}
