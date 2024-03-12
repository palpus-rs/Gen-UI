use makepad_widgets::*;
live_design!{
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{padding: {top: 16, right: 16, bottom: 16, left: 16}, width: Fill, show_bg: true, draw_bg: { color: #7733ff }, height: Fill,  body = <View>{spacing: 20, flow: Down, align: {x: 0.5, y: 0.5}, } } }
}
#[derive(Live, LiveHook)]
pub struct App { #[live] ui: WidgetRef }
impl LiveRegister for App { fn live_register(cx: &mut Cx) {crate::makepad_widgets::live_design(cx);} }impl AppMain for App { fn handle_event(&mut self, cx: &mut Cx, event: &Event) { self.ui.handle_event(cx, event, &mut Scope::empty()); } }
app_main!(App);
