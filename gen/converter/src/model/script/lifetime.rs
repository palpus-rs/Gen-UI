use proc_macro2::TokenStream;
/// GenUI内置组件生命周期事件
/// 目前只设置两种事件
#[derive(Debug, Clone, Default)]
pub struct LifeTime {
    /// 启动事件
    pub startup: Option<syn::StmtMacro>,
    /// 关闭事件
    pub shutdown: Option<syn::StmtMacro>,
}

impl LifeTime {
    pub fn set_startup(&mut self, startup: syn::StmtMacro) {
        let _ = self.startup.replace(startup);
    }
    pub fn set_shutdown(&mut self, shutdown: syn::StmtMacro) {
        let _ = self.shutdown.replace(shutdown);
    }
    /// 获取启动事件中的执行的代码
    pub fn startup_token(&self) -> Option<&TokenStream> {
        match &self.startup {
            Some(startup) => Some(&startup.mac.tokens),
            None => None,
        }
    }
    pub fn shutdown_token(&self) -> Option<&TokenStream> {
        match &self.shutdown {
            Some(shutdown) => Some(&shutdown.mac.tokens),
            None => None,
        }
    }
}
