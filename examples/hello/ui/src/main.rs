use gen_compiler::{app, DepType, RustDependence, Target};

fn main() {
    // set app and specify target
    let mut app = app(Target::Makepad);
    // add makepad widget dependence
    let mut makepad_widget = RustDependence::new("makepad-widgets");
    makepad_widget.set_ty(DepType::local(
        "E:/Rust/try/makepad/makepad/rik/makepad/widgets",
    ));
    
    // compile and run
    let _ = app
        .entry("app")
        .root("E:/Rust/try/makepad/Gen-UI/examples/hello/ui/views/root.gen")
        .add_dep(makepad_widget)
        .compile();

    let _ = app.run();
}
