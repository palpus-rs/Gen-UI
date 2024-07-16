use makepad_widgets::*;

live_design! {
    GPopupBase = {{GPopup}}{
        flow: Down,
        clip_x: true,
        clip_y: true,
        animator: {
            open = {
                default: on
                off = {
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    redraw: true
                    apply: {
                        content_opened: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    redraw: true
                    apply: {
                        content_opened: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GPopup {
    #[rust]
    area: Area,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[find]
    #[redraw]
    #[live]
    content: WidgetRef,
    #[find]
    #[redraw]
    #[live]
    trigger: WidgetRef,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[live]
    content_walk: Walk,
    #[rust]
    rect_size: f64,
    #[live(0.0)]
    content_opened: f64,
    #[animator]
    animator: Animator,
    #[live(false)]
    opened: bool,
    #[rust]
    is_init: bool,
    #[rust]
    trigger_area: Area,
}

#[derive(Clone)]
enum DrawState {
    DrawTrigger,
    DrawContent,
}

impl Widget for GPopup {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.draw_state.begin(cx, DrawState::DrawTrigger) {
            cx.begin_turtle(walk, self.layout);
        }

        if let Some(DrawState::DrawTrigger) = self.draw_state.get() {
            let walk = self.trigger.walk(cx);
            self.trigger.draw_walk(cx, scope, walk)?;
            cx.begin_turtle(
                self.content_walk,
                Layout::flow_down()
                    .with_scroll(dvec2(0.0, self.rect_size * (1.0 - self.content_opened))),
            );
            // self.draw_state.set(DrawState::DrawContent);

            if self.content_opened == 1.0 {
                self.draw_state.set(DrawState::DrawContent);
            } else {
                self.rect_size = cx.turtle().used().y;
                cx.end_turtle();
                
                cx.end_turtle_with_area(&mut self.area);
                // // dbg!(&self.area);
                // self.area = self.trigger_area.clone();
                self.trigger_area = self.area.clone();
                self.draw_state.end();
            }

            // self.trigger_area = self.area.clone();
            // let refw = self.widget(id!("trigger")).a
        }

        if let Some(DrawState::DrawContent) = self.draw_state.get() {
            let walk = self.content.walk(cx);
            self.content.draw_walk(cx, scope, walk)?;
            self.rect_size = cx.turtle().used().y;
            cx.end_turtle();
            cx.end_turtle_with_area(&mut self.area);
            self.draw_state.end();
        }

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            if self.animator.is_track_animating(cx, id!(open)) {
                self.area.redraw(cx);
                self.trigger_area = self.area.clone();
            }
        };

        match event.hits(cx, self.trigger_area) {
            Hit::FingerDown(_) => {
                // self.content_opened = !self.content_opened;
                // // cx.redraw_child_area(self.area);
                // dbg!(self.content_opened);
                dbg!("down");
                if self.content_opened == 1.0 {
                    self.animator_play(cx, id!(open.off))
                } else {
                    self.animator_play(cx, id!(open.on))
                }
                // dbg!(self.content_opened);
            }
            _ => {}
        }
        // self.trigger.handle_event(cx, event, scope);
    }
}

impl LiveHook for GPopup {
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if !self.is_init {
            if self.opened {
                self.content_opened = 1.0;
            } else {
                self.content_opened = 0.0;
            }
            self.is_init = true;
        }
    }
}
