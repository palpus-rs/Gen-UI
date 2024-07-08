use gen_compiler::{app, Target};

fn main() {
    // set app and specify target
    let mut app = app(Target::Makepad)
        .entry("app")
        .root("E:/Rust/try/makepad/Gen-UI/examples/gosim_example/ui/views/root.gen")
        .add_dep("makepad-widgets")
        .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
        .build()
        .wasm()
        .build()
        .build();

    let _ = app.run();
}
