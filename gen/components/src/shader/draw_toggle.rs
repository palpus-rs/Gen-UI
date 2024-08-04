use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;

    DrawGToggle = {{DrawGToggle}} {

        fn get_background_color(self) -> vec4 {
            return mix(
                mix(
                    self.background_color,
                    self.hover_color,
                    self.hover
                ),
                self.selected_color,
                self.selected
            )
        }

        fn get_stroke_color(self) -> vec4 {
            return mix(
                mix(
                    self.stroke_color,
                    self.stroke_hover_color,
                    self.hover
                ),
                self.stroke_color,
                self.selected
            ) 
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let box_size = vec2(self.rect_size.x - self.border_width *2.0, self.rect_size.y - self.border_width * 2.0);
            let border_radius = self.border_radius;
            match self.toggle_type {
                GToggleType::Round => {
                    border_radius = box_size.y * 0.25;
                }
                GToggleType::Rect => {}
            }
            sdf.box(self.border_width, self.border_width, box_size.x, box_size.y, border_radius);
            sdf.fill_keep(self.get_background_color());
            sdf.stroke(self.border_color, self.border_width);
            let circle = vec2(box_size.y * 0.5 - 1.0);
            let center = self.rect_size.y * 0.5;
            // let sz = 16.0;
            let offset = self.rect_size.x / 20.0;
            // let c = vec2(left + sz, self.rect_size.y * 0.5);
            // let isz = sz * 0.65;
            sdf.circle(mix(
                circle.x + self.border_width + offset,
                self.rect_size.x - circle.x - offset - self.border_width,
                self.selected
            ), center, circle.x);
            
            // sdf.subtract();
            // sdf.circle(left + sz + self.selected * sz, center, circle.x);
            sdf.circle(mix(
                circle.x + self.border_width + offset,
                self.rect_size.x - circle.x - offset - self.border_width,
                self.selected
            ), center, circle.x);

            sdf.blend(self.selected)
            sdf.fill(
               self.get_stroke_color()
            );

            return sdf.result
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGToggle {
    #[deref]
    pub draw_super: DrawQuad,
    // ---- event state
    #[live]
    pub hover: f32, // 盒子的hover状态
    #[live]
    pub selected: f32, // 盒子的选中状态
    // ---- colors
    #[live]
    pub background_color: Vec4, // 盒子的背景色
    #[live]
    pub hover_color: Vec4, // 盒子的hover颜色
    #[live]
    pub selected_color: Vec4,
    #[live]
    pub stroke_color: Vec4, // 盒子中内部绘制的线条颜色
    #[live]
    pub stroke_hover_color: Vec4, // 盒子中内部绘制的线条颜色
    #[live]
    pub border_color: Vec4, // 盒子的边框颜色
    #[live(1.0)]
    pub border_width: f32, // 盒子的边框宽度
    #[live(2.0)]
    pub border_radius: f32, // 盒子的圆角半径
    #[live(0.64)]
    pub scale: f32, // 盒子内部绘制的缩放比例
    #[live]
    pub toggle_type: GToggleType, // 盒子内部绘制的类型
}

#[derive(Live, LiveHook, Clone)]
#[live_ignore]
#[repr(u32)]
pub enum GToggleType {
    #[pick]
    Round = shader_enum(1),
    Rect = shader_enum(2),
}