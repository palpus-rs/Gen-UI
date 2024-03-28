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
use syn::{Field, Type};
pub use widget::*;
pub use item::*;

pub struct MakepadFieldConverter;

impl MakepadFieldConverter {
    pub fn convert(origin: &mut Field)-> (){
        match origin.ty {
            Type::Path(_) => {
                
            },
            _ => {
                // 添加#[live]宏后放行
                // todo!("waiting to do other makepad field type")
            },
        }
    }
}