use gen_macros::{on_shutdown, Event, Props};
#[derive(Live, LiveHook, Widget)]
pub struct MyProps {
    pub label1: String,
}
#[derive(DefaultNone, Clone, Debug)]
pub enum Events {
    Clicked(String),
    None,
}
println!("{}", "on shutdown!");
println!("GenUI: {}", "good bye!");
