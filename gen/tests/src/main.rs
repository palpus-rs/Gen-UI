use std::{fs::File, io::Write, path::Path, process::Command};

use gen_compiler::{app, Target};
use gen_converter::model::Model;
fn main() {
    //
    // Users/user/Workspace/others/Gen-UI/gen/tests/ui/view/easy.gen
    // /Users/user/Workspace/others/Gen-UI/gen/tests/ui/app.gen

    let current_dir = std::env::current_dir().unwrap();

    // let mut view_model = Model::new(
    //     Path::new("E:/Rust/try/makepad/Gen-UI/gen/tests/ui/components/hello.gen"),
    //     current_dir,
    //     false,
    // )
    // .unwrap();

    // let app = app!{
    //     Target::Makepad,
    //     "../app.gen"
    // };


    let app = app(Target::Makepad);


    // let code = Makepad::ast(view_model);
    
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
