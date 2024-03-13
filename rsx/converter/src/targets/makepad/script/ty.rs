use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, Expr, Ident, LocalInit, Path, PathArguments, PathSegment, Type,
    TypePath,
};

pub fn parse_init_type(init: Option<LocalInit>) -> (Type, Option<LocalInit>) {
    let expr = *init.clone().unwrap().expr;
    handle_expr(expr, init)
}

fn handle_expr(expr: Expr, init: Option<LocalInit>) -> (Type, Option<LocalInit>) {
    match expr {
        syn::Expr::Array(_) => todo!(),
        syn::Expr::Assign(_) => todo!(),
        syn::Expr::Async(_) => todo!(),
        syn::Expr::Await(_) => todo!(),
        syn::Expr::Binary(_) => todo!(),
        syn::Expr::Block(_) => todo!(),
        syn::Expr::Break(_) => todo!(),
        syn::Expr::Call(call) => {
            // dbg!(&call);
            // handle func
            let func = *call.func;
            handle_expr(func, init)
        }
        syn::Expr::Cast(_) => todo!(),
        syn::Expr::Closure(_) => todo!(),
        syn::Expr::Const(_) => todo!(),
        syn::Expr::Continue(_) => todo!(),
        syn::Expr::Field(_) => todo!(),
        syn::Expr::ForLoop(_) => todo!(),
        syn::Expr::Group(_) => todo!(),
        syn::Expr::If(_) => todo!(),
        syn::Expr::Index(_) => todo!(),
        syn::Expr::Infer(_) => todo!(),
        syn::Expr::Let(_) => todo!(),
        syn::Expr::Lit(lit) => match lit.lit {
            syn::Lit::Str(s) => (ty_string(), init),
            syn::Lit::ByteStr(bs) => todo!(),
            syn::Lit::Byte(b) => todo!(),
            syn::Lit::Char(c) => todo!(),
            syn::Lit::Int(i) => todo!(),
            syn::Lit::Float(f) => todo!(),
            syn::Lit::Bool(b) => todo!(),
            syn::Lit::Verbatim(v) => todo!(),
            _ => panic!("unexpect lit type in this script"),
        },
        syn::Expr::Loop(_) => todo!(),
        syn::Expr::Macro(_) => todo!(),
        syn::Expr::Match(_) => todo!(),
        syn::Expr::MethodCall(_) => todo!(),
        syn::Expr::Paren(_) => todo!(),
        syn::Expr::Path(path) => {
            // path is the type
            let ty = Type::Path(TypePath {
                qself: path.qself,
                path: path.path,
            });

            (ty, init)
        }
        syn::Expr::Range(_) => todo!(),
        syn::Expr::Reference(_) => todo!(),
        syn::Expr::Repeat(_) => todo!(),
        syn::Expr::Return(_) => todo!(),
        syn::Expr::Struct(_) => todo!(),
        syn::Expr::Try(_) => todo!(),
        syn::Expr::TryBlock(_) => todo!(),
        syn::Expr::Tuple(_) => todo!(),
        syn::Expr::Unary(_) => todo!(),
        syn::Expr::Unsafe(_) => todo!(),
        syn::Expr::Verbatim(_) => todo!(),
        syn::Expr::While(_) => todo!(),
        syn::Expr::Yield(_) => todo!(),
        _ => todo!(),
    }
}

/// generate syn::Type for String
fn ty_string() -> Type {
    let seg = PathSegment {
        ident: Ident::new("String", Span::call_site()),
        arguments: PathArguments::None,
    };
    let mut p = Punctuated::new();
    p.push(seg);
    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: p,
        },
    })
}

#[cfg(test)]
mod test_ty {
    use core::str;

    use quote::quote;
    use syn::{Expr, Type};

    #[test]
    fn str() {
        // rs:93] ast = Expr::Lit {
        //     attrs: [],
        //     lit: Lit::Str {
        //         token: "1",
        //     },
        // }
        let str1: &str = "1";
        let str2: String = "1".to_string();
        let str3 = String::from("1");
        let str4 = "1";
        let str1_t = quote! { String };
        let ast: Type = syn::parse2(str1_t).unwrap();
        dbg!(ast);
    }
}
