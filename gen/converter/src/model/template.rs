use std::borrow::Cow;

use gen_parser::{ASTNodes, Props};

use crate::keyword::KeyWords;

use super::{ ConvertProp, ModelAction};

/// # GenUI组件模型
/// 它用于完整的表示一个.gen文件，因为.gen文件就是一个完整的组件，所以这个模型也是一个完整的组件
/// 组件严格意义上并没有区分
/// 在GenUI中甚至没有内置组件的概念（因为GenUI是可插拔的，如果你想要转化为Makepad，那么内置组件就是Makepad的内置组件）
/// 
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TemplateModel {
    /// 组件的唯一标识符
    /// 它可以与文件模型的唯一标识符组合
    /// 以此来在不同的文件中区分相同的组件
    special: Option<String>,
    /// class是一个数组，一个组件模型可以有多个class
    /// 这些class指向style中的样式
    /// 这些class可以是动态绑定的
    class: Option<Vec<String>>,
    /// id是一个字符串，一个组件模型只能有一个id
    /// 这个id不能是动态绑定的，只能是静态的
    id: Option<String>,
    /// 组件的名字，这个名字标识了组件应该如何在.gen文件中书写
    /// 例如，如果组件名字是`button`，那么在.gen文件中书写`<button></button>`就是正确的
    name: String,
    /// 组件的属性
    /// 无论是自定义组件还是内置组件，都有属性，只是有些被显示的书写在.gen文件中，有些被隐藏在组件内部
    /// 对GenUI来说，不需要关心这些属性的默认值是什么，这些都由插入的转化框架来决定
    /// 但是，GenUI需要关心这些属性是否是绑定的还是静态的
    /// 对于自定义组件来说，这些属性却是一个重要的部分，因为这些属性需要被外部传入
    props: Option<Props>,
    /// 组件的事件
    /// 事件也可以被认为是组件状态
    /// 例如，一个按钮组件，它有一个点击事件，那么这个按钮被点击时，这个事件就会被触发，也就是这个按钮进入了点击状态
    /// GenuI中事件实际上是由外部影响的
    /// 例如，在组件中有一个按钮，当这个按钮被点击时，这个按钮会激发组件的点击事件并把这个事件传递给外部（连带参数）
    /// 外部可以根据这个事件来做一些事情
    /// 语法：`<define_component @click="do_click" />`
    /// 对于定义组件时就需要相应的使用Rust编写
    /// ```rust
    /// #[derive(Debug, Clone, PartialEq, Event)]
    /// pub enum Events{
    ///     #[name = "click"]
    ///     Clicked(//内部给到外部的参数),
    /// }
    /// ```
    events: Option<>,

    // props: Option<ConvertProp>,
    // props: Option<>
    // /// tag actions
    // actions: Option<Vec<ModelAction>>,
    // /// children tag model
    // children: Option<Vec<TemplateModel>>,
    // /// is root
    // is_root: bool,
    // /// inherits
    // inherits: Option<String>,
}

// impl TemplateModel {
//     pub fn is_component(&self) -> bool {
//         self.tag_name.eq("component") && self.is_inherit()
//     }
//     pub fn is_inherit(&self) -> bool {
//         self.inherits.is_some()
//     }
//     pub fn set_inherit(&mut self, inherits: &str) {
//         let _ = self.inherits.replace(inherits.to_string());
//     }
//     pub fn get_inherit(&self) -> Option<&String> {
//         self.inherits.as_ref()
//     }
//     pub fn get_contexts(&self) -> Option<&Vec<String>> {
//         self.contexts.as_ref()
//     }
//     pub fn set_contexts(&mut self, context: &Vec<String>) -> () {
//         self.contexts.replace(context.clone());
//     }
//     pub fn has_contexts(&self) -> bool {
//         self.contexts.is_some()
//     }
//     pub fn has_links(&self) -> bool {
//         self.has_contexts() || self.has_special()
//     }
//     pub fn get_links(&self) -> Option<Vec<String>> {
//         match (self.has_special(), self.has_contexts()) {
//             (true, true) => {
//                 let mut res = self.get_contexts().unwrap().clone();
//                 res.push(self.get_special().unwrap().to_string());
//                 Some(res)
//             }
//             (true, false) => Some(vec![self.get_special().unwrap().to_string()]),
//             (false, true) => Some(self.get_contexts().unwrap().clone()),
//             (false, false) => None,
//         }
//     }
//     pub fn get_special(&self) -> Option<&String> {
//         self.special.as_ref()
//     }
//     pub fn set_special(&mut self, special: &str) {
//         if !special.is_empty() {
//             self.special.replace(special.to_string());
//         }
//     }
//     pub fn has_special(&self) -> bool {
//         self.special.is_some()
//     }
//     pub fn get_props(&self) -> Option<&ConvertProp> {
//         self.props.as_ref()
//     }
//     pub fn has_props(&self) -> bool {
//         self.props.is_some()
//     }
//     // pub fn get_bind_props(&self) -> Option<ConvertProp> {
//     //     if let Some(props) = self.get_props() {
//     //         let mut res = vec![];
//     //         for prop in props {
//     //             match prop {
//     //                 ::Bind(_, _) => {
//     //                     res.push(prop.clone());
//     //                 }
//     //                 _ => {}
//     //             }
//     //         }
//     //         if res.is_empty() {
//     //             None
//     //         } else {
//     //             Some(res)
//     //         }
//     //     } else {
//     //         None
//     //     }
//     // }
//     // pub fn push_prop(&mut self, item: ConvertProp) -> () {
//     //     match &mut self.props {
//     //         Some(props) => props.push(item),
//     //         None => {
//     //             let _ = self.props.replace(vec![item]);
//     //         }
//     //     };
//     // }
//     // pub fn set_props(&mut self, props: Vec<ConvertProp>) -> () {
//     //     let _ = self.props.replace(props);
//     // }
// }

