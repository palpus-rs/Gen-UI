use font_atlas::CxFontsAtlasRc;
use makepad_widgets::*;
use shader::draw_text::TextWrap;
use crate::{shader::draw_text::DrawGText, utils::get_font_family};

live_design! {
    GIconBase = {{GIcon}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        // draw_button: {pressed: 0.0, hover: 0.0}
                        // draw_icon: {pressed: 0.0, hover: 0.0}
                        draw_text: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        // draw_button: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        // draw_icon: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        // draw_button: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        // draw_icon: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GIcon {
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
    #[live(true)]
    pub visible: bool,
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
    #[animator]
    animator: Animator,
    #[rust]
    area: Area,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum GIconEvent {
    Hovered(KeyModifiers),
    Clicked(KeyModifiers),
    Released(KeyModifiers),
    Pressed(KeyModifiers),
    None,
}



impl Widget for GIcon {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font;
        
        self.draw_text.draw_walk(
            cx,
            walk.with_add_padding(self.padding),
            self.align,
            self.text.as_ref(),
        );

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_text.redraw(cx);
        }
       
        match event.hits(cx, self.area) {
            Hit::FingerDown(f_down) => {
                // if self.grab_key_focus {
                //     cx.set_key_focus(self.draw_text.area());
                // }
                cx.widget_action(uid, &scope.path, GIconEvent::Pressed(f_down.modifiers));
                self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(h) => {
                dbg!("sadasds");
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GIconEvent::Hovered(h.modifiers));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, GIconEvent::Clicked(f_up.modifiers));
                    cx.widget_action(uid, &scope.path, GIconEvent::Released(f_up.modifiers));
                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    cx.widget_action(uid, &scope.path, GIconEvent::Released(f_up.modifiers));
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
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
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GIcon {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
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
        self.draw_text.wrap = self.wrap.clone();
        self.area = self.draw_text.area();
        self.draw_text.redraw(cx);

    }
}