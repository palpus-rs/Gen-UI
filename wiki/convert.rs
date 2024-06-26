use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
MyApp = {{MyApp}}{ ui: <Window>{show_bg: true, height: Fill, draw_bg: { color: #96CEF8 }, width: Fill,  body = <View>{align: {x: 0.5, y: 0.5},  btn1 = <Button>{} t_label = <Label>{ draw_text: { wrap: Word, color: #ffffff, text_style: { font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}, brightness: 1.1,  } }} } } }
}
#[derive(Live, LiveHook)]
pub struct MyApp {
    #[live]
    ui: WidgetRef,
    #[rust]
    instance: Instance,
}
#[derive(Debug, Clone, Default)]
struct Instance {
    pub view_flow: Flow,
    pub label_text: String,
}
impl Instance {
    pub fn new() -> Self {
        Self {
            view_flow: Flow::Down,
            label_text: String::from("this is a Hello, World!! emoji failed"),
        }
    }
    pub fn get_view_flow(&self) -> &Flow {
        &self.view_flow
    }
    pub fn set_view_flow(&mut self, view_flow: Flow) {
        self.view_flow = view_flow
    }
    pub fn get_label_text(&self) -> &String {
        &self.label_text
    }
    pub fn set_label_text(&mut self, label_text: String) {
        self.label_text = label_text
    }
}
impl MatchEvent for MyApp {
    
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { flow: Down,  });
        let label_t_label = self.ui.label(id!(t_label));
        label_t_label.apply_over_and_redraw(
            cx,
            live! { text: "this is a Hello, World!! emoji failed",  draw_text: {  } },
        );
        let label_t_label = self.ui.label(id!(t_label));
        label_t_label.apply_over_and_redraw(
            cx,
            live! {  draw_text: { text_style: { font_size: 24,  } } },
        );
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { spacing: 20,  });
        let button_btn1 = self.ui.button(id!(btn1));
        button_btn1.apply_over_and_redraw(cx, live! { text: "Click Me",  });
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn1)).clicked(&actions) {
            let mut change_text = || {
                self.instance.label_text = String::from("I have been clicked!");
            };
            change_text();
            let label_t_label = self.ui.label(id!(t_label));
            label_t_label
                .apply_over_and_redraw(cx, live! { text: (self.instance.get_label_text()), });
        }
    }
}
impl LiveRegister for MyApp {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}
impl AppMain for MyApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => self.handle_startup(cx),
            _ => (),
        };
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
app_main!(MyApp);
