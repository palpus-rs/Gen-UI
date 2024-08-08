use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, Stmt};

use super::TypeGetter;

impl TypeGetter for Option<&Vec<Stmt>> {
    fn get(&self, var: &str) -> Option<&syn::PatType> {
        self.and_then(|stmts| stmts.iter().find_map(|stmt| stmt.get(var)))
    }

    fn ty(&self, var: &str) -> Option<String> {
        self.and_then(|stmts| stmts.iter().find_map(|stmt| stmt.ty(var)))
    }
}

/// get all let statements and convert them to self, depending on the check_list
/// if check_list is None, do nothing
/// example:
/// `let a = 1;` -> `self.a = 1;`
pub fn let_to_self(stmts: &Vec<Stmt>, check_list: Option<HashSet<String>>) -> Option<TokenStream> {
    if let Some(check_list) = check_list {
        let let_stmts = stmts.iter().fold(TokenStream::new(), |mut acc, stmt| {
            if let Stmt::Local(local) = stmt {
                // get the variable name, get variable init, then use quote to generate the new statement
                // maybe local.pat is Pat::Ident or Pat::Type
                let var_init = match &local.pat {
                    syn::Pat::Ident(ident) => {
                        let var = ident.ident.to_string();
                        Some((var, local.init.as_ref().unwrap().expr.to_token_stream()))
                    }
                    syn::Pat::Type(ident_ty) => {
                        if let syn::Pat::Ident(ident) = &*ident_ty.pat {
                            Some((
                                ident.ident.to_string(),
                                local.init.as_ref().unwrap().expr.to_token_stream(),
                            ))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                // if var_init is None, do nothing, if is Some, extend the acc
                if let Some((var, init)) = var_init {
                    if check_list.contains(&var) {
                        let var = parse_str::<TokenStream>(&var).unwrap();

                        acc.extend(quote! {self.#var = #init;});
                    }
                }
            }
            acc
        });

        return Some(let_stmts);
    }

    None
}
