use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
EasyWidget = {{EasyWidget}}{  <Label>{text: "label 1",  draw_text: { color: #ffffff, wrap: Word, text_style: { font_size: 32, brightness: 1.1, font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } }} second_lb = <Label>{text: "label 2",  draw_text: { color: #ffffff, wrap: Word, text_style: { brightness: 1.1, font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } }}  }
}
#[derive(Live, LiveHook, Widget)]
pub struct EasyWidget {
    #[deref]
    #[redraw]
    instance: View,
}
impl Widget for EasyWidget {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.instance.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
}
