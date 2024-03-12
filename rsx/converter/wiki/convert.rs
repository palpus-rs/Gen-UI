use makepad_widgets::*;
live_design!{
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{padding: {top: 10, right: 16, bottom: 10, left: 16}, width: Fill, show_bg: true, height: Fill, draw_bg: { color: #7733ff },  body = <View>{spacing: 20, align: {x: 0.5, y: 0.5}, flow: Down, } } }
}
#[derive(Live, LiveHook)]
pub struct App { #[live] ui: WidgetRef }
impl LiveRegister for App { fn live_register(cx: &mut Cx) {crate::makepad_widgets::live_design(cx);} }impl AppMain for App { fn handle_event(&mut self, cx: &mut Cx, event: &Event) { self.ui.handle_event(cx, event, &mut Scope::empty()); } }
app_main!(App);
