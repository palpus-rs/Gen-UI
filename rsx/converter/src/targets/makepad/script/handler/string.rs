use parser::Value;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;

pub fn handle_string(init: TokenStream) -> Result<Value, syn::Error> {
    syn::parse2::<syn::LitStr>(init.clone())
        .and_then(|s| Ok(Value::String(s.value())))
        .or_else(|_| {
            syn::parse2::<syn::ExprCall>(init.clone()).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &expr.args[0]
                {
                    Ok(Value::String(lit_str.value()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected string literal"))
                }
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprMethodCall>(init).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &*expr.receiver
                {
                    Ok(Value::String(lit_str.value()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected string literal"))
                }
            })
        })
}
