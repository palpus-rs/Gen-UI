use makepad_widgets::*;
live_design! { 
    import makepad_widgets :: base ::*; 
    import makepad_widgets :: theme_desktop_dark ::*; 
    Hello = {{ Hello }}{ 
        first_lb = < label >{ draw_text : { text_style : { font_size : 32 , brightness : 1.1 , } , wrap : Word , color : # ffffff , } , } 
        second_lb = < label >{ draw_text : { text_style : { brightness : 1.1 , } , wrap : Word , color : # ffffff , } , text : "label 2" , } 
        bb = < button >{ text : "text btn" , }
    } 
}

#[derive(Live, LiveHook, Widget)]
pub struct Hello {
    #[live]
    pub label1: String,
    #[redraw]
    #[rust]
    pub area: Area,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
}
#[derive(DefaultNone, Clone, Debug)]
pub enum Events {
    Clicked(String),
    None,
}
impl Widget for Hello {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        self.label(id!(first_lb))
            .apply_over_and_redraw(cx, live! { text : (self . label1) , });
        let fs: f64 = 18.0;
        self.label(id!(second_lb)).apply_over_and_redraw(
            cx,
            live! { draw_text : { text_style : { font_size : (fs) , } , } , },
        );
        cx.end_turtle();
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if let Event::Actions(actions) = event {
            if self.button(id!(bb)).clicked(actions) {
                let mut btn_click = || {
                    println!("Button bb Clicked");
                    cx.widget_action(uid, &scope.path, Events::Clicked("Hello".to_string()));
                };
                btn_click()
            }
        }
    }
}
