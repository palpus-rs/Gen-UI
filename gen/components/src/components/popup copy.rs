use icon_atlas::HashMap;
use makepad_widgets::*;

use crate::{
    shader::draw_card::DrawCard,
    themes::{get_color, Themes}, utils::set_cursor,
};

use super::card::{Card, CardEvent, DrawState};

live_design! {
    // GPopupItemBase = {{GPopupItem}} {}
    GPopupBase = {{GPopup}} {}
}

/// A popup is a floating window that appears on top of other content
/// It can be used to display an additional information or to ask for a confirmation
#[derive(Live, LiveRegister)]
pub struct GPopup {
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
    pub scroll_bars: Option<LivePtr>,
    #[rust]
    pub scroll_bars_obj: Option<Box<ScrollBars>>,
    // control ---------------------
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub block_signal_event: bool,
    // deref ---------------------
    #[live]
    pub draw_card: DrawCard,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    #[rust]
    pub children: ComponentMap<LiveId, WidgetRef>,
    #[rust]
    pub draw_order: Vec<LiveId>,
    #[live]
    event_order: EventOrder,
    #[rust]
    pub defer_walks: Vec<(LiveId, DeferWalk)>,
    #[animator]
    animator: Animator,
    #[rust]
    find_cache: HashMap<u64, WidgetSet>,
    /// draw list is necessary!!!
    /// because we need to draw the popup on top of everything
    /// although the name of DrawList2d may let you think it's only for 2d list drawing
    /// actually it's for all the drawing that needs to be on top of everything!!! 
    #[live] draw_list: DrawList2d,
}

impl LiveHook for GPopup {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            self.draw_order.clear();
        }
    }
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
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

impl GPopup {
    pub fn area(&self) -> Area{
        self.draw_card.area()
    }
    /// ## Get the popup container position
    /// if you need to get the position of the popup container, use this method
    /// ### Example
    /// ```rust
    /// let global = cx.global::<PopupMenuGlobal>().clone();
    /// let mut map = global.map.borrow_mut();
    /// let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
    /// if let Event::MouseDown(e) = event {
    ///     if !popup_menu.menu_contains_pos(cx, e.abs) {
    ///         self.close(cx);
    ///         self.animator_play(cx, id!(hover.off));
    ///         return;
    ///     }
    /// }
    /// ```
    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.draw_card.area().clipped_rect(cx).contains(pos)
    }
    /// ## Begin to draw popup
    /// this method is used to begin drawing the popup
    pub fn begin(&mut self, cx: &mut Cx2d) {
        self.draw_list.begin_overlay_reuse(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());

        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                // visible is false, so we are done
                self.draw_state.end();
                return;
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
                .begin(cx, self.walk, self.layout.with_scroll(scroll));
        }
    }
    /// ## End to draw popup
    pub fn end(&mut self, cx: &mut Cx2d, scope: &mut Scope, shift_area: Area, shift: DVec2) {
        while let Some(DrawState::DeferWalk(step)) = self.draw_state.get() {
            if step < self.defer_walks.len() {
                let (id, d_walk) = &mut self.defer_walks[step];
                if let Some(child) = self.children.get_mut(&id) {
                    let walk = d_walk.resolve(cx);
                    scope
                        .with_id(*id, |scope| child.draw_walk(cx, scope, walk))
                        .expect("popup draw items need scope with id");
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                let area = self.draw_card.area();

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
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
    }
    pub fn redraw(&mut self, cx: &mut Cx){
        self.draw_list.redraw(cx);
        // self.draw_card.redraw(cx);
    }
    /// ## Draw items
    pub fn draw_items(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
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
                        scope
                            .with_id(id, |scope| child.draw_walk(cx, scope, walk))
                            .expect("popup draw items need scope with id");
                    }
                }
                // set the next step
                self.draw_state.set(DrawState::Drawing(step + 1, false));
            } else {
                self.draw_state.set(DrawState::DeferWalk(0));
            }
        }
    }
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        
    ) {
       
        // // let mut actions = Vec::new();
        // for (item_id, node) in self.children.iter_mut(){
        //     node.handle_event(cx, event, scope)
        // }
        // // for (node_id, action) in actions {
            
        // // }
        // let uid = self.widget_uid();
        let uid = WidgetUid(cx.redraw_id());
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if self.block_signal_event {
            if let Event::Signal = event {
                return;
            }
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
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, &mut Vec::new());
        }
    }
}
