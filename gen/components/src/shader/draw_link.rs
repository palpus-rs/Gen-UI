use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    DrawLink = {{DrawLink}}{
        instance inset: vec4(0.0, 0.0, 0.0, 0.0)
        instance hover: 0.0,
        instance pressed: 0.0,

        fn stroke_color(self) -> vec4 {
            return mix(
                mix(
                    self.underline_color,
                    self.hover_color,
                    self.hover
                ),
                self.pressed_color,
                self.pressed
            )
        }

        fn get_color(self) -> vec4 {
            return mix(
                mix(
                    self.background_color,
                    self.hover_color,
                    self.hover
                ),
                self.pressed_color,
                self.pressed
            )
        }

        fn get_border_color(self) -> vec4 {
            return self.border_color
        }

        fn get_border_width(self) -> float {
            return self.border_width
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size)

            // sdf.box(
            //     self.inset.x + self.border_width,
            //     self.inset.y + self.border_width,
            //     self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
            //     self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
            //     max(1.0, self.border_radius)
            // )
            if self.transparent == 0.0 {
               sdf.fill_keep(self.get_color())
            }
            let offset_y = 1.0
            sdf.move_to(0., self.rect_size.y - offset_y);
            sdf.line_to(self.rect_size.x, self.rect_size.y - offset_y);
            // sdf.stroke(self.get_border_color(), self.border_width)
            sdf.stroke(self.stroke_color(), self.underline_width)
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawLink{
    #[deref] pub draw_super: DrawQuad,
    #[live] pub background_color: Vec4,
    #[live(1.0)] pub underline: f32,
    #[live] pub underline_color: Vec4,
    #[live(1.0)] pub underline_width: f32,
    #[live] pub hover_color: Vec4,
    #[live] pub pressed_color: Vec4,
    #[live] pub transparent: f32,
    #[live] pub border_color: Vec4,
    #[live(0.0)] pub border_width: f32,
    #[live(4.0)] pub border_radius: f32,
}