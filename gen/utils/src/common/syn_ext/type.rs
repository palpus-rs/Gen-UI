use quote::ToTokens;
use syn::{Pat, PatType, Stmt, Type};

#[allow(dead_code)]
pub trait TypeGetter {
    /// Get the type of the variable
    fn get(&self, var: &str) -> Option<&PatType>;
    fn has(&self, var: &str) -> bool {
        self.get(var).is_some()
    }
    fn ty(&self, var: &str) -> Option<String>;
}

impl TypeGetter for Stmt {
    fn get(&self, var: &str) -> Option<&PatType> {
        if let Stmt::Local(local) = self {
            if let Pat::Type(ident_ty) = &local.pat {
                if let Pat::Ident(ident) = &*ident_ty.pat {
                    if ident.ident.eq(var) {
                        return Some(ident_ty);
                    }
                }
            }
        }
        None
    }

    fn ty(&self, var: &str) -> Option<String> {
        self.get(var).map(|x| {
            if let Type::Path(path) = &*x.ty {
                path.path.segments.to_token_stream().to_string()
            } else {
                panic!("Type not supported")
            }
        })
    }
}
