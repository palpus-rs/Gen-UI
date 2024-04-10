use makepad_widgets::*;
live_design! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; App = {{ App }}{ ui : < Window >{ show_bg : true , } } }
#[derive(Debug, Clone, Default)]
struct Instance {
    pub view_bg: bool,
}
impl Instance {
    fn new() -> Self {
        let mut view_bg = true;
        Self { view_bg }
    }
}
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    pub ui: WidgetRef,
    #[rust]
    pub instance: Instance,
}
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {}
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        self.ui
            .view(id!(body))
            .apply_over_and_redraw(cx, live! { show_bg : (self . instance . view_bg) , });
        println!("{}", "hello");
    }
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => self.handle_startup(cx),
            _ => (),
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::app::live_design(cx)
    }
}
app_main!(App);
