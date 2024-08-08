use syn::Stmt;

use super::TypeGetter;

impl TypeGetter for Option<&Vec<Stmt>> {
    fn get(&self, var: &str) -> Option<&syn::PatType> {
        self.and_then(|stmts| stmts.iter().find_map(|stmt| stmt.get(var)))
    }

    fn ty(&self, var: &str) -> Option<String> {
        self.and_then(|stmts| stmts.iter().find_map(|stmt| stmt.ty(var)))
    }
}
