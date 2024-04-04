use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{parse2, Block, Meta};

use crate::{error::Errors, model::Model};

/// 在GenUI中Rust脚本是直接写在`<script>`标签里的
/// 例如：`<script>println!("Hello, World!");</script>`
/// 其实最主要的目的在于为TemplateModel的prop_ptr和event_ptr赋值
/// 这个策略器中包含有多个闭包
/// - use 策略器： 处理use语句，获取所有use语句并返回TokenTree
/// - prop 策略器：获取带有`#[derive(Prop)]`的结构体，并返回TokenTree
/// - event 策略器：获取带有`#[derive(Event)]`的枚举，并返回TokenTree
/// - 生命周期策略器: 处理生命周期,目前只处理带有`on_startup!, on_shutdown!`标识的返回TokenTree
/// - 其他策略器： 用于处理其他的语句，例如`let a = 1;`等
pub fn script<F>(
    mut model: Model,
    mut use_f: F,
    mut prop_f: F,
    mut event_f: F,
    mut lifetime_f: F,
    mut other_f: F,
) -> Result<(), Errors>
where
    F: FnMut(&mut TokenStream) -> (),
{
    if !model.has_script() {
        return Err(Errors::StrategyNoScript);
    }

    let script = model.script.unwrap().to_origin();

    // let tt = parse2::<TokenTree>(script).unwrap();
    // use_f(script);
    // prop_f(script);
    // event_f(script);
    // lifetime_f(script);
    // other_f(script);

    Ok(())
}

fn split_script(block: Block) {
    let stmts = block.stmts;
    let mut uses = Vec::new();
    let mut props = Vec::new();
    for stmt in stmts {
        match stmt {
            syn::Stmt::Item(item) => {
                match item {
                    syn::Item::Use(use_item) => uses.push(use_item),
                    syn::Item::Struct(struct_item) => {
                        // 查看是否有`#[derive(Prop)]`的属性
                        if let Some(_) = struct_item.attrs.iter().find(|attr| {
                            if let Meta::List(list) = &attr.meta{
                                if list.path.is_ident("derive")&& list.tokens.to_string().contains("Prop"){
                                    return true;
                                }
                            }
                            return false;
                        }){
                            props.push(struct_item);
                        }
                    },
                    syn::Item::Enum(enum_item) => {
                        // 处理带有`#[derive(Event)]`的枚举
                    }
                    _ => {
                        // 其他情况
                    }
                }
            }
            
            _ => {
                // 其他情况
            }
        }
    }
}
