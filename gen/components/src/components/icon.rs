use makepad_widgets::*;

use crate::{shader::draw_icon::DrawGIcon, themes::{get_color, Themes}, utils::set_cursor};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GIconBase = {{GIcon}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_icon: {hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_icon: { hover: [{time: 0.0, value: 1.0}],}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GIcon {
    #[live]
    theme: Themes,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.6)]
    pub curve: f32,
    #[live(0.5)]
    pub linearize: f32,
    #[live]
    pub src: LiveDependency,
    /// svg path command (todo!)
    #[live]
    pub command: Option<String>,
    #[live(1.0)]
    pub scale: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live(1.0)]
    pub draw_depth: f32,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub cursor: Option<MouseCursor>,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[animator]
    animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    draw_icon: DrawGIcon,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

#[derive(Debug, Clone, DefaultNone)]
pub enum GIconEvent {
    Clicked,
    Hover,
    None,
}

impl Widget for GIcon {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.draw_icon.draw_walk(cx, walk);
      
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        match event.hits(cx, self.draw_icon.area()) {
            Hit::FingerHoverIn(_) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GIconEvent::Hover);
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, GIconEvent::Clicked);

                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
}

impl LiveHook for GIcon {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ color -----------------------------------------------
        let color = get_color(self.theme, self.color, 500);

        self.draw_icon.apply_over(
            cx,
            live! {
                hover_color: (hover_color),
                color: (color),
                brightness: (self.brightness),
                curve: (self.curve),
                linearize: (self.linearize),
                scale: (self.scale),
                draw_depth: (self.draw_depth),
            },
        );

        self.draw_icon.set_src(self.src.clone());

        self.draw_icon.redraw(cx);
    }
}
