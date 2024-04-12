use makepad_widgets::*;
live_design! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; App = {{ App }}{ ui : < Window >{ show_bg : true , body = < View >{ btn = < Button >{ } } } } }
#[derive(Debug, Clone, Default)]
struct Instance {
    pub view_bg: bool,
    pub btn_name: String,
}
impl Instance {
    fn new() -> Self {
        let mut view_bg = true;
        let mut btn_name = String::from("Click Me!");
        Self { view_bg, btn_name }
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
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn)).clicked(&actions) {
            let mut on_clicked = || {
                self.instance.btn_name = "I have been Clicked!".to_string();
            };
            on_clicked();
        }
        self.ui
            .button(id!(btn))
            .apply_over_and_redraw(cx, live! { text : (self .instance .btn_name) });
    }
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        self.ui
            .view(id!(body))
            .apply_over_and_redraw(cx, live! { show_bg : (self . instance . view_bg) , });
        self.ui
            .button(id!(btn))
            .apply_over_and_redraw(cx, live! { text : (self . instance . btn_name) , });
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
        crate::makepad_widgets::live_design(cx)
    }
}
app_main!(App);
