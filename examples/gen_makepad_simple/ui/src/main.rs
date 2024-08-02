use gen_compiler::{app, Builder};

fn main() {
    let mut app = app(None).build();
    let _ = app.run();
}
