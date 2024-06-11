use makepad_widgets::*;
use shader::draw_text::{TextStyle, TextWrap};
// use shader::draw_text::TextWrap;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;


    // font styles -------------------------------------------------
    FONT_FAMILY = dep("crate://self/resources/font/GoNotoKurrent-Regular.ttf")
    FONT_FAMILY_BOLD = dep("crate://self/resources/font/GoNotoKurrent-Bold.ttf")


    GLabelBase = {{GLabel}}{
        origin_label = <Label>{
            draw_text: {
                text_style: {
                    font: { path: (FONT_FAMILY) }
                }
            }
        }
    }
}

#[derive(Live, Widget, LiveHook)]
pub struct GLabel {
    #[live]
    text: RcStringMut,
    #[live]
    color: Vec4,
    #[live(12.0)]
    font_size: f64,
    #[live(TextWrap::Word)]
    wrap: TextWrap,
    #[live]
    font_family: LiveDependency,
    #[deref]
    base: View,
}

impl GLabel {
    
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let origin_label = self.label(id!(origin_label));
        
        

        origin_label.apply_over_and_redraw(
            cx,
            live! {
                text: (self.text),
                draw_text: {
                    color: (self.color),
                    text_style: {
                        font: { path: (self.font_family) },
                        font_size: (self.font_size),
                    },
                },
            },
        );
        origin_label.draw_walk_all(cx, scope, walk);
        // set wrap
        match self.wrap {
            TextWrap::Ellipsis => origin_label.apply_over_and_redraw(
                cx,
                live! {
                    draw_text: {
                        wrap: Ellipsis
                    }
                },
            ),
            TextWrap::Word => origin_label.apply_over_and_redraw(
                cx,
                live! {
                    draw_text: {
                        wrap: Word
                    }
                },
            ),
            TextWrap::Line => origin_label.apply_over_and_redraw(
                cx,
                live! {
                    draw_text: {
                        wrap: Line
                    }
                },
            ),
        }

        let _ = self.base.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
    /// copy label text
    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v)
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.label(id!(origin_label)).redraw(cx);
        self.redraw(cx);
    }
}
