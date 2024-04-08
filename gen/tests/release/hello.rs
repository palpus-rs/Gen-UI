use ::gen_macros::on_startup;
impl MatchEvent for app {
    fn handle_startup(&mut self, cx: &mut Cx) {
        println("{}", "hello");
    }
}
