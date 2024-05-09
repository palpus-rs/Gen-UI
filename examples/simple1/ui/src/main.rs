use gen_compiler::{app, Target};

fn main() {
    let mut app = app(Target::Makepad);
    let _ = app
        .entry("app")
        .root("E:/Rust/try/makepad/Gen-UI/examples/simple1/ui/views/root.gen")
        .compile();
    let _ = app.run();
}
