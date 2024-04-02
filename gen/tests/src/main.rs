use std::path::Path;

use gen_converter::{model::Model, strategy::id};
use gen_parser::*;
use proc_macro2::TokenStream;

fn main() {
    // let mut view_model = Model::new(Path::new(
    //     "E:/Rust/try/makepad/Gen-UI/gen/tests/ui/view/index.gen",
    // ))
    // .unwrap();

    // let _ = id(&mut view_model, |t_model, id_style| {
    //     id_style.into_iter().for_each(|(k, v)| {
    //         t_model.push_prop(k, v);
    //     });
    // });

    // dbg!(view_model);
    let input = r#"
    use makepad_widgets::*;
    live_design! {
        import makepad_widgets::base::*;
        import makepad_widgets::theme_desktop_dark::*;

        App = {{App}}{ 
            ui: <Window>{
                show_bg: true, 
                width: Fill, 
                draw_bg: { color: #96CEF8 }, 
                height: Fill,  
                body = <View>{
                    align: {x: 0.5, y: 0.5}, 
                } 
            } 
        }
    }
    #[derive(Live, LiveHook)]
    pub struct App {
        #[live]
        ui: WidgetRef,
    }

    impl LiveRegister for App {
        fn live_register(cx: &mut Cx) {
            crate::makepad_widgets::live_design(cx);
        }
    }
    impl AppMain for App {
        fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        }
    }
    app_main!(App);
    "#;

    let makepad_ast = input.parse::<TokenStream>().unwrap();
    dbg!(makepad_ast);
}
