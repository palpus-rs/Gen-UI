use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{ body = <View>{ btn1 = <Button>{} t_label = <Label>{ draw_text: { text_style: { font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } }} } } }
}
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    instance: Instance,
}
#[derive(Debug, Clone, Default)]
struct Instance {
    pub label_text: String,
}
impl Instance {
    pub fn new() -> Self {
        Self {
            label_text: String::from("this is a Hello, World!! emoji failed"),
        }
    }
    pub fn get_label_text(&self) -> &String {
        &self.label_text
    }
    pub fn set_label_text(&mut self, label_text: String) {
        self.label_text = label_text
    }
}
impl App {
    fn start_up(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        let label_t_label = self.ui.label(id!(t_label));
        label_t_label.apply_over_and_redraw(
            cx,
            live! { text: "this is a Hello, World!! emoji failed",  draw_text: {  } },
        );
    }
}
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.Button(id!(btn1)).clicked(&actions) {
            let change_text = || {
                let label_t_label = self.ui.label(id!(t_label));
                self.ui.instance.label_text =
                    label_t_label.apply_over_and_redraw(cx, String::from("I have been clicked!"));
                let a = self.ui.instance.label_text;
            };
            change_text();
        }
    }
}
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => self.start_up(cx),
            _ => (),
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
app_main!(App);
