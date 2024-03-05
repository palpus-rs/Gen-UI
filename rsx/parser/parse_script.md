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
//
// impl MatchEvent for App{
//     fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
//         if self.ui.button(id!(button1)).clicked(&actions) {
//             log!("BUTTON CLICKED {}", self.counter);
//             self.counter += 1;
//         }
//     }
// }

// rsx
let mut counter:usize = 0_usize;

let mut click = ||{
  counter += 1;
};
```
