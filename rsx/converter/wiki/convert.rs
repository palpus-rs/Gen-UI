use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{width: All,  body = <View>{} } }
}
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    instance: Instance,
}
#[derive(Debug, Clone, Default)]
struct Instance {
    pub view_hei: String,
}
impl Instance {
    pub fn new() -> Self {
        Self {
            view_hei: String::from("Fit"),
        }
    }
    pub fn get_view_hei(&self) -> &String {
        &self.view_hei
    }
    pub fn set_view_hei(&mut self, view_hei: String) {
        self.view_hei = view_hei
    }
}
impl App {
    fn start_up(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { height: Fit });
        let window_ui = self.ui.window(id!(ui));
        window_ui.apply_over_and_redraw(cx, live! { flow: Down });
    }
}
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => self.start_up(cx),
            _ => (),
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
app_main!(App);
