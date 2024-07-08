use super::compiler::CompilerBuilder;

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
    pub fn check(mut self) -> Self {
        self.check = true;
        self
    }
    pub fn no_fresh(mut self) -> Self {
        self.fresh = false;
        self
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port.replace(port);
        self
    }
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
