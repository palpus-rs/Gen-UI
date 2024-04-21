use std::{fs::File, io::Write, path::Path, process::Command};

use gen_converter::model::Model;

use makepad_gen_plugin::Makepad;

fn main() {
    //
    // Users/user/Workspace/others/Gen-UI/gen/tests/ui/view/easy.gen
    // /Users/user/Workspace/others/Gen-UI/gen/tests/ui/app.gen

    let mut view_model = Model::new(Path::new(
        "E:/Rust/try/makepad/Gen-UI/gen/tests/ui/app.gen",
    ))
    .unwrap();

    // let code = Makepad::ast(view_model);
    let code = makepad_gen_plugin::widget::model::Model::new(view_model);
    // let mut f =
    //     File::create("E:/Rust/learn/makepad/makepad-rik/examples/simple/src/app.rs").unwrap();
    // let _ = f.write(code.to_token_stream().to_string().as_bytes());
    // if let Err(e) = Command::new("cargo")
    //     .args(["run"])
    //     .current_dir("E:/Rust/learn/makepad/makepad-rik/examples/simple")
    //     .status()
    // {
    //     eprintln!("Failed to start Makepad project: {}", e);
    // }
}
