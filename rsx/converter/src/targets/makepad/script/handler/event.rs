pub fn build_handle_actions(action_str: &str) -> String {
    format!(
        "fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){{ {} }} ",
        action_str
    )
}

pub fn build_handle_startup(mut_setup: &str, immut_setup: &str) -> String {
    format!(
        "fn handle_startup(&mut self, _cx: &mut Cx) {{ self.instance = Instance::new(); {} {} }}",
        mut_setup, immut_setup
    )
}
