use font_atlas::CxFontsAtlasRc;
use makepad_widgets::*;
use shader::draw_text::TextWrap;

live_design! {
    GLabelBase = {{GLabel}}{}
}

#[derive(Live, Widget, LiveHook)]
pub struct GLabel {
    #[live]
    pub color: Vec4,
    #[live(9.0)]
    pub font_size: f64,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.5)]
    pub curve: f32,
    #[live(1.4)]
    pub line_spacing: f64,
    #[live(1.1)]
    pub top_drop: f64,
    #[live(1.3)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    // deref ---------------------
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[walk]
    walk: Walk,
    #[live]
    align: Align,
    #[live]
    padding: Padding,
    #[live]
    text: RcStringMut,
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let font_family = self.font_family.clone();

        let atlas = cx.get_global::<CxFontsAtlasRc>().clone();
        let font_id = Some(
            atlas
                .0
                .borrow_mut()
                .get_font_by_path(cx, font_family.as_str()),
        );
        let font = Font {
            font_id,
            path: font_family,
        };

        self.draw_text.apply_over(
            cx,
            live! {
                color: (self.color),
                text_style: {
                    brightness: (self.brightness),
                    curve: (self.curve),
                    line_spacing: (self.line_spacing),
                    top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );

        self.draw_text.text_style.font = font;
        self.draw_text.wrap = self.wrap.clone();
        self.draw_text.redraw(cx);

        self.draw_text.draw_walk(
            cx,
            walk.with_add_padding(self.padding),
            self.align,
            self.text.as_ref(),
        );

        DrawStep::done()
    }
    /// copy label text
    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx)
    }
}
