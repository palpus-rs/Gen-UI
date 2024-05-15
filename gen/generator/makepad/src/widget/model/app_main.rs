use gen_converter::model::{
    script::{GenScriptModel, LifeTime, PropFn, ScriptModel, UseMod},
    Source,
};
use gen_utils::common::{token_tree_ident, token_tree_punct_joint};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    widget::model::{widget::Widget, ToLiveDesign},
    ToToken,
};

use super::{
    field::Field, handler::WidgetHandler, live_design::LiveDesign, match_event::MatchEventTrait,
    traits::AppMainTrait,
};

#[derive(Debug, Clone)]
pub struct AppMain {
    /// 当前实例
    pub name: String,
    /// app main的ui入口的name
    pub root_ref: String,
    /// app main的ui入口指向
    pub root_ref_ptr: String,
    /// 处理在实例中的属性
    pub props: Option<Vec<Field>>,
    pub match_event: MatchEventTrait,
    pub app_main: AppMainTrait,
    /// 有哪些组件需要被注册
    /// live design import widget
    pub live_register: Option<Vec<String>>,
    pub source: Source,
    /// rust use code
    pub uses: Option<TokenStream>,
}

impl AppMain {
    pub fn new(source: &Source) -> Self {
        let name = source.source_name();
        AppMain {
            name,
            root_ref: String::from("root"),
            root_ref_ptr: String::new(),
            props: None,
            match_event: Default::default(),
            app_main: Default::default(),
            live_register: None,
            source: source.clone(),
            uses: None,
        }
    }
    pub fn set_live_register(&mut self, children: Vec<String>) -> &mut Self {
        let children = children
            .into_iter()
            .filter(|item| !item.ends_with("mod"))
            .collect();
        self.live_register.replace(children);
        self
    }
    pub fn push_live_register(&mut self, child: String) -> &mut Self {
        if !child.ends_with("mod") {
            match self.live_register.as_mut() {
                Some(live_register) => {
                    let _ = live_register.push(child);
                }
                None => {
                    self.live_register.replace(vec![child]);
                }
            }
        }
        self
    }
    pub fn set_script(&mut self, script: Option<ScriptModel>) -> &mut Self {
        if let Some(sc) = script {
            if let ScriptModel::Gen(sc) = sc {
                let GenScriptModel {
                    uses,
                    prop_ptr,
                    event_ptr,
                    sub_prop_binds,
                    sub_event_binds,
                    other,
                    lifetimes,
                } = sc;

                self.set_uses(uses)
                    .set_props(sub_prop_binds.as_ref())
                    .handle_lifetime(sub_prop_binds, lifetimes)
                    .handle_actions(sub_event_binds);
            }
        }
        self
    }
    pub fn handle_actions(&mut self, actions: Option<Vec<PropFn>>) -> &mut Self {
        if let Some(actions) = actions {
            self.match_event.handle_actions(&self.root_ref, actions);
        }
        self
    }
    pub fn handle_lifetime(
        &mut self,
        binds: Option<Vec<PropFn>>,
        lifetimes: Option<LifeTime>,
    ) -> &mut Self {
        self.match_event
            .handle_lifetime(&self.root_ref, binds, lifetimes);
        self
    }

    pub fn set_props(&mut self, props: Option<&Vec<PropFn>>) -> &mut Self {
        if let Some(props) = props {
            for prop in props {
                match self.props.as_mut() {
                    Some(props) => {
                        props.push(Field::from(prop));
                    }
                    None => {
                        let _ = self.props.replace(vec![Field::from(prop)]);
                    }
                }
            }
        }
        self
    }
    pub fn set_uses(&mut self, uses: Option<UseMod>) -> &mut Self {
        if let Some(uses) = uses {
            self.uses = WidgetHandler::uses(&uses);
        }
        self
    }
    pub fn set_root_ref(&mut self, id: String) -> &mut Self {
        self.root_ref = id;
        self
    }
    pub fn set_root_ref_ptr(&mut self, widget: &str) -> &mut Self {
        self.root_ref_ptr = widget.to_string();
        self
    }
    fn build_live_register(&self) -> TokenStream {
        let items = if let Some(imports) = self.live_register.as_ref() {
            // `crate::a::b::live_design(cx);`
            let tk = imports.iter().fold(TokenStream::new(), |mut acc, item| {
                let mut item = item.split("::").into_iter().fold(
                    TokenStream::new(),
                    |mut item_acc, item_value| {
                        item_acc.extend(vec![
                            token_tree_ident(item_value),
                            token_tree_punct_joint(':'),
                            token_tree_punct_joint(':'),
                        ]);
                        item_acc
                    },
                );
                item.extend(quote! {live_design(cx)});
                acc.extend(quote! {crate::#item;});
                acc
            });

            Some(tk)
        } else {
            None
        };

        quote! {
            fn live_register (cx : & mut Cx) {
                crate::makepad_widgets::live_design(cx);
                #items
            }
        }
    }
}

impl ToLiveDesign for AppMain {
    fn widget_tree(&self) -> Option<TokenStream> {
        let app = token_tree_ident(&self.name);
        let root = token_tree_ident(&self.root_ref);
        let root_widget = token_tree_ident(&self.root_ref_ptr);
        let imports = if let Some(imports) = self.live_register.as_ref() {
            let tk = imports.iter().fold(TokenStream::new(), |mut acc, item| {
                let mut item = item.split("::").into_iter().fold(
                    TokenStream::new(),
                    |mut item_acc, item_value| {
                        item_acc.extend(vec![
                            token_tree_ident(item_value),
                            token_tree_punct_joint(':'),
                            token_tree_punct_joint(':'),
                        ]);
                        item_acc
                    },
                );
                item.extend(quote!(*));
                acc.extend(quote! {import crate::#item;});
                acc
            });
            Some(tk)
        } else {
            None
        };
        let tk = quote! {
            #imports
            #app = {{#app}}{
                #root: <#root_widget>{}
            }
        };
        Some(tk)
    }

    fn widget_logic(&self) -> Option<TokenStream> {
        let root_struct = token_tree_ident(&self.name);
        let ui_field = Field::ui_widget_ref(&self.root_ref).to_token_stream();
        let root_fields = if self.props.is_some() {
            self.props
                .as_ref()
                .unwrap()
                .iter()
                .fold(ui_field, |mut acc, item| {
                    acc.extend(item.to_token_stream());
                    acc
                })
        } else {
            ui_field
        };
        let live_register = self.build_live_register();

        let app_main_trait = self.app_main.to_token_stream(&self.root_ref);

        let tk = quote! {
            #[derive(Live, LiveHook)]
            pub struct #root_struct{
                #root_fields
            }

            impl MatchEvent for #root_struct {

            }

            impl AppMain for App {
                #app_main_trait
            }

            impl LiveRegister for #root_struct {
                #live_register
            }

            app_main!(#root_struct);
        };

        Some(tk)
    }

    fn to_live_design(&self) -> LiveDesign {
        self.into()
    }
}

impl From<gen_converter::model::Model> for AppMain {
    fn from(value: gen_converter::model::Model) -> Self {
        // clone a new script, other make to widget tree
        let script = value.script.clone();
        let mut app = AppMain::new(value.get_special());
        let widget = Widget::from(value);
        let root_id = widget.id.as_ref().expect("root id is required").to_string();
        app.set_root_ref(root_id).set_script(script);
        // let app_tk = app.to_live_design().to_token_stream();
        app
    }
}
