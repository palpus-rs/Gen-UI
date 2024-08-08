use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; Button01J4SQSJKN8APB68HNM6XNS5ZC = { { Button01J4SQSJKN8APB68HNM6XNS5ZC } } { item : < Button >{ } } }
#[derive(Live, Widget)]
pub struct Button01J4SQSJKN8APB68HNM6XNS5ZC {
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
impl Widget for Button01J4SQSJKN8APB68HNM6XNS5ZC {
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

impl LiveHook for Button01J4SQSJKN8APB68HNM6XNS5ZC {
    fn before_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.btnlist = vec!["a".to_string(), "b".to_string()];
    }
}

impl Button01J4SQSJKN8APB68HNM6XNS5ZCRef {
    pub fn set_btnlist(&mut self, looper: Vec<String>) {
        if let Some(mut instance) = self.borrow_mut() {
            instance.btnlist = looper;
        }
    }
}
