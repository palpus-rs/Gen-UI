use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{show_bg: true, draw_bg: { color: #96CEF8 }, width: Fill, height: Fill,  body = <View>{align: {x: 0.5, y: 0.5},  t_label = <Label>{ draw_text: { wrap: Word, color: #ffffff, text_style: { brightness: 1.1, font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } }} } } }
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
    pub view_flow: Flow,
}
impl Instance {
    pub fn new() -> Self {
        Self {
            view_flow: Flow::Down,
        }
    }
    pub fn get_view_flow(&self) -> &Flow {
        &self.view_flow
    }
    pub fn set_view_flow(&mut self, view_flow: Flow) {
        self.view_flow = view_flow
    }
}
impl App {
    fn start_up(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { flow: Down,  });
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { spacing: 20,  });
        let label_t_label = self.ui.label(id!(t_label));
        label_t_label.apply_over_and_redraw(cx, live!{ text: "this is a Hello, World!!😇",  draw_text: { text_style: { font_size: 24,  } } });
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
