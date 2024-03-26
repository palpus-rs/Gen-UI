use makepad_widgets::*;
live_design! {
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
easy_widget = {{easy_widget}}{  <Label>{text: "label 1",  draw_text: { wrap: Word, color: #ffffff, text_style: { font_size: 32, brightness: 1.1, font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } }} second_lb = <Label>{text: "label 2",  draw_text: { wrap: Word, color: #ffffff, text_style: { brightness: 1.1, font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } }}  }
}
impl LiveRegister for easy_widget {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}
impl AppMain for easy_widget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
