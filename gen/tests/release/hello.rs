use ::gen_macros::on_startup;
impl MatchEvent for app {
    fn handle_startup(&mut self, cx: &mut Cx) {
        println("{}", "hello");
    }
}

fn a() {
    let view_bg = true;
    self.view(id!(body).apply_over_and_redraw(cx, live! { show_bg : (view_bg) , }));
}

