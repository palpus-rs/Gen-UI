use std::collections::HashMap;

use gen_converter::model::TemplateModel;
use gen_parser::{PropsKey, Value};

pub fn id() -> impl FnMut(&mut TemplateModel, HashMap<PropsKey, Value>) -> () {
    return |t_model, id_style| {
        id_style.into_iter().for_each(|(k, v)| {
            t_model.push_prop(k, v);
        });
    };
}
