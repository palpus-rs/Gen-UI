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
        if self.btn.button(id!(btn)).on_clicked(&actions) {
            let mut on_clicked = || {
                view_bg = false;
            };
        }
    }
    fn handle_startup(&mut self, cx: &mut Cx) {
        println("{}", "hello");
        self.body.view(
            id!(body).apply_over_and_redraw(cx, live! { show_bg : (self . instance . view_bg) , }),
        );
    }
}
