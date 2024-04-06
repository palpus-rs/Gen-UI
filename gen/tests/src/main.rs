use std::{path::Path, time::Instant};

use gen_converter::{model::Model, strategy::id};
use gen_parser::*;
use makepad_gen_plugin::Makepad;
use proc_macro2::TokenStream;

fn main() {
    // E:/Rust/try/makepad/Gen-UI/gen/tests/ui/view/index.gen
    // Users/user/Workspace/others/Gen-UI/gen/tests/ui/view/easy.gen

    let mut view_model = Model::new(Path::new(
        "E:/Rust/try/makepad/Gen-UI/gen/tests/ui/app.gen",
    ))
    .unwrap();
    // dbg!(&view_model.script);
    let t = Instant::now();
    let _ = Makepad::ast(view_model);

    dbg!(t.elapsed());
    // let input = r#"
    // use makepad_widgets::*;
    // live_design! {
    //     import makepad_widgets::base::*;
    //     import makepad_widgets::theme_desktop_dark::*;

    //     App = {{App}}{
    //         ui: <Window>{
    //             show_bg: true,
    //             width: Fill,
    //             draw_bg: { color: #96CEF8 },
    //             height: Fill,
    //             body = <View>{
    //                 align: {x: 0.5, y: 0.5},
    //             }
    //         }
    //     }
    // }
    // #[derive(Live, LiveHook)]
    // pub struct App {
    //     #[live]
    //     ui: WidgetRef,
    // }

    // impl LiveRegister for App {
    //     fn live_register(cx: &mut Cx) {
    //         crate::makepad_widgets::live_design(cx);
    //     }
    // }

    // impl AppMain for App {
    //     fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
    //         self.ui.handle_event(cx, event, &mut Scope::empty());
    //     }
    // }

    // impl MatchEvent for MyApp {
    //     fn handle_startup(&mut self, cx: &mut Cx) {}
    // }
    // app_main!(App);
    // "#;

    // let makepad_ast = input.parse::<TokenStream>().unwrap();
    // dbg!(makepad_ast);
}
