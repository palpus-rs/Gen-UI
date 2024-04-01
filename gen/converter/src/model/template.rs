use std::collections::HashMap;

use gen_parser::{ASTNodes, PropertyKeyType, Props, PropsKey, Tag, Value};
use gen_traits::{event::Event, prop::Prop};

use ulid::Ulid;

use super::event::Callbacks;

/// # GenUI组件模型
/// 它用于完整的表示一个.gen文件，因为.gen文件就是一个完整的组件，所以这个模型也是一个完整的组件
/// 组件严格意义上并没有区分
/// 在GenUI中甚至没有内置组件的概念（因为GenUI是可插拔的，如果你想要转化为Makepad，那么内置组件就是Makepad的内置组件）
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemplateModel<E: Event, P: Prop> {
    /// 组件的唯一标识符
    /// 它可以与文件模型的唯一标识符组合
    /// 以此来在不同的文件中区分相同的组件
    special: Option<String>,
    /// class是一个数组，一个组件模型可以有多个class
    /// 这些class指向style中的样式
    /// 这些class可以是动态绑定的
    class: Option<Value>,
    /// id是一个字符串，一个组件模型只能有一个id
    /// 这个id不能是动态绑定的，只能是静态的
    id: Option<String>,
    /// 组件的名字，这个名字标识了组件应该如何在.gen文件中书写
    /// 例如，如果组件名字是`button`，那么在.gen文件中书写`<button></button>`就是正确的
    name: String,
    /// 组件的属性(由外部设置的属性)
    /// 无论是自定义组件还是内置组件，都有属性，只是有些被显示的书写在.gen文件中，有些被隐藏在组件内部
    /// 对GenUI来说，不需要关心这些属性的默认值是什么，这些都由插入的转化框架来决定
    /// 但是，GenUI需要关心这些属性是否是绑定的还是静态的
    /// 对于自定义组件来说，这些属性却是一个重要的部分，因为这些属性需要被外部传入
    props: Props,
    /// 组件的属性
    /// 这表示组件允许外部传入给内部的属性，需要使用GenUI的Prop宏进行标注
    /// 例如：
    /// ```rust
    /// #[derive(Debug, Clone, PartialEq, Prop)]
    /// pub enum Props{
    ///     text: String,
    ///     height: f64,
    /// }
    /// ```
    prop_ptr: P,
    /// 组件的事件的回调(是指组件内部允许暴露到外部的事件)
    /// 指的是外部组件当组件内部的事件被触发后，进行处理
    /// 回调的参数依赖于组件的事件提供给外部参数
    /// 回调表现为一个闭包或一个函数
    /// 语法：`<define_component @click="do_click" />`
    callbacks: Option<Callbacks>,
    /// 组件事件
    /// 事件也可以被认为是组件状态
    /// 由编写者决定，所以并不一定存在，但若存在则必须要使用GenUI的Event宏进行标注
    /// 例如，一个按钮组件，它有一个点击事件，那么这个按钮被点击时，这个事件就会被触发，也就是这个按钮进入了点击状态
    /// GenuI中事件实际上是由外部影响的
    /// 例如，在组件中有一个按钮，当这个按钮被点击时，这个按钮会激发组件的点击事件并把这个事件传递给外部（连带参数）
    /// 外部可以根据这个事件来做一些事情
    /// 对于定义组件时就需要相应的使用Rust编写
    /// ```rust
    /// #[derive(Debug, Clone, PartialEq, Event)]
    /// pub enum Events{
    ///     #[name("click")]
    ///     Clicked(//内部给到外部的参数),
    /// }
    /// ```
    event_ptr: E,
    /// 组件是否继承另一个组件
    /// 若继承另一个组件，当前组件就会自动继承另一个组件的所有属性和事件
    /// 注意这个属性只能是normal的不能是动态绑定的
    inherits: Option<String>,
    /// 当前组件是否为根组件
    /// 根组件指的是当前组件是整个.gen文件的组件树的根
    /// 在GenUI中，每个.gen文件都有一个根组件
    root: bool,
    /// 组件的子组件
    children: Option<Vec<TemplateModel<E, P>>>,
    /// 记录父组件的唯一标识符
    parent: Option<String>,
    // /// 组件的插槽(暂不开启)
    // /// 插槽的作用在于将子组件插入到指定的位置
    // /// 在GenUI中插槽使用<slot>标签进行指定
    // /// ```gen
    // /// // parent
    // /// <my-widget>
    // ///     <slot ptr="footer">
    // ///         <input></input>
    // ///     </slot>
    // /// </my-widget>
    // ///
    // /// // child
    // /// <component name="my-widget">
    // ///     <view></view>
    // ///     <view>
    // ///         <slot name="footer"></slot>
    // ///     </view>
    // /// </component>
    // /// ```
    // slots:
}

