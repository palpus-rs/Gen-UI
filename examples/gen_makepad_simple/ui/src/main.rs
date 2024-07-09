use gen_compiler::{app, Target};

fn main() {
    // set app and specify target
    let mut app = app(Target::Makepad)
        .entry("app")
        .root("E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen")
        .add_dep("makepad-widgets")
        .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
        .build()
        .build();

    let _ = app.run();
}
