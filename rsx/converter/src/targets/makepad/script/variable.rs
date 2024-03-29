use std::fmt::Display;

use parser::Value;
use quote::{quote, ToTokens};
use syn::{LocalInit, Type};

use crate::targets::makepad::handler::{handle_expr_default};

use super::handler::{handle_bool, handle_f64, handle_isize, handle_string, handle_usize};

/// 编译时设置（转换时设置）（init）
/// ## 优点
///
/// - 性能： 在makepad应用运行时，由于所有的值已经在编译时被确定并填充，这减少了运行时的计算开销。对于那些不会改变的值，这是一个性能上的优势。
/// - 简化： 不需要额外的逻辑来在运行时解析和绑定值，减少了代码的复杂度。
/// ## 缺点
///
/// - 灵活性低： 一旦值在编译时被绑定，如果想要改变这个值，就需要重新编译整个应用，这减少了灵活性。
/// - 可维护性： 对于大型项目，如果有大量的值需要在编译时绑定，管理这些值可能会变得复杂。
#[derive(Debug, Clone, PartialEq)]
pub struct NodeVariable {
    name: String,
    ty: Type,
    init: Option<LocalInit>,
    is_mut: bool,
}

#[allow(dead_code)]
impl NodeVariable {
    pub fn new(name: &str, ty: &Type, init: Option<LocalInit>, is_mut: bool) -> Self {
        Self::new_unwrap(name.to_string(), ty.clone(), init, is_mut)
    }
    pub fn new_unwrap(name: String, ty: Type, init: Option<LocalInit>, is_mut: bool) -> Self {
        NodeVariable {
            name,
            ty,
            init,
            is_mut,
        }
    }
    pub fn is_mut(&self) -> bool {
        self.is_mut
    }
    pub fn init_to_string(&self) -> Option<String> {
        match &self.init {
            Some(init) => {
                let expr = &*init.expr;
                let expr_token = quote! {#expr}.to_string();
                Some(expr_token)
            }
            None => None,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_ty(&self) -> &Type {
        &self.ty
    }
    pub fn get_ty_str(&self) -> String {
        self.ty.to_token_stream().to_string()
    }
    pub fn get_init(&self) -> Option<&LocalInit> {
        self.init.as_ref()
    }
    // ensure init exist (Some)
    // pub fn init_to_mk_value(&self) -> Result<MakepadPropValue, syn::Error> {
    //     match &self.init {
    //         Some(init) => {
    //             let expr = &*init.expr;
    //             let input = quote! {#expr};
    //             let ty = self.get_ty().to_token_stream().to_string();

    //             let value = match ty.as_str() {
    //                 "String" | "& str" => {
    //                     let s = syn::parse2::<syn::LitStr>(input)?;
    //                     MakepadPropValue::String(s.value())
    //                 }
    //                 "f64" => {
    //                     let f = syn::parse2::<syn::LitFloat>(input)?;
    //                     MakepadPropValue::F64(f.base10_parse::<f64>().unwrap())
    //                 }
    //                 "Size" => {
    //                     let s = syn::parse2::<Size>(input)?;
    //                     MakepadPropValue::Size(s)
    //                 }
    //                 "Color" => {
    //                     let c = syn::parse2::<Color>(input)?;
    //                     MakepadPropValue::Color(c)
    //                 }
    //                 "bool" => {
    //                     let b = syn::parse2::<syn::LitBool>(input)?;
    //                     MakepadPropValue::Bool(b.value)
    //                 }
    //                 "Margin" => {
    //                     let m = syn::parse2::<Margin>(input)?;
    //                     MakepadPropValue::Margin(m)
    //                 }
    //                 "Padding" => {
    //                     let p = syn::parse2::<Padding>(input)?;
    //                     MakepadPropValue::Padding(p)
    //                 }
    //                 "Align" => {
    //                     let a = syn::parse2::<Align>(input)?;
    //                     MakepadPropValue::Align(a)
    //                 }
    //                 "Flow" => {
    //                     let f = syn::parse2::<Flow>(input)?;
    //                     MakepadPropValue::Flow(f)
    //                 }
    //                 "DVec2" => {
    //                     let dv = syn::parse2::<DVec2>(input)?;
    //                     MakepadPropValue::DVec2(dv)
    //                 }
    //                 "Optimize" => {
    //                     // template just write for ViewOptimize
    //                     let o = syn::parse2::<ViewOptimize>(input)?;
    //                     MakepadPropValue::Optimize(Optimize::View(o))
    //                 }
    //                 "EventOrder" => {
    //                     let eo = syn::parse2::<EventOrder>(input)?;
    //                     MakepadPropValue::EventOrder(eo)
    //                 }
    //                 "Cursor" => {
    //                     let c = syn::parse2::<Cursor>(input)?;
    //                     MakepadPropValue::Cursor(c)
    //                 }
    //                 _ => {
    //                     return Err(syn::Error::new_spanned(
    //                         self.get_ty(),
    //                         "unsupported type for init_to_mk_value",
    //                     ))
    //                 }
    //             };
    //             Ok(value)
    //         }
    //         None => panic!("init is None"),
    //     }
    // }
}

impl Display for NodeVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.get_ty();
        let ty_token = quote! {#ty}.to_string();

        f.write_fmt(format_args!("{}: {}", self.name, ty_token))
    }
}

/// init value mut be exist
impl From<NodeVariable> for Value {
    fn from(value: NodeVariable) -> Self {
        let expr = value
            .init
            .unwrap_or_else(|| panic!("init cannot be None"))
            .expr;
        let init = quote! {#expr};
        let ty = value.ty.to_token_stream().to_string();
        match ty.as_str() {
            "String" | "String :: from" => handle_string(init).unwrap(),
            "& str" => {
                panic!("immutable var type:&str , expected String")
            }
            "f64" => handle_f64(init).unwrap(),
            "bool" => handle_bool(init).unwrap(),
            "usize" | "u8" | "u16" | "u32" | "u64" | "int" => handle_usize(init).unwrap(),
            "isize" | "i8" | "i16" | "i32" | "i64" => handle_isize(init).unwrap(),
            other => {
                // 对other进行处理识别匹配default
                if other.contains(":: default"){
                    // 是Struct使用了default()方法，处理为：
                    // #[live]
                    // pub props: MyProps,       
                    // Value::Struct()
                    // dbg!(init);
                    // return handle_struct(init).unwrap();
                    return handle_expr_default(init).unwrap();
                }


                panic!("unexpected value type: {:?}", &ty);
            }
        }
    }
}
