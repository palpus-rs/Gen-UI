use parser::Value;

use crate::{
    error::Errors,
    targets::makepad::{value::MakepadPropValue, PropRole},
};

pub fn action_clicked(value: &Value) -> Result<PropRole, Errors> {
    match value.is_fn_and_get() {
        Some(f) => {
            let name = f.get_name();
            let params = f.get_params();
            Ok(PropRole::func(
                "clicked",
                MakepadPropValue::fn_without_value(name),
            ))
        }
        None => Err(Errors::PropConvertFail(format!(
            "{} can not convert @clicked closure",
            value
        ))),
    }
}
