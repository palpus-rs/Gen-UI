use makepad_widgets::*;
live_design!{
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{show_bg: true, height: Fill, width: Fill, draw_bg: { color: #96CEF8 },  body = <View>{align: {x: 0.5, y: 0.5}, } } }
}
#[derive(Live, LiveHook)]
pub struct App { #[live] ui: WidgetRef, #[rust] instance: Instance,}
#[derive(Debug, Clone, Default)]
struct Instance { pub view_flow: String  }
impl Instance { pub fn new() -> Self { Self { view_flow: String :: from ("Down") } } pub fn get_view_flow(&self) -> &String  { &self.view_flow } pub fn set_view_flow(&mut self, view_flow: String ) { self.view_flow = view_flow } }
impl App{ fn start_up(&mut self, cx: &mut Cx){ self.instance = Instance::new(); let view_body = self.ui.view(id!(body)); view_body.apply_over_and_redraw(cx, live!{ flow: Down }); let view_body = self.ui.view(id!(body)); view_body.apply_over_and_redraw(cx, live!{ spacing: 20 }); } }
impl LiveRegister for App { fn live_register(cx: &mut Cx) {crate::makepad_widgets::live_design(cx);} }impl AppMain for App { fn handle_event(&mut self, cx: &mut Cx, event: &Event) { match event{ Event::Startup => self.start_up(cx), _ =>() }self.ui.handle_event(cx, event, &mut Scope::empty());} }app_main!(App);