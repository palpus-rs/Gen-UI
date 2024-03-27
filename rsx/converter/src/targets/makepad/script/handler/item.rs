use parser::Value;
use proc_macro2::TokenStream;
use quote::quote;

pub fn handle_expr_default(init: TokenStream)->Result<Value,syn::Error>{
    Ok(Value::Struct(quote!(#init).to_string()))
}