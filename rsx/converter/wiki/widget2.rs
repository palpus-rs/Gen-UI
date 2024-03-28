use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
EasyWidget = {{EasyWidget}}{  first_lb = <Label>{ draw_text: { wrap: Word, color: #ffffff, text_style: { font_size: 32, brightness: 1.1, font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } }} second_lb = <Label>{text: "label 2",  draw_text: { wrap: Word, color: #ffffff, text_style: { brightness: 1.1, font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } }} bb = <Button>{text: "text btn", }  }
}
#[derive(Default, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct MyProps {
    #[live]
    pub label1: RcStringMut,
}
#[derive(Live, LiveHook, Widget)]
pub struct EasyWidget {
    #[deref]
    #[redraw]
    instance: View,
    #[live]
    props: MyProps,
}
impl Widget for EasyWidget {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let label_second_lb = self.label(id!(second_lb));
        label_second_lb.apply_over_and_redraw(
            cx,
            live! {  draw_text: { text_style: { font_size: 18,  } } },
        );
        let _ = self.instance.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for action in cx.capture_actions(|cx| self.button(id!(bb)).handle_event(cx, event, scope)) {
            match action.as_widget_action().cast() {
                ButtonAction::Clicked => {
                    let mut btn_click = || {
                        log!("Button bb Clicked");
                    };
                    btn_click();
                }
                _ => (),
            }
        }
        let _ = self.instance.handle_event(cx, event, scope);
    }
}
