use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
  App = {{App}}{
    ui: <Window>{
      height: Fill,
      show_bg: true,
      width: Fill,
      draw_bg: { color: #7733ff },
      body = <View>{
        flow: Down,
        spacing: 20,
        align: {x: 0.5, y: 0.5},
      }
    }
  }
}
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {}
}
app_main!(App);
