mod boolean;
mod number;
mod string;
mod event;
mod widget;
mod item;

pub use boolean::*;
pub use number::*;
pub use string::*;
pub use event::*;
use syn::{Field, PathSegment, Type};
pub use widget::*;
pub use item::*;

use crate::utils::{macros::build_attr_macro, token_stream::build_path_segment};

pub struct MakepadFieldConverter;

impl MakepadFieldConverter {
    pub fn convert(origin: &mut Field)-> (){
        // 添加#[live]宏
        origin.attrs.push(build_attr_macro("live"));
        if let Type::Path(ty_path) = &mut origin.ty {
            let ident = &ty_path.path.segments[0].ident;
            if ident.to_string().eq("String"){
                ty_path.path.segments[0] = build_path_segment("RcStringMut");
            }
        }
        // todo!("waiting to do other makepad field type")
    }
}