impl<E: Event, P: Prop> TemplateModel<E, P> {
    pub fn get_special(&self) -> Option<&String> {
        self.special.as_ref()
    }
    pub fn set_special(&mut self, special: &str) -> () {
        let _ = self.special.replace(special.to_string());
    }
    pub fn has_special(&self) -> bool {
        self.special.is_some()
    }
    pub fn get_class(&self) -> Option<&Value> {
        self.class.as_ref()
    }
    pub fn set_class(&mut self, class: Value) -> () {
        let _ = self.class.replace(class);
    }

    pub fn set_class_from_prop(&mut self) -> bool {
        let tmp_props = self.props.clone();

        match self.props.as_mut() {
            Some(props) => {
                // 目前解析器部分还不支持解析数组，只能采用绑定方式，并且可能未来也不打算支持
                // 支持直接在标签属性中解析数组可能会引发一些不好的编写习惯
                // let normal_remove_item = PropsKey::new("class", false, PropertyKeyType::Normal);
                // let bind_remove_item
                let item = tmp_props
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|(prop, _)| prop.name() == "class");
                match item {
                    Some((prop, _)) => {
                        let class = props.remove(prop).unwrap();
                        self.set_class(class);
                        true
                    }
                    None => false,
                }
            }
            None => false,
        }
    }
    pub fn has_class(&self) -> bool {
        self.class.is_some()
    }

    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    pub fn set_id(&mut self, id: &str) -> () {
        let _ = self.id.replace(id.to_string());
    }

    /// 从props中获取key为id的属性
    /// 并从props中删除
    /// 会返回bool
    /// - true: 表示有id并已经设置完成
    /// - false: 表示没有id
    fn set_id_from_props(&mut self) -> bool {
        match self.props.as_mut() {
            Some(props) => {
                let remove_item = PropsKey::new("id", false, PropertyKeyType::Normal);
                match props.remove(&remove_item) {
                    Some(value) => {
                        let _ = self.set_id(value.to_string().as_str());
                        true
                    }
                    None => false,
                }
            }
            None => false,
        }
    }
    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, name: &str) -> () {
        self.name = name.to_string();
    }
    pub fn get_props(&self) -> &Props {
        &self.props
    }
    pub fn set_props(&mut self, props: Props) -> () {
        self.props = props;
    }
    pub fn has_props(&self) -> bool {
        self.props.is_some()
    }
    pub fn push_prop(&mut self, key: PropsKey, value: Value) -> () {
        match &mut self.props {
            Some(props) => {
                let _ = props.insert(key, value);
            }
            None => {
                let mut item = HashMap::new();
                item.insert(key, value);
                self.set_props(Some(item));
            }
        }
    }
    pub fn get_prop_ptr(&self) -> &P {
        &self.prop_ptr
    }
    pub fn set_prop_ptr(&mut self, prop_ptr: P) -> () {
        self.prop_ptr = prop_ptr;
    }
    // pub fn has_prop_ptr(&self) -> bool {
    //     let target = self.get_prop_ptr();
    //     let token = quote!{ #target }.to_token_stream();
    //     let prop = parse2::<ExprStruct>(token).unwrap();
    //     prop.fields.len() > 0
    // }
    pub fn get_callbacks(&self) -> Option<&Callbacks> {
        self.callbacks.as_ref()
    }
    pub fn set_callbacks(&mut self, callbacks: Callbacks) -> () {
        let _ = self.callbacks.replace(callbacks);
    }
    pub fn push_callbacks(&mut self, k: PropsKey, v: Value) -> () {
        match self.callbacks.as_mut() {
            Some(callbacks) => {
                let _ = callbacks.insert(k, v);
            }

            None => {
                self.callbacks = Some(
                    vec![(k, v)]
                        .into_iter()
                        .collect::<HashMap<PropsKey, Value>>(),
                )
            }
        }
    }
    pub fn has_callbacks(&self) -> bool {
        self.callbacks.is_some()
    }
    pub fn set_callbacks_from_props(&mut self) -> bool {
        let tmp_props = self.props.clone();
        match self.props.as_mut() {
            Some(props) => {
                // 所有callbacks都是Value::Function的并且也直接在PropKey上的ty是Function
                tmp_props.unwrap().iter().for_each(|(k, _)| {
                    if PropertyKeyType::Function.eq(k.ty()) {
                        match props.remove_entry(k) {
                            Some((k, v)) => match self.callbacks.as_mut() {
                                Some(callbacks) => {
                                    let _ = callbacks.insert(k, v);
                                }

                                None => {
                                    self.callbacks = Some(
                                        vec![(k, v)]
                                            .into_iter()
                                            .collect::<HashMap<PropsKey, Value>>(),
                                    )
                                }
                            },
                            None => (),
                        }
                    }
                });

                self.has_callbacks()
            }
            None => false,
        }
    }
    pub fn get_event_ptr(&self) -> &E {
        &self.event_ptr
    }
    pub fn set_event_ptr(&mut self, event_ptr: E) -> () {
        self.event_ptr = event_ptr;
    }
    pub fn get_inherits(&self) -> Option<&String> {
        self.inherits.as_ref()
    }
    pub fn set_inherits(&mut self, inherits: &str) -> () {
        let _ = self.inherits.replace(inherits.to_string());
    }
    pub fn set_inherits_from_props(&mut self) -> bool {
        match self.props.as_mut() {
            Some(props) => {
                let remove_item = PropsKey::new("inherits", false, PropertyKeyType::Normal);
                match props.remove(&remove_item) {
                    Some(value) => {
                        let _ = self.set_inherits(value.to_string().as_str());
                        true
                    }
                    None => false,
                }
            }
            None => false,
        }
    }
    pub fn has_inherit(&self) -> bool {
        self.inherits.is_some()
    }
    pub fn is_root(&self) -> bool {
        self.root
    }
    pub fn set_root(&mut self, root: bool) -> () {
        self.root = root;
    }
    pub fn get_children(&self) -> Option<&Vec<TemplateModel<E, P>>> {
        self.children.as_ref()
    }
    pub fn set_children(&mut self, children: Vec<TemplateModel<E, P>>) -> () {
        let _ = self.children.replace(children);
    }
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }
    pub fn push_child(&mut self, child: TemplateModel<E, P>) -> () {
        match &mut self.children {
            Some(children) => children.push(child),
            None => {
                let _ = self.children.replace(vec![child]);
            }
        }
    }
    pub fn set_parent(&mut self, special: &str) -> () {
        let _ = self.parent.replace(special.to_string());
    }
    pub fn convert(ast: &ASTNodes, is_root: bool) -> Option<Self> {
        let mut model = TemplateModel::default();
        let mut flag = false;
        match ast {
            ASTNodes::Tag(tag) => {
                if !flag {
                    flag = true;
                }
                convert_template(&*tag, &mut model, is_root)
            }
            ASTNodes::Comment(_) => {}
            ASTNodes::Style(_) => panic!("cannot write styles in template node"),
        }
        if flag {
            Some(model)
        } else {
            None
        }
    }
}

