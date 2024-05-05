// use makepad_widgets::*;
// live_design! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; App = {{ App }}{ ui : < Window >{ show_bg : true , body = < View >{ btn = < Button >{ } } } } }
// #[derive(Debug, Clone, Default)]
// struct Instance {
//     pub btn_text: String,
//     pub view_bg: bool,
// }
// impl Instance {
//     fn new() -> Self {
//         let mut btn_text = String::from("Clicked!");
//         let mut view_bg = true;
//         Self { btn_text, view_bg }
//     }
// }
// #[derive(Live, LiveHook)]
// pub struct App {
//     #[live]
//     pub ui: WidgetRef,
//     #[rust]
//     pub instance: Instance,
// }
// impl MatchEvent for App {
//     fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
//         if self.ui.button(id!(btn)).clicked(&actions) {
//             let mut on_clicked = || {
//                 self.instance.btn_text = "I have been clicked!".to_string();
//             };
//             on_clicked();
//         }
//         self.ui
//             .button(id!(btn))
//             .apply_over_and_redraw(cx, live! { text : (self .instance .btn_text) });
//     }
//     fn handle_startup(&mut self, cx: &mut Cx) {
//         self.instance = Instance::new();
//         self.ui
//             .button(id!(btn))
//             .apply_over_and_redraw(cx, live! { text : (self . instance . btn_text) , });
//         self.ui
//             .view(id!(body))
//             .apply_over_and_redraw(cx, live! { show_bg : (self . instance . view_bg) , });
//         println!("{}", "hello");
//     }
// }
// impl AppMain for App {
//     fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
//         match event {
//             Event::Startup => self.handle_startup(cx),
//             _ => (),
//         }
//         self.match_event(cx, event);
//         self.ui.handle_event(cx, event, &mut Scope::empty());
//     }
// }
// impl LiveRegister for App {
//     fn live_register(cx: &mut Cx) {
//         crate::makepad_widgets::live_design(cx)
//     }
// }
// app_main!(App);

use gen_converter::model::{
    script::{GenScriptModel, LifeTime, PropFn, ScriptModel, UseMod},
    Source,
};
use proc_macro2::TokenStream;

use crate::widget::model::widget::Widget;

use super::{
    field::Field, handler::WidgetHandler, live_design::LiveDesign, match_event::MatchEvent,
};

#[derive(Debug, Clone, Default)]
pub struct AppMain {
    /// 当前实例
    pub name: String,
    /// app main的ui入口
    pub root_ref: String,
    /// 处理在实例中的属性
    pub props: Field,
    pub match_event: MatchEvent,
    pub app_main: Option<TokenStream>,
    /// 有哪些组件需要被注册
    pub live_register: Vec<String>,
    pub source: Source,
    /// DSL Tree Node in Live Design
    pub dsl: TokenStream,
    pub root_id: String,
    pub uses: Option<TokenStream>,
}

impl AppMain {
    pub fn new(source: &Source) -> Self {
        let name = source.source_name();

        let mut app = AppMain::default();
        app.name = name;

        app
    }
    pub fn set_script(&mut self, script: Option<ScriptModel>) -> &mut Self {
        if let Some(sc) = script {
            if let ScriptModel::Gen(sc) = sc {
                dbg!(&sc);
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
                    .handle_lifetime(sub_prop_binds, lifetimes)
                    .handle_actions(sub_event_binds);
                // self.set_uses(uses)
                //     .set_prop_ptr(prop_ptr)
                //     .set_event_ptr(event_ptr)
                //     .draw_walk(sub_prop_binds)
                //     .handle_event(sub_event_binds);
            }
        }
        self
    }
    pub fn handle_actions(&mut self, actions: Option<Vec<PropFn>>) -> &mut Self {
        if let Some(actions) = actions {
            self.match_event.handle_actions(&self.root_id, actions);
        }
        self
    }
    pub fn handle_lifetime(
        &mut self,
        binds: Option<Vec<PropFn>>,
        lifetimes: Option<LifeTime>,
    ) -> &mut Self {
        self.match_event
            .handle_lifetime(&self.root_id, binds, lifetimes);
        self
    }

    pub fn set_dsl(&mut self, dsl: TokenStream) -> &mut Self {
        self.dsl = dsl;
        self
    }
    pub fn set_uses(&mut self, uses: Option<UseMod>) -> &mut Self {
        if let Some(uses) = uses {
            self.uses = WidgetHandler::uses(&uses);
        }
        self
    }
    pub fn set_root_id(&mut self, id: String) -> &mut Self {
        self.root_id = id;
        self
    }
}

impl From<gen_converter::model::Model> for AppMain {
    fn from(value: gen_converter::model::Model) -> Self {
        // clone a new script, other make to widget tree
        let script = value.script.clone();
        // let gen_converter::model::Model {
        //     special,
        //     template,
        //     script,
        //     style,
        //     compile,
        //     is_entry,
        // } = value;
        let mut app = AppMain::new(value.get_special());

        let widget = Widget::from(value);
        let root_id = widget.id.as_ref().expect("root id is required").to_string();
        let dsl = widget.widget_tree().unwrap();
        app.set_root_id(root_id).set_dsl(dsl).set_script(script);

        todo!("{:#?}", app);
    }
}
