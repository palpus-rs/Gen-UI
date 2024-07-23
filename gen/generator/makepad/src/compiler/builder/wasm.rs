use super::compiler::CompilerBuilder;

/// # Wasm Builder
#[derive(Debug, Clone)]
pub struct WasmBuilder {
    parent: CompilerBuilder,
    /// 是否需要对wasm进行检查
    check: bool,
    /// 是否需要在每次Gen更新后重新编译
    fresh: bool,
    /// 默认端口 (默认8010)
    port: Option<u16>,
}

impl WasmBuilder {
    /// ## set wasm check
    /// wasm check can help you check wasm toolchain
    /// ### Makepad
    /// if you use Makepad, it will check `makepad toolchain`
    /// makepad toolchain not found, please install it:
    /// 1. 👍 install from makepad project branch `rik`(recommended): `cargo install --path=./tools/cargo_makepad`
    /// 2. 👎 install from crate.io(not recommended): `cargo install cargo-makepad`
    /// the more information please visit: https://github.com/makepad/makepad/
    /// ### Example
    /// ```rust
    /// let mut app = app(Target::Makepad)
    /// .wasm()
    /// .check();
    /// ```
    pub fn check(mut self) -> Self {
        self.check = true;
        self
    }
    /// ## set wasm no fresh
    /// 
    /// wasm fresh can help you recompile wasm after Gen update
    /// 
    /// No need to manually recompile wasm, it can be completed by Gen's compiler without the need for commands
    /// ### Example
    /// ```rust
    /// let mut app = app(Target::Makepad)
    /// .wasm()
    /// .no_fresh();
    /// ```
    pub fn no_fresh(mut self) -> Self {
        self.fresh = false;
        self
    }
    /// ## set wasm port
    /// wasm port is the port of the wasm server
    /// ### Makepad
    /// if you use Makepad, it will use `8010` as the default port
    /// ### Example
    /// ```rust
    /// let mut app = app(Target::Makepad)
    /// .wasm()
    /// .port(4568);
    /// ```
    pub fn port(mut self, port: u16) -> Self {
        self.port.replace(port);
        self
    }
    /// build back to `CompilerBuilder`
    pub fn build(mut self) -> CompilerBuilder {
        self.parent.wasm = true;
        self.parent.wasm_check = self.check;
        self.parent.wasm_fresh = self.fresh;
        self.parent.wasm_port = self.port;
        self.parent
    }
}

impl From<CompilerBuilder> for WasmBuilder {
    fn from(value: CompilerBuilder) -> Self {
        Self {
            parent: value,
            check: false,
            fresh: true,
            port: None,
        }
    }
}
