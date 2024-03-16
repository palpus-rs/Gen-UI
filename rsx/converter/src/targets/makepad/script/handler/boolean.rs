use parser::Value;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;

pub fn handle_bool(init: TokenStream) -> Result<Value, syn::Error> {
    syn::parse2::<syn::LitBool>(init.clone())
        .and_then(|b| Ok(Value::Bool(b.value())))
        .or_else(|_| {
            syn::parse2::<syn::Lit>(init.clone()).and_then(|b| match b {
                syn::Lit::Bool(lb) => Ok(Value::Bool(lb.value())),
                _ => Err(syn::Error::new(b.span(), "expected bool literal")),
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprCall>(init.clone()).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Bool(lit_bool),
                    ..
                }) = &expr.args[0]
                {
                    Ok(Value::Bool(lit_bool.value()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected bool literal"))
                }
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprMethodCall>(init).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Bool(lit_bool),
                    ..
                }) = &*expr.receiver
                {
                    Ok(Value::Bool(lit_bool.value()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected bool literal"))
                }
            })
        })
}
