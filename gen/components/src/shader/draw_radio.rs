use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;

    DrawGRadio = {{DrawGRadio}} {
        // uniform size: 20.0;
        // uniform border_radius: 5.0
        // instance bodytop: #00FF00,
        // instance bodybottom: #FF0000,

        fn get_color(self) -> vec4 {
            return mix(
                mix(
                    mix(
                        self.color,
                        self.hover_color,
                        self.hover
                    ),
                    self.hover_color,
                    self.focus
                ),
                self.hover_color,
                self.selected
            )
        }

        fn get_radio_color(self) -> vec4 {
            return mix(
                mix(
                    mix(
                        self.color,
                        self.focus_color,
                        self.hover
                    ),
                    self.focus_color,
                    self.focus
                ),
                self.selected_color,
                self.selected
            )
        }

        fn stroke_color(self) -> vec4 {
            return mix(
                mix(
                    mix(
                        self.border_color,
                        self.hover_color,
                        self.hover
                    ),
                    self.focus_color,
                    self.focus
                ),
                self.selected_color,
                self.selected
            )
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size)
            let sz = self.size;
            let left = sz + self.border_width;
            let dx = 1.0;
            let c = vec2(left + sz, self.rect_size.y * 0.5);
            sdf.circle(left, c.y, sz);
            sdf.fill_keep(self.get_color())
            sdf.stroke(self.border_color, self.border_width)
            match self.radio_type {
                GChooseType::Round => {
                    let isz = sz * self.scale;
                    sdf.circle(left, c.y, isz);
                    sdf.fill(
                        self.get_radio_color()
                    );
                }
                GChooseType::Tick => {
                    let stroke_width = self.size * 0.16;
                    let szs = sz * 0.5;
                    
                    sdf.move_to(c.x / 4, c.y);
                    sdf.line_to(c.x / 2 - dx, c.y + szs - dx);
                    sdf.line_to(c.x / 2 + szs + dx, c.y - szs + dx);
                    sdf.stroke(self.get_radio_color(), stroke_width);
                }
                GChooseType::Cross => {
                    let stroke_width = self.size * pow(self.scale, 1.86);
                    let szs = sz * 0.5;
                    sdf.move_to(szs + dx , c.y );
                    sdf.line_to(c.x - szs , c.y );
                    sdf.stroke(self.get_radio_color(), stroke_width);
                }
            }
            return sdf.result
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGRadio {
    #[deref]
    pub draw_super: DrawQuad,
    // ---- event state
    #[live]
    pub hover: f32,
    #[live]
    pub focus: f32,
    #[live]
    pub selected: f32,
    // ---- colors
    #[live]
    pub color: Vec4,
    #[live]
    pub hover_color: Vec4,
    #[live]
    pub focus_color: Vec4,
    #[live]
    pub selected_color: Vec4,
    #[live]
    pub border_color: Vec4,
    // ---- size
    #[live(8.0)]
    pub size: f32,
    #[live(1.0)]
    pub border_width: f32,
    #[live(0.48)]
    pub scale: f32,
    // ---- type
    #[live]
    pub radio_type: GChooseType,
}

impl DrawGRadio {
    pub fn apply_radio_type(&mut self, radio_type: GChooseType) {
        self.radio_type = radio_type;
    }
}

#[derive(Live, LiveHook, Clone)]
#[live_ignore]
#[repr(u32)]
pub enum GChooseType {
    #[pick]
    /// 🔴 (实心圆)
    Round = shader_enum(1),
    /// ✔️ (勾)
    Tick = shader_enum(2),
    /// ⛔ (横线)
    Cross = shader_enum(3),
}
