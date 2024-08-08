use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; Button01J4RCH40K2SS3T56592HYZXMA = { { Button01J4RCH40K2SS3T56592HYZXMA } } { item : "< Button >{ }" } }
#[derive(Live, Widget, LiveHook)]
pub struct Button01J4RCH40K2SS3T56592HYZXMA {
    #[redraw]
    #[rust]
    area: Area,
    #[live]
    item: Option<LivePtr>,
    #[rust]
    children: ComponentMap<LiveId, ButtonRef>,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[rust]
    pub btnlist: Vec<String>,
}
impl Widget for Button01J4RCH40K2SS3T56592HYZXMA {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        for (index, value) in self.btnlist.iter().enumerate() {
            let target = self.children.get_or_insert(cx, LiveId(index as u64), |cx| {
                WidgetRef::new_from_ptr(cx, self.item).as_button()
            });
            target.set_text(value);
            target.draw_all(cx, &mut Scope::empty());
        }
        cx.end_turtle();
        self.children.retain_visible();
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.children
            .iter()
            .enumerate()
            .for_each(|(_index, (_id, widget_ref))| {
                widget_ref.handle_event(cx, event, scope);
            });
    }
}
impl Button01J4RCH40K2SS3T56592HYZXMARef {
    pub fn set_btnlist(&mut self, looper: Vec<String>) {
        if let Some(mut instance) = self.borrow_mut() {
            instance.btnlist = looper;
        }
    }
}
