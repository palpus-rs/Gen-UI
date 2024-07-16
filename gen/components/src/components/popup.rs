use makepad_widgets::*;

use super::card::Card;

live_design!{
    // GPopupItemBase = {{GPopupItem}} {}
    GPopupBase = {{GPopup}} {}
}

#[derive(Live, LiveRegister)]
pub struct GPopup {
    #[deref]
    super_widget: Card
}


impl LiveHook for GPopup {
   
}

impl GPopup {
    
    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.draw_card.area().clipped_rect(cx).contains(pos)
    }
    
    pub fn begin(&mut self, cx: &mut Cx2d) {
        // self.draw_list.begin_overlay_reuse(cx);
        
        cx.begin_pass_sized_turtle(Layout::flow_down());
        let walk = self.walk.clone();
        let layout = self.layout.clone();
        // ok so. this thing needs a complete position reset
        self.draw_card.begin(cx, walk, layout);
        // self.count = 0;
    }
    
    pub fn end(&mut self, cx: &mut Cx2d, shift_area: Area, shift: DVec2) {
        // ok so.
        /*
        let menu_rect1 = cx.turtle().padded_rect_used();
        let pass_rect = Rect {pos: dvec2(0.0, 0.0), size: cx.current_pass_size()};
        let menu_rect2 = pass_rect.add_margin(-dvec2(10.0, 10.0)).contain(menu_rect1);
        */
        //cx.turtle_mut().set_shift(shift + (menu_rect2.pos - menu_rect1.pos));
        //let menu_rect1 = cx.turtle().padded_rect_used();
        self.draw_card.end(cx);
        
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        //cx.debug.rect_r(self.draw_card.area().get_rect(cx));
        // self.draw_list.end(cx);
        // self.menu_items.retain_visible();
        // if let Some(init_select_item) = self.init_select_item.take() {
        //     self.select_item_state(cx, init_select_item);
        // }
    }
    
    
}
