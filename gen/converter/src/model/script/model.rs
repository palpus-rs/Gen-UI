use std::default;

use gen_parser::{Script, Value};

use syn::{Block, Meta, Pat};

use crate::model::PropTree;

use super::{r#use::UseMod, LifeTime, PropFn};

#[derive(Debug, Clone)]
pub enum ScriptModel {
    Gen(GenScriptModel),
    Rs(Block),
}

impl Default for ScriptModel {
    fn default() -> Self {
        ScriptModel::Gen(GenScriptModel::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct GenScriptModel {
    /// 使用的包,依赖
    pub uses: Option<UseMod>,
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
    // prop_ptr: Box<dyn Prop>,
    pub prop_ptr: Option<syn::ItemStruct>,
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
    // event_ptr: Box<dyn Event>,
    pub event_ptr: Option<syn::ItemEnum>,
    /// 组件的生命周期
    /// 在GenUI中声明周期使用宏来进行标注
    /// 例如: on_startup! on_shutdown!
    pub lifetimes: Option<LifeTime>,
    /// 表示当前组件的内部子组件的属性绑定
    pub sub_prop_binds: Option<Vec<PropFn>>,
    /// 表示当前组件的内部子组件的事件绑定
    pub sub_event_binds: Option<Vec<PropFn>>,
    /// 其他的代码，例如一些过程代码
    pub other: Option<Vec<syn::Stmt>>,
}

impl GenScriptModel {
    pub fn new(block: Block, bind_fn_tree: &(PropTree, PropTree)) -> Self {
        build_script(block, bind_fn_tree)
    }
    pub fn get_uses(&self) -> Option<&UseMod> {
        self.uses.as_ref()
    }
    pub fn get_prop_ptr(&self) -> Option<&syn::ItemStruct> {
        self.prop_ptr.as_ref()
    }
    pub fn get_event_ptr(&self) -> Option<&syn::ItemEnum> {
        self.event_ptr.as_ref()
    }
    pub fn get_lifetimes(&self) -> Option<&LifeTime> {
        self.lifetimes.as_ref()
    }
    pub fn get_sub_prop_binds(&self) -> Option<&Vec<PropFn>> {
        self.sub_prop_binds.as_ref()
    }
    pub fn get_sub_event_binds(&self) -> Option<&Vec<PropFn>> {
        self.sub_event_binds.as_ref()
    }
    pub fn get_other(&self) -> Option<&Vec<syn::Stmt>> {
        self.other.as_ref()
    }
    pub fn set_uses(&mut self, uses: UseMod) {
        self.uses = Some(uses);
    }
    pub fn set_prop_ptr(&mut self, prop: syn::ItemStruct) {
        if self.prop_ptr.is_none() {
            let _ = self.prop_ptr.replace(prop);
            return;
        }
        panic!("Only one struct can be derived from Prop")
    }
    pub fn set_event_ptr(&mut self, event: syn::ItemEnum) {
        if self.event_ptr.is_none() {
            let _ = self.event_ptr.replace(event);
            return;
        }
        panic!("Only one enum can be derived from Event");
    }
    pub fn set_lifetimes(&mut self, lifetimes: Option<LifeTime>) {
        self.lifetimes = lifetimes;
    }
    pub fn push_other(&mut self, stmt: syn::Stmt) {
        if self.other.is_none() {
            self.other.replace(vec![stmt]);
        } else {
            self.other.as_mut().unwrap().push(stmt);
        }
    }
    pub fn push_sub_prop_binds(
        &mut self,
        bind_tree: &PropTree,
        ident: &str,
        code: &syn::Stmt,
    ) -> bool {
        push_sub_prop_fn(
            self,
            bind_tree,
            ident,
            code,
            |v| v.is_bind_and_get().unwrap(),
            |target, item| {
                if target.sub_prop_binds.is_none() {
                    let _ = target.sub_prop_binds.replace(vec![]);
                }
                let _ = target.sub_prop_binds.as_mut().unwrap().push(item);
            },
        )
    }
    pub fn push_sub_fn_binds(
        &mut self,
        bind_tree: &PropTree,
        ident: &str,
        code: &syn::Stmt,
    ) -> bool {
        push_sub_prop_fn(
            self,
            bind_tree,
            ident,
            code,
            |v| v.is_fn_and_get().unwrap().get_name(),
            |target, item| {
                if target.sub_event_binds.is_none() {
                    let _ = target.sub_event_binds.replace(vec![]);
                }
                let _ = target.sub_event_binds.as_mut().unwrap().push(item);
            },
        )
    }
    pub fn push_sub_prop_fn(
        &mut self,
        bind_fn_tree: &(PropTree, PropTree),
        ident: &str,
        code: &syn::Stmt,
    ) -> bool {
        if self.push_sub_prop_binds(&bind_fn_tree.0, ident, code) {
            return true;
        }
        self.push_sub_fn_binds(&bind_fn_tree.1, ident, code)
    }
}

fn build_script(block: Block, bind_fn_tree: &(PropTree, PropTree)) -> GenScriptModel {
    let stmts = block.stmts;

    let mut model = GenScriptModel::default();
    let mut lifetimes: Option<LifeTime> = None;

    for stmt in &stmts {
        match stmt {
            syn::Stmt::Item(item) => {
                match item {
                    syn::Item::Use(use_item) => {
                        // 过滤gen中的所有的依赖

                        if model.uses.is_none() {
                            model.uses.replace(UseMod::default());
                        }

                        model.uses.as_mut().unwrap().push(use_item.clone());
                    }
                    syn::Item::Struct(struct_item) => {
                        // 查看是否有`#[derive(Prop)]`的属性
                        // 如果有则将其将prop设置为Some
                        // 否则放到other中
                        for attr in struct_item.clone().attrs {
                            if let Meta::List(list) = &attr.meta {
                                if list.path.is_ident("derive")
                                    && list.tokens.to_string().contains("Prop")
                                {
                                    model.set_prop_ptr(struct_item.clone());
                                } else {
                                    model.push_other(stmt.clone());
                                }
                            }
                        }
                    }
                    syn::Item::Enum(enum_item) => {
                        // 处理带有`#[derive(Event)]`的枚举
                        // 如果有则将其将event设置为Some
                        // 否则放到other中
                        for attr in enum_item.clone().attrs {
                            if let Meta::List(list) = &attr.meta {
                                if list.path.is_ident("derive")
                                    && list.tokens.to_string().contains("Event")
                                {
                                    model.set_event_ptr(enum_item.clone());
                                } else {
                                    model.push_other(stmt.clone());
                                }
                            }
                        }
                    }
                    _ => {
                        // 其他情况也直接放到other中
                        model.push_other(stmt.clone());
                    }
                }
            }
            syn::Stmt::Macro(item) => {
                if lifetimes.is_none() {
                    lifetimes.replace(LifeTime::default());
                }
                // 处理生命周期
                // 目前只处理带有`on_startup!, on_shutdown!`标识的
                // 其他的放到other中
                if item.mac.path.is_ident("on_startup") {
                    // 处理生命周期
                    lifetimes.as_mut().unwrap().set_startup(item.clone());
                } else if item.mac.path.is_ident("on_shutdown") {
                    lifetimes.as_mut().unwrap().set_shutdown(item.clone());
                } else {
                    model.push_other(stmt.clone());
                }
            }
            syn::Stmt::Local(local) => {
                // 处理属性绑定 和 事件绑定

                let ident = match &local.pat {
                    Pat::Ident(ident) => Some(ident.ident.to_string()),
                    Pat::Type(ty) => {
                        if let Pat::Ident(ident) = &*ty.pat {
                            Some(ident.ident.to_string())
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(ident) = ident {
                    if model.push_sub_prop_fn(&bind_fn_tree, &ident, &stmt) {
                        continue;
                    } else {
                        model.push_other(stmt.clone());
                    }
                } else {
                    model.push_other(stmt.clone());
                }
            }
            _ => {
                // 其他情况直接放到other中
                model.push_other(stmt.clone());
            }
        }
    }
    model.set_lifetimes(lifetimes);
    model
}

fn push_sub_prop_fn<C, F>(
    target: &mut GenScriptModel,
    bind_tree: &PropTree,
    ident: &str,
    code: &syn::Stmt,
    condition: C,
    f: F,
) -> bool
where
    C: Fn(&Value) -> &str,
    F: Fn(&mut GenScriptModel, PropFn) -> (),
{
    let mut flag = false;
    'out: for ((widget, id), prop_fn_key) in bind_tree {
        if prop_fn_key.is_some() {
            for (k, v) in prop_fn_key.as_ref().unwrap() {
                let target_ident = condition(v);
                // dbg!(target_ident, ident);
                let is_prop = if target_ident.eq(ident) {
                    true
                } else if target_ident.starts_with(ident) {
                    false
                } else {
                    continue;
                };

                let item = PropFn {
                    widget: widget.to_string(),
                    id: id.to_string(),
                    key: k.clone(),
                    ident: v.clone(),
                    code: code.clone(),
                    is_prop,
                };
                f(target, item);
                flag = true;
                break 'out;
                // if target_ident.eq(ident) || target_ident.starts_with(ident) {

                // } else {
                //     continue;
                // }
            }
        }
    }
    flag
}
