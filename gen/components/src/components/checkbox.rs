use makepad_widgets::*;

use crate::{
    shader::{draw_check_box::DrawGCheckBox, draw_radio::GChooseType},
    themes::{get_color, Themes},
    utils::set_cursor,
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GCheckBoxBase = {{GCheckBox}}{
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_check: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_check: {hover: 1.0}
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_check: {selected: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_check: {selected: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Widget, Live)]
pub struct GCheckBox {
    #[live]
    pub theme: Themes,
    #[live(8.0)]
    pub size: f32,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub selected_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(1.6)]
    pub border_width: f32,
    #[live(0.64)]
    pub scale: f32,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    // ---- type
    #[live]
    check_type: GChooseType,
    // deref -------------------
    #[redraw]
    #[live]
    draw_check: DrawGCheckBox,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    // animator -----------------
    #[animator]
    animator: Animator,
}

#[derive(DefaultNone, Clone, Debug)]
pub enum GCheckBoxEvent {
    Change(bool),
    Hover,
    None,
}

impl Widget for GCheckBox {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_check.draw_walk(cx, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.draw_check.area()) {
            Hit::FingerHoverIn(_) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GCheckBoxEvent::Hover)
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_fe) => {
                if self.animator_in_state(cx, id!(selected.on)) {
                    self.animator_play(cx, id!(selected.off));
                    cx.widget_action(uid, &scope.path, GCheckBoxEvent::Change(false));
                }else{
                    self.animator_play(cx, id!(selected.on));
                    cx.widget_action(uid, &scope.path, GCheckBoxEvent::Change(true));
                }
            }
            Hit::FingerUp(_fe) => {}
            Hit::FingerMove(_fe) => {}
            _ => (),
        }
    }
}

impl LiveHook for GCheckBox {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 50);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 100);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 600);
        // ------------------ focus color -----------------------------------------------
        let focus_color = get_color(self.theme, self.focus_color, 600);
        // ------------------ selected color ---------------------------------------------
        let selected_color = get_color(self.theme, self.selected_color, 600);
        // ------------------ apply to draw_check ----------------------------------------
        self.draw_check.apply_over(
            cx,
            live! {
                color: (bg_color),
                hover_color: (hover_color),
                focus_color: (focus_color),
                selected_color: (selected_color),
                border_color: (border_color),
                border_width: (self.border_width),
                scale: (self.scale),
                size: (self.size),
                scale: (self.scale),
            },
        );
        self.draw_check.apply_check_type(self.check_type.clone());

        self.draw_check.redraw(cx);
    }
}
