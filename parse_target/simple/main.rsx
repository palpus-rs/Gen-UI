pub mod app;
pub use makepad_widgets;

/// app.rsx -> app.rs
use app::App;

/// build app main use `expose`
let app = app_main!(App);

let _ = app.app_main();