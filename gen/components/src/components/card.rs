use crate::shader::draw_card::DrawCard;
use crate::themes::{get_color, Themes};
use crate::utils::set_cursor;
use event::TriggerHitEvent;
use makepad_widgets::*;
/// Card component
live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    CardBase = {{Card}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_card: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_card: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_card: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
            // down = {
            //     default: off,
            //     off = {
            //         from: {
            //             all: Forward {duration: (GLOBAL_DURATION)}
            //         }
            //         apply:{
            //             draw_card: {pressed: 0.0, hover: 0.0}
            //         }
            //     }
            //     on = {
            //         from: {
            //             all: Forward {duration: (GLOBAL_DURATION)}
            //         }
            //         apply:{
            //             draw_card: {pressed: 0.0, hover: 1.0}
            //         }
            //     }
            // }
        }
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct Card {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    #[live(true)]
    pub visible: bool,
    #[live(false)]
    pub transparent: bool,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(false)]
    pub animator_key: bool,
    // scroll ---------------------
    #[live]
    scroll_bars: Option<LivePtr>,
    #[rust]
    scroll_bars_obj: Option<Box<ScrollBars>>,
    // control ---------------------
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub block_signal_event: bool,
    // deref ---------------------
    #[live]
    draw_card: DrawCard,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    children: ComponentMap<LiveId, WidgetRef>,
    #[rust]
    draw_order: Vec<LiveId>,
    #[live]
    event_order: EventOrder,
    #[rust]
    defer_walks: Vec<(LiveId, DeferWalk)>,
    #[animator]
    animator: Animator,
}

#[derive(Clone)]
enum DrawState {
    Drawing(usize, bool),
    DeferWalk(usize),
}

/// copy Hit from makepad_widgets
#[derive(Clone, Debug, DefaultNone)]
pub enum CardEvent {
    // These are not the events we are interested in
    // KeyFocus(KeyFocusEvent),
    // KeyFocusLost(KeyFocusEvent),
    // Trigger(TriggerHitEvent),
    // TextInput(TextInputEvent),
    // TextCopy(TextClipboardEvent),
    // TextCut(TextClipboardEvent),
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    // FingerScroll(FingerScrollEvent),
    FingerDown(FingerDownEvent),
    FingerMove(FingerMoveEvent),
    FingerHoverIn(FingerHoverEvent),
    FingerHoverOver(FingerHoverEvent),
    FingerHoverOut(FingerHoverEvent),
    FingerUp(FingerUpEvent),
    // None is eq Nothing
    None,
}

