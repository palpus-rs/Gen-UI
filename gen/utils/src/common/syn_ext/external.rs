use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
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

/// get all let statements and convert them to self, depending on the check_list
/// if check_list is None, do nothing
/// example: 
/// `let a = 1;` -> `self.a = 1;` 
pub fn let_to_self(stmts: &Vec<Stmt>, check_list: Option<HashSet<String>>) -> Option<TokenStream> {
    if check_list.is_some(){
        dbg!(stmts, check_list);
        todo!();
    }
    // if let Some(check_list)= check_list {
    //     let let_stmts = stmts.iter().fold(TokenStream::new(), |mut acc, stmt|{
    //         if let Stmt::Local(local) = stmt {
    //             // get the variable name, get variable init, then use quote to generate the new statement
    //             // maybe local.pat is Pat::Ident or Pat::Type
    //             let (var, init) =  match &local.pat {
    //                 syn::Pat::Ident(ident) => {
    //                     let var = ident.ident.to_string();
    //                     let init = quote! {#ident.init};
    //                     (var, init)
    //                 }
    //                 syn::Pat::Type(ident_ty) => {
    //                     if let syn::Pat::Ident(ident) = &*ident_ty.pat {
    //                         ( ident.ident.to_string(), quote! {#ident_ty.ty} )
    //                     }
    //                 }
    //                 _ => {},
    //             }  
    
    //             if check_list.contains(&var) {
    //                 Some(quote! {
    //                     self.#var = #init;
    //                 })
    //             }
    
    //         }
    //     });
    
    //    return Some(
    //         let_stmts
    //    );
    // }

    None
    
}