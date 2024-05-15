mod prop;
mod prop_ptr;

use gen_utils::common::{token_stream_to_tree, trees_to_token_stream};
use proc_macro2::{Group, TokenStream, TokenTree};
pub use prop::ButtonProps;
pub use prop_ptr::ButtonPropPtr;

struct EventVisitor {
    replace: TokenStream,
    fields: Vec<String>,
}

impl EventVisitor {
    pub fn new(replace: TokenStream, fields: Vec<String>) -> Self {
        Self { replace, fields }
    }
    fn visit(&self, target: Vec<TokenTree>) -> (Vec<TokenTree>, Vec<String>) {
        let mut res = target.clone();
        let mut indexs = Vec::new();
        // 需要更新的props
        let mut updates = Vec::new();
        target.iter().enumerate().for_each(|(index, item)| {
            if let TokenTree::Group(group) = item {
                let (handled, c_updates) = self.visit(token_stream_to_tree(group.stream()));
                updates.extend(c_updates);
                res[index] = TokenTree::Group(Group::new(
                    group.delimiter(),
                    trees_to_token_stream(handled),
                ));
            }
            if let TokenTree::Ident(ident) = item {
                if self.fields.contains(&ident.to_string()) {
                    // 收集需要更改的索引
                    indexs.push(index);
                }
            }
        });
        for index in indexs.iter().rev() {
            updates.push(res[*index].to_string());
            res.splice(*index..*index, self.replace.clone());
        }

        (res, updates)
    }
}
