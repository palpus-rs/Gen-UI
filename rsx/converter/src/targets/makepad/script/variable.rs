use std::fmt::Display;

use quote::{quote, ToTokens};
use syn::{parse, LocalInit, Type};

use crate::targets::makepad::value::MakepadPropValue;
use syn::parse::Parse;

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
}

#[allow(dead_code)]
impl NodeVariable {
    pub fn new(name: &str, ty: &Type, init: Option<LocalInit>) -> Self {
        Self::new_unwrap(name.to_string(), *ty, init)
    }
    pub fn new_unwrap(name: String, ty: Type, init: Option<LocalInit>) -> Self {
        NodeVariable { name, ty, init }
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
    pub fn convert_init_to<T>(&self) -> T
    where
        T: Default + Parse,
    {
        match self.init {
            Some(init) => {
                let expr = &*init.expr;
                let init_token = quote! {#expr};
                syn::parse2::<T>(init_token).unwrap()
            }
            None => T::default(),
        }
    }
    pub fn to_makepad_value(&self) -> MakepadPropValue {
        match self.get_ty().to_token_stream().to_string().as_str() {
            "String" => ,
            "f64" =>,
            _ => panic!("not support type waiting to implement"),
        }
    }
}

impl Display for NodeVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.get_ty();
        let ty_token = quote! {#ty}.to_string();

        f.write_fmt(format_args!("#[rust] {}: {}", self.name, ty_token))
    }
}