/// ## 转换模板
/// 将ASTNodes::Tag转换为TemplateModel
/// - 为模型生成一个唯一标识符
/// - 获取Tag的名称作为TemplateModel的名称
/// - 获取Tag被设置的属性作为TemplateModel传入的属性
/// - 提取id, class, inherits
/// - 无需设置prop_ptr和event_ptr（这两个需要解析script才能决定）
/// - 设置root
/// - 获取所有外部传入的事件设置到callbacks上
/// - 设置children
fn convert_template<E: Event, P: Prop>(
    tag: &Tag,
    model: &mut TemplateModel<E, P>,
    is_root: bool,
) -> () {
    // [生成ulid作为模型的唯一标识符]------------------------------------------------------
    let special = Ulid::new().to_string();
    model.set_special(&special);
    model.set_root(is_root);
    // [获取Tag的名称作为TemplateModel的名称]------------------------------------------------
    model.set_name(tag.get_name());
    // [获取Tag被设置的属性作为TemplateModel传入的属性]--------------------------------------
    // 其中id、class会被单独提出来，其他的属性会被放入props中（for,if,inherits等也一样）
    if tag.has_props() {
        let props = tag.get_props().unwrap();
        model.set_props(Some(props.clone()));
    }
    // [完成属性设置后提取id]--------------------------------------------------------------
    model.set_id_from_props();
    // [完成属性设置后提取class列表]--------------------------------------------------------
    model.set_class_from_prop();
    // [完成属性设置后提取inherits]--------------------------------------------------------
    model.set_inherits_from_props();
    // [设置callbacks]------------------------------------------------------------------
    model.set_callbacks_from_props();
    // [设置children]-------------------------------------------------------------------
    if tag.has_children() {
        let children = tag
            .get_children()
            .unwrap()
            .iter()
            .map(|child| {
                let mut model = TemplateModel::convert(child, false).unwrap();
                model.set_special(&special);
                model
            })
            .collect();
        model.set_children(children)
    }
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
