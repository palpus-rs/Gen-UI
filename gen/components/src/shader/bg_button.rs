use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawGButton = {{DrawGButton}}{
        instance inset: vec4(0.0, 0.0, 0.0, 0.0)
        
        fn get_color(self) -> vec4 {
            return self.background_color
        }

        fn get_border_color(self) -> vec4 {
            return self.border_color
        }

        fn get_border_width(self) -> float {
            return self.border_width
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size)
            sdf.box(
                self.inset.x + self.border_width,
                self.inset.y + self.border_width,
                self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                max(1.0, self.border_radius)
            )
            sdf.fill_keep(self.get_color())
            // if self.border_width > 0.0 {
            //     sdf.stroke(self.get_border_color(), self.border_width)
            // }

            sdf.stroke(self.get_border_color(), self.border_width)
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGButton{
    #[deref] pub draw_super: DrawQuad,
    #[live] pub background_color: Vec4,
    #[live] pub border_color: Vec4,
    #[live(0.0)] pub border_width: f32,
    #[live(4.0)] pub border_radius: f32,
}