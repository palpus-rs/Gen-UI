use parser::Value;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn handle_f64(init: TokenStream) -> Result<Value, syn::Error> {
    syn::parse2::<syn::LitFloat>(init.clone())
        .and_then(|f| Ok(Value::Double(f.base10_parse::<f64>().unwrap())))
        .or_else(|_| {
            syn::parse2::<syn::Lit>(init.clone()).and_then(|f| match f {
                syn::Lit::Int(i) => Ok(Value::Double(i.base10_parse::<f64>().unwrap())),
                syn::Lit::Float(f) => Ok(Value::Double(f.base10_parse::<f64>().unwrap())),
                _ => Err(syn::Error::new(f.span(), "expected float literal")),
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprCall>(init.clone()).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Float(lit_float),
                    ..
                }) = &expr.args[0]
                {
                    Ok(Value::Double(lit_float.base10_parse::<f64>().unwrap()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected float literal"))
                }
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprMethodCall>(init).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Float(lit_float),
                    ..
                }) = &*expr.receiver
                {
                    Ok(Value::Double(lit_float.base10_parse::<f64>().unwrap()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected float literal"))
                }
            })
        })
}

pub fn handle_usize(init: TokenStream) -> Result<Value, syn::Error> {
    syn::parse2::<syn::LitInt>(init.clone())
        .and_then(|f| Ok(Value::USize(f.base10_parse::<usize>().unwrap())))
        .or_else(|_| {
            syn::parse2::<syn::Lit>(init.clone()).and_then(|f| match f {
                syn::Lit::Int(i) => Ok(Value::USize(i.base10_parse::<usize>().unwrap())),
                syn::Lit::Float(f) => Ok(Value::USize(f.base10_parse::<usize>().unwrap())),
                _ => Err(syn::Error::new(f.span(), "expected usize literal")),
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprCall>(init.clone()).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) = &expr.args[0]
                {
                    Ok(Value::USize(lit_int.base10_parse::<usize>().unwrap()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected usize literal"))
                }
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprMethodCall>(init).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) = &*expr.receiver
                {
                    Ok(Value::USize(lit_int.base10_parse::<usize>().unwrap()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected usize literal"))
                }
            })
        })
}

pub fn handle_isize(init: TokenStream) -> Result<Value, syn::Error> {
    syn::parse2::<syn::LitInt>(init.clone())
        .and_then(|f| Ok(Value::ISize(f.base10_parse::<isize>().unwrap())))
        .or_else(|_| {
            syn::parse2::<syn::Lit>(init.clone()).and_then(|f| match f {
                syn::Lit::Int(i) => Ok(Value::ISize(i.base10_parse::<isize>().unwrap())),
                syn::Lit::Float(f) => Ok(Value::ISize(f.base10_parse::<isize>().unwrap())),
                _ => Err(syn::Error::new(f.span(), "expected isize literal")),
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprCall>(init.clone()).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) = &expr.args[0]
                {
                    Ok(Value::ISize(lit_int.base10_parse::<isize>().unwrap()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected isize literal"))
                }
            })
        })
        .or_else(|_| {
            syn::parse2::<syn::ExprMethodCall>(init).and_then(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) = &*expr.receiver
                {
                    Ok(Value::ISize(lit_int.base10_parse::<isize>().unwrap()))
                } else {
                    Err(syn::Error::new(expr.span(), "expected isize literal"))
                }
            })
        })
}

// pub fn handle_struct(init: TokenStream)->Result<Value,syn::Error>{
//     match syn::parse2::<syn::ItemStruct>(init){
//         Ok(s) => Ok(Value::Struct(s.to_token_stream().to_string())),
//         Err(e) => Err(e),
//     }
// }

