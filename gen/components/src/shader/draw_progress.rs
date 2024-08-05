use makepad_widgets::*;

use super::manual::Direction;

live_design!{
    import makepad_draw::shader::std::*;
    DrawGProgress = {{DrawGProgress}}{

        fn get_background_color(self) -> vec4 {
            return mix(
                self.background_color,
                self.hover_color,
                self.hover
            )
        }
        fn get_stroke_color(self) -> vec4 {
            return mix(
                self.stroke_color,
                self.stroke_hover_color,
                self.hover
            )
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let progress_height = self.rect_size.y - 2.0 * self.border_width;
            let progress_width = self.rect_size.x - 2.0 * self.border_width;
            let progress_bg = self.get_background_color();
            let progress_in_bg = self.get_stroke_color();

            match self.direction {
                Direction::Horizontal => {
                    sdf.box(self.border_width, self.border_width, progress_width, progress_height, self.border_radius);
                    sdf.fill(progress_bg);
                    sdf.stroke(self.border_color, self.border_width);
   
                    sdf.box(
                            self.border_width,
                            self.border_width,
                            self.position * self.rect_size.x,
                            progress_height,
                            self.border_radius
                    )
                    sdf.fill(progress_in_bg);
                }
                Direction::Vertical => {

                }
            }
            return sdf.result
        }
    }
}



#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct  DrawGProgress{
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub position: f32,
    #[live]
    pub direction: Direction,
    #[live]
    pub background_color: Vec4, // 盒子的背景色
    #[live]
    pub hover_color: Vec4, // 盒子的hover颜色
    #[live]
    pub stroke_color: Vec4, // 盒子的背景色
    #[live]
    pub stroke_hover_color: Vec4, // 盒子的hover颜色
    #[live]
    pub border_color: Vec4, // 盒子的边框颜色
    #[live(1.0)]
    pub border_width: f32, // 盒子的边框宽度
    #[live(2.0)]
    pub border_radius: f32, // 盒子的圆角半径
    #[live]
    pub hover: f32, // 盒子的hover状态
}

impl DrawGProgress {
    pub fn apply_progress_type(&mut self, direction: Direction) {
        self.direction = direction;
    }
}