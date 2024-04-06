use ::gen_macros::on_startup;
impl MatchEvent for app {
    fn handle_startup(&mut self, cx: &mut Cx) {
        println("{}", "hello");
        let view_space: f64 = 20;
        let mut view_flow = String::from("Down");
        let mut label_text = String::from("this is a Hello, World!! emoji failed");
        let label_size = 24.0;
        let btn_text = String::from("Click Me");
        let mut change_text = || {
            label_text = String::from("I have been clicked!");
        };
    }
}
