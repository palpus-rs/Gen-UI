# QuickStart

## init a project

```shell
# new a project called hello
cargo new hello
# add makepad deps
cargo add makepad-widgets
cargo add makepad (waiting makepad to update)
```

## add lib

create a `lib.rs` under `src`

```rust
pub use makepad_widgets;
pub mod app;
```

## create app

### add use widgets

```rust
use makepad_widgets::*;
```

### add live_design

```rust
use makepad_widgets::*;
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    App = {{App}} {}
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
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
```

## fn main

```rust
fn main(){
    hello::app::app_main();
}
```