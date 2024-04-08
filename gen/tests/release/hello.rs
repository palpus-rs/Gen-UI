use ::gen_macros::on_startup;
impl MatchEvent for app {
    fn handle_startup(&mut self, cx: &mut Cx) {
        println("{}", "hello");
    }
}

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
