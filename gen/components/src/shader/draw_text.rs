use makepad_widgets::*;

live_design!{    
    import makepad_draw::shader::std::*;

    DrawGText = {{DrawGText}} {
        instance hover: 0.0,
        instance pressed: 0.0,

        text_style: {
            font_size: 12.0,
        }

        fn get_color(self) -> vec4 {
            return mix(
                self.color,
                mix(self.hover_color, self.pressed_color, self.pressed),
                self.hover
            )
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGText {
    #[deref]
    pub draw_super: DrawText,
    #[live] pub hover_color: Vec4,
    #[live] pub pressed_color: Vec4,
}