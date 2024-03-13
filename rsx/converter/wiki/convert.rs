use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{cursor: Hand, width: All, flow: Down, clip_x: true,  body = <View>{height: 170.2, line_spacing: 20, } } }
}
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    view_hei: f64,
    #[rust]
    view_width: String,
    #[rust]
    f1: String,
    #[rust]
    line_s: f64,
    #[rust]
    cx: bool,
    #[rust]
    c_hand: String,
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
app_main!(App);