// pub fn handle_template(template: &ASTNodes) -> (TemplateModel, Props) {
//     if let ASTNodes::Tag(t) = template {
//         let mut model = TemplateModel::default();
//         let mut props = Props::default();

//         if t.has_props() {
//             for (k, v) in t.get_props().unwrap() {
//                 // check keyword
//                 match KeyWords::try_from(k.name()) {
//                     Ok(prop_name) => {
//                         let _ = prop_name.value_prop(v, &mut model);
//                     }
//                     Err(e) => panic!("{}", e.to_string()),
//                 }
//             }

//             return (model, props);
//         }
        
//     }
//     panic!("handle_template fn can only handle ASTNodes::Tag")
// }

// /// acturally if the handle_tag() function can run
// /// it must have ConvertScript
// fn handle_tag(
//     t: &Tag,
//     styles: Option<&ConvertStyle>,
//     is_ref: bool,
// ) -> (MakepadModel, Vec<BindProp>, Vec<BindAction>) {
//     // 1. uppercase the first title case of the tag
//     // if can not upper - panic!
//     let tag_name = snake_to_camel(t.get_name());
//     // 2. add `<` `>` surround the tag
//     // 3. add `{` `}` after the tag
//     let mut tag_model = MakepadModel::new(&tag_name, is_ref);
//     let mut binds = Vec::new();
//     let mut actions: Vec<BindAction> = Vec::new();
//     // check props
//     if t.has_props() {
//         let mut has_bind = false;
//         let mut has_action = false;
//         for prop in t.get_props().unwrap() {
//             match PropRole::try_from((tag_name.as_str(), prop)) {
//                 Ok(p) => {
//                     // dbg!(&p);
//                     match p {
//                         PropRole::Normal(_, _) => tag_model.push_prop(p),
//                         PropRole::Bind(k, v) => {
//                             has_bind = true;
//                             binds.push((tag_name.clone(), String::new(), (k, v)));
//                         }
//                         PropRole::Function(k, v) => {
//                             has_action = true;
//                             actions.push((
//                                 tag_name.clone(),
//                                 String::new(),
//                                 (k, v.get_fn_key().to_string()),
//                             ));
//                             // tag_model.push_action(p)
//                         }
//                         PropRole::Context(c) => {
//                             c.into_iter().for_each(|x| tag_model.push_context(x));
//                         }
//                         PropRole::Special(s) => tag_model.set_special(s),
//                         PropRole::Component(c) => tag_model.set_inherit(Some(c)),
//                     }
//                 }
//                 Err(e) => panic!("{}", e.to_string()),
//             };
//         }

//         // add special for all binds
//         if has_bind {
//             match tag_model.get_special() {
//                 Some(special) => {
//                     let _ = binds
//                         .iter_mut()
//                         .for_each(|bind| bind.1 = special.to_string());
//                 }
//                 None => {
//                     if !tag_model.is_component() {
//                         dbg!(&tag_model);
//                         panic!(
//                             "the widget(expcet component) which has binds need to add special id"
//                         );
//                     }
//                 }
//             }
//         }
//         if has_action {
//             match tag_model.get_special() {
//                 Some(special) => {
//                     let _ = actions
//                         .iter_mut()
//                         .for_each(|action| action.1 = special.to_string());
//                 }
//                 None => panic!("the widget which has actions need to add special id"),
//             }
//         }
//     }

//     // have styles
//     // true: do not need to associate with styles
//     // false: need if style exists
//     if styles.is_some() {
//         let styles = styles.unwrap();
//         // when special and context means link , need to patch
//         if let Some(links) = tag_model.get_links() {
//             for link in links {
//                 if let Some(sheets) = styles.get(&Cow::Borrowed(link.as_str())) {
//                     let _ = sheets.iter().try_for_each(|kv| {
//                         PropRole::try_from((&tag_name, kv)).map(|item| tag_model.push_prop(item))
//                     });
//                 }
//             }
//         }
//     }

//     // children
//     if t.has_children() {
//         for child_node in t.get_children().unwrap() {
//             match child_node {
//                 ASTNodes::Tag(child) => {
//                     let (child_model, child_binds, child_actions) =
//                         handle_tag(*&child, styles, false);
//                     tag_model.push_child(child_model);
//                     binds.extend(child_binds);
//                     actions.extend(child_actions);
//                 }
//                 ASTNodes::Comment(_) => (),
//                 ASTNodes::Style(_) => panic!("{}", "cannot write styles in template node"),
//             }
//         }
//     }

//     (tag_model, binds, actions)
// }
