use syn::{Ident, Pat, Stmt};

#[allow(dead_code)]
pub trait VarGetter {
    fn get(&self, var: &str) -> Option<&Ident>;
    fn get_to_str(&self, var: &str) -> Option<String> {
        self.get(var).map(|x| x.to_string())
    }
    fn has(&self, var: &str) -> bool {
        self.get(var).is_some()
    }
}
impl VarGetter for Stmt {
    fn get(&self, var: &str) -> Option<&Ident> {
        if let Stmt::Local(local) = self {
            if let Pat::Ident(ident) = &local.pat {
                if ident.ident.eq(var) {
                    return Some(&ident.ident);
                }
            }
        }
        None
    }
}
