// use std::collections::HashMap;

// use gen_converter::model::TemplateModel;
// use gen_parser::{PropsKey, Value};
// use gen_utils::common::{snake_to_camel, token_tree_ident, token_tree_punct_alone};
// use proc_macro2::TokenTree;

// use crate::error::MkError;

// // use super::Widget;

// /// generate makepad dsl ast for define widget
// pub fn ast(is_app: bool, model: &TemplateModel) {
//     let id = model.get_id();
//     let name = {
//         Widget::from(
//             snake_to_camel(model.get_name())
//                 .ok_or(MkError::HandleChar(
//                     "widget name convert failed".to_string(),
//                 ))
//                 .unwrap()
//                 .as_str(),
//         )
//     };
//     let unbind_props = model.get_unbind_props();
// }

// /// generate makepad widget prop ast
// /// ```makepad
// /// prop: xxx
// /// ```
// pub fn prop(widget: &Widget, unbind_props:Option<&HashMap<PropsKey,Value>>)->Vec<TokenTree> {
//     let mut ast = vec![];
//     if let Some(props) = unbind_props {
//        ast.extend(widget.props(props));
//     }
//     ast
// }

// /// generate makepad widget ast
// /// ```makepad
// /// id = <Widget>{
// ///     prop: xxx
// /// }
// /// ```
// pub fn widget(
//     id: Option<&String>,
//     name: &str,
//     props: Vec<TokenTree>,
//     is_prop: bool,
// ) -> Vec<TokenTree> {
//     let mut ast = vec![];
//     let sign = if is_prop {
//         token_tree_punct_alone(':')
//     } else {
//         token_tree_punct_alone('=')
//     };

//     // add id if exist
//     if let Some(id) = id {
//         ast.push(token_tree_ident(&id));
//         ast.push(sign);
//     }

//     // add widget name: <widget_name>
//     ast.push(token_tree_punct_alone('<'));
//     ast.push(token_tree_ident(name));
//     ast.push(token_tree_punct_alone('>'));

//     // add props
//     ast.extend(props);
//     ast
// }
