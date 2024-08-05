use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawGLoading = {{DrawGLoading}}{
        
        fn pixel(self) -> vec4 {
            let loading_size = vec2(self.width, self.height);
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let loading_dot_size = vec2(loading_size.x * 0.2 * 0.96);
            let rotate_time = self.time * 0.2;
            let counter = 0.0;
            // draw 16 dots around as a loading animation
            for i in 0..16{
                // each dot is a circle and we place it around the circle, with a bit of spacing
                // there are 16 dots so angle is 0.125PI
                let angle = 0.125 * 3.1415926;
                // now we use time to rotate the dots
                let dot_pos = vec2(
                    self.rect_size.x * 0.5 - cos(angle * counter + rotate_time) * loading_size.x * 0.5,
                    self.rect_size.y * 0.5 - sin(angle * counter + rotate_time) * loading_size.y * 0.5
                );
                
                sdf.circle(dot_pos.x, dot_pos.y, loading_dot_size.x * 0.5);
                sdf.fill(self.background_color + vec4(counter* 0.05, counter* 0.05, counter* 0.05, -0.0525 * counter));
                counter += 1.0;
            }
            
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister)]
#[repr(C)]
pub struct DrawGLoading{
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub background_color: Vec4, 
    #[live(64.0)]
    pub height: f32,
    #[live(64.0)]
    pub width: f32,
   
}

impl LiveHook for DrawGLoading {
     
}