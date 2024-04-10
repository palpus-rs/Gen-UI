use makepad_widgets::*;
live_design! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; App = {{ App }}{ ui : < Window >{ show_bg : true , } } }
use gen_macros::on_startup;
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
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn)).clicked(&actions) {
            let mut on_clicked = || {
                self.instance.view_bg = false;
            };
        }
    }
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        self.ui
            .view(id!(body))
            .apply_over_and_redraw(cx, live! { show_bg : (self . instance . view_bg) , });
        println!("{}", "hello");
    }
}