impl Widget for Card {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // begin the draw state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                // visible is false, so we are done
                self.draw_state.end();
                return DrawStep::done();
            }
            self.defer_walks.clear();
            
            // get scroll position
            let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                scroll_bars.begin_nav_area(cx);
                scroll_bars.get_scroll_pos()
            } else {
                self.layout.scroll
            };

            // begin draw the card
            let _ = self
                .draw_card
                .begin(cx, walk, self.layout.with_scroll(scroll));
        }

        // loop handle the inner children
        while let Some(DrawState::Drawing(step, resumed)) = self.draw_state.get() {
            if step < self.draw_order.len() {
                // get id from draw_order list
                let id = self.draw_order[step];
                // get the child widget by id
                if let Some(child) = self.children.get_mut(&id) {
                    // is the child visible?
                    // true -> draw the child walk
                    if child.is_visible() {
                        let walk = child.walk(cx);
                        // if resumed
                        if !resumed {
                            self.draw_state.set(DrawState::Drawing(step, true));
                        }
                        scope.with_id(id, |scope| child.draw_walk(cx, scope, walk))?;
                    }
                }
                // set the next step
                self.draw_state.set(DrawState::Drawing(step + 1, false));
            } else {
                self.draw_state.set(DrawState::DeferWalk(0));
            }
        }

        // loop handle the defer walk
        while let Some(DrawState::DeferWalk(step)) = self.draw_state.get() {
            if step < self.defer_walks.len() {
                let (id, d_walk) = &mut self.defer_walks[step];
                if let Some(child) = self.children.get_mut(&id) {
                    let walk = d_walk.resolve(cx);
                    scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                let area = self.area();

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.draw_scroll_bars(cx);
                }

                // draw background
                self.draw_card.end(cx);

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.set_area(area);
                    scroll_bars.end_nav_area(cx);
                }
            }
            self.draw_state.end();
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            let mut actions = Vec::new();
            scroll_bars.handle_main_event(cx, event, &mut actions);
            if actions.len().gt(&0) {
                cx.redraw_area_and_children(self.area());
            }
        }

        match &self.event_order {
            EventOrder::Down => {
                for id in self.draw_order.iter() {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            })
                        }
                    }
                }
            }
            EventOrder::Up => {
                // the default event order is Up
                for id in self.draw_order.iter().rev() {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            });
                        }
                    }
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            })
                        }
                    }
                }
            }
        }

        // handle event and set cursor to control
        match event.hits(cx, self.area()) {
            Hit::KeyDown(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, CardEvent::KeyDown(e))
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, CardEvent::KeyUp(e))
                }
            }
            // Hit::FingerScroll(e) => cx.widget_action(uid, &scope.path, CardEvent::FingerScroll(e)),
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
                cx.widget_action(uid, &scope.path, CardEvent::FingerDown(e));
            }
            Hit::FingerMove(e) => cx.widget_action(uid, &scope.path, CardEvent::FingerMove(e)),
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(uid, &scope.path, CardEvent::FingerHoverIn(e));
                if self.animator.live_ptr.is_some() && self.animator_key {
                    self.animator_play(cx, id!(hover.on))
                }
            }
            Hit::FingerHoverOver(e) => {
                cx.widget_action(uid, &scope.path, CardEvent::FingerHoverOver(e));
            }
            Hit::FingerHoverOut(e) => {
                cx.widget_action(uid, &scope.path, CardEvent::FingerHoverOut(e));
                if self.animator.live_ptr.is_some() && self.animator_key {
                    self.animator_play(cx, id!(hover.off))
                }
            }
            Hit::FingerUp(e) => {
                cx.widget_action(uid, &scope.path, CardEvent::FingerUp(e));
            }
            _ => (),
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl WidgetNode for Card {
    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for child in self.children.values_mut() {
            child.find_widgets(path, cached, results);
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_card.redraw(cx);
        for child in self.children.values_mut() {
            child.redraw(cx);
        }
    }
}

impl LiveHook for Card {
    fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            self.draw_order.clear();
        }
    }
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = get_color(self.theme, self.pressed_color, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 800);
        // ------------------ is transparent --------------------------------------------
        let transparent = (self.transparent) as u8 as f32;
        // ------------------ check scroll bar -------------------------------------------
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }
        // ------------------ apply draw_card --------------------------------------------
        self.draw_card.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                transparent: (transparent),
            },
        );
        self.draw_card.redraw(cx);
    }
    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::Animate | ApplyFrom::Over => {
                if let Some(child) = self.children.get_mut(&id) {
                    child.apply(cx, apply, index, nodes)
                } else {
                    nodes.skip_node(index)
                }
            }
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if nodes[index].is_instance_prop() {
                    self.draw_order.push(id);
                    return self
                        .children
                        .get_or_insert(cx, id, |cx| WidgetRef::new(cx))
                        .apply(cx, apply, index, nodes);
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                    nodes.skip_node(index)
                }
            }
            _ => nodes.skip_node(index),
        }
    }
}

impl Card {
    pub fn area(&self) -> Area {
        self.draw_card.area()
    }
}

impl CardRef {
    pub fn set_visible(&self, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible
        }
    }

    pub fn set_visible_and_redraw(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible;
            inner.redraw(cx);
        }
    }

    pub fn visible(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.visible
        } else {
            false
        }
    }

    pub fn area(&self) -> Area {
        if let Some(inner) = self.borrow() {
            inner.draw_card.area()
        } else {
            Area::Empty
        }
    }
}
