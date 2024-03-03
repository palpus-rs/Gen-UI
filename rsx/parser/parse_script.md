# parse script

## Normal

### rsx

```rust
// makepad
// #[derive(Live, LiveHook)]
// pub struct App {
//     #[live] ui: WidgetRef,
//     #[rust] counter: usize,
//  }

// rsx
let counter: usize = 0_usize;
let label: String = "";

// makepad
// impl MatchEvent for App{
//     fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
//         if self.ui.button(id!(button1)).clicked(&actions) {
//             log!("BUTTON CLICKED {}", self.counter);
//             self.counter += 1;
//             let label = self.ui.label(id!(label1));
//             label.set_text_and_redraw(cx,&format!("Counter: {}", self.counter));
//         }
//     }
// }

// rsx
let handle_actions = ||{
  log!("BUTTON CLICKED {}", counter);
  counter += 1;
  label = format!("Counter: {}", counter);
}
```
