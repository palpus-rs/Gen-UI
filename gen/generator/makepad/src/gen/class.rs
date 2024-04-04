use std::collections::HashMap;

use gen_converter::model::TemplateModel;
use gen_parser::{PropsKey, Value};

/// 对TemplateModel(模板部分)的class的处理的策略器
/// 目前只处理class为非绑定值的情况
/// class只能是字符串形式
pub fn class() -> impl FnMut(&mut TemplateModel, HashMap<PropsKey, Value>) -> () {
    return |model, styles| {
        // 将style放到model中的prop中即可
        styles.into_iter().for_each(|(k, v)| {
            let _ = model.push_prop(k, v);
        });
    };
}
