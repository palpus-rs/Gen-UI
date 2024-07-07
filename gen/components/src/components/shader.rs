use makepad_widgets::*;
live_design!{
    GShaderBase = {{GShader}} {}
}

#[derive(Live, Widget)]
pub struct GShader {
    #[redraw] #[live] draw_shader: DrawQuad,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] time: f32,
    #[rust] next_frame: NextFrame,
}

impl LiveHook for GShader{
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // starts the animation cycle on startup
        self.next_frame = cx.new_next_frame();
    }
}


impl Widget for GShader{
    fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope
    ){
        if let Some(ne) = self.next_frame.is_event(event) {
            // update time to use for animation
            self.time = (ne.time * 0.001).fract() as f32;
            
            // force updates, so that we can animate in the absence of user-generated events
            self.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_shader.begin(cx, walk, self.layout);
        self.draw_shader.end(cx);
        DrawStep::done()
    }
}