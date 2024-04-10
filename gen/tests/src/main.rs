use std::{path::Path, time::Instant};

use gen_converter::{model::Model, strategy::id};
use gen_parser::*;
use makepad_gen_plugin::Makepad;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Block};

fn main() {
    // E:/Rust/try/makepad/Gen-UI/gen/tests/ui/app.gen
    // Users/user/Workspace/others/Gen-UI/gen/tests/ui/view/easy.gen
    // /Users/user/Workspace/others/Gen-UI/gen/tests/ui/app.gen

    let mut view_model = Model::new(Path::new(
        "/Users/user/Workspace/others/Gen-UI/gen/tests/ui/app.gen",
    ))
    .unwrap();
    // dbg!(&view_model.script);
    let t = Instant::now();
    let _ = Makepad::ast(view_model);

    dbg!(t.elapsed());


    // let input = r#"live_design! { import makepad_widgets::base::*; import makepad_widgets::theme_desktop_dark::*; App = {{App}}{ [ui] : <Window>{ show_bg : true, } } }"#;

}
