use gen_converter::model::TemplateModel;
use gen_utils::common::token_tree_ident;

/// generate makepad dsl ast for define widget
pub fn ast(is_app:bool, model: &TemplateModel){
    let id = model.get_id();

}

pub fn widget_prop(){}

pub fn widget(id: Option<&String>){
    let mut ast = vec![];
    if let Some(id) = id{
        ast.push(token_tree_ident(&id))
    }
}