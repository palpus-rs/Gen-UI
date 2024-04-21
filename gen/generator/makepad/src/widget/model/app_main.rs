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


use proc_macro2::TokenStream;

use super::{field::Field, live_design::LiveDesign, match_event::MatchEvent};

#[derive(Debug,Clone,Default)]
pub struct AppMain {
    pub live_design: LiveDesign,
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
    
}

impl AppMain {
    pub fn new(name:&str)->Self{
        let mut app = AppMain::default();
        app.name = name.to_string();
        app
    }
    
}