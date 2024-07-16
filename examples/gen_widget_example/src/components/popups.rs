use gen_components::components::{
    button::{GButtonEvent, GButtonWidgetExt, GButtonWidgetRefExt},
    card::{Card, CardWidgetExt},
};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    // GPopupExample = <ScrollYView>{
    //     height: 180.0,
    //     width: Fill,
    //     flow: Down,
    //     spacing: 10.0,
    //     <Label>{
    //         text: "GPopup",
    //     }
    //     <GVLayout>{
    //         height: Fit,
    //         width: 300,
    //         spacing: 10.0,

    //         <Pop>{
    //             trigger = <GButton>{text:"open"},

    //             popup = <GCard>{

    //             }
    //         }
    //         // <GPopup>{
    //         //     height: Fit,
    //         //     width: Fit,
    //         //     trigger: <GButton>{text:"open"},
    //         //     content: <GCard>{
    //         //         height: 60.0,
    //         //         width: 200.0,
    //         //         <GLabel>{
    //         //             text: "Content"
    //         //         }
    //         //     }
    //         // }
    //     }
    // }

    Pop = {{Pop}}{

    }
}

#[derive(Live, Widget)]
pub struct Pop {
    #[deref]
    pop: Card,

}

impl Widget for Pop {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.pop.draw_walk(cx, scope, walk);
        let popup = self.pop.gbutton(id!(trigger));
        let mut pop_walk  =walk.clone();
        
        let abs_pos = popup.area().rect(cx).pos;
        let size = popup.area().rect(cx).size;
       
        pop_walk.height = Size::Fixed(size.y);

        let real_pos = DVec2{
            x: abs_pos.x,
            y: abs_pos.y + size.y,
        };
        
        let content = self.pop.card(id!(popup));

        content.set_abs_pos(cx, real_pos);
        self.walk.height = Size::Fixed(size.y);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.pop.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            match actions
                .find_widget_action(self.pop.widget(id!(trigger)).widget_uid())
                .cast()
            {
                GButtonEvent::Clicked(_) => {
                    let content = self.pop.card(id!(popup));
                    dbg!(content.is_visible());
                    if content.is_visible() {
                        content.set_visible_and_redraw(cx, false)
                    } else {
                        content.set_visible_and_redraw(cx, true)
                    }
                }
                _ => {}
            }
        }
    }
}

impl LiveHook for Pop {
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
       
        
    }
}
