//! 暂不开启使用
use std::fmt::Display;

use quote::quote;

/// ## SDF DrawQuad
/// "signed distance field" (SDF) 的技术来绘制图形。
/// 这种方法允许你通过编写函数来定义图形和颜色，而不仅仅是使用预定义的颜色值
/// ### Example
/// ```rust
/// log = <Icon> {
/// draw_bg: {
///     fn pixel(self) -> vec4 {
///         let sdf = Sdf2d::viewport(self.pos * self.rect_size)
///         sdf.circle(5., 5., 4.);
///         // #FFFFFF40
///         sdf.fill(THEME_COLOR_TEXT_META);
///         let sz = 1.;
///         sdf.move_to(5., 5.);
///         sdf.line_to(5., 5.);
///         sdf.stroke(#a, 0.8);
///         return sdf.result
///     }
///   }
/// }
/// ```
/// 1. **Sdf2d::viewport(self.pos * self.rect_size)**:
///    - 这行代码初始化了一个 `Sdf2d` 对象，设置了视口的大小。`self.pos * self.rect_size` 表示将当前坐标乘以矩形大小，以适应视口。
/// 2. **sdf.circle(5., 5., 4.)**:
///    - 这是在视口的 (5, 5) 位置绘制一个半径为 4 的圆。
/// 3. **sdf.fill(THEME_COLOR_TEXT_META)**:
///    - 使用 `THEME_COLOR_TEXT_META` 填充圆形。`THEME_COLOR_TEXT_META` 是一个预定义的颜色变量。
/// 4. **sdf.move_to(5., 5.)** 和 **sdf.line_to(5., 5.)**:
///    - 这些方法用来定义从 (5, 5) 到 (5, 5) 的线条。这里的代码看起来有点奇怪，因为它实际上定义了一条长度为零的线条。
/// 5. **sdf.stroke(#a, 0.8)**:
///    - 这行代码用 `#a` 颜色和 0.8 的线条宽度描边。`#a` 是一种特定的颜色表示法。
/// 6. **return sdf.result**:
///    - 返回生成的 SDF 结果。
/// ### Example vec4
///
/// 以下是一个完整的示例，展示如何在 Makepad 中使用不同的方法定义和使用颜色：
///
/// ```rust
/// log = <Icon> {
///     draw_bg: {
///         fn pixel(self) -> vec4 {
///             let sdf = Sdf2d::viewport(self.pos * self.rect_size);
///             sdf.circle(5., 5., 4.);
///            
///             // 使用预定义的颜色填充圆形
///             sdf.fill(THEME_COLOR_TEXT_META);
///            
///             // 使用 vec4 定义颜色并描边
///             let custom_color = vec4(1.0, 0.0, 0.0, 1.0); // 红色
///             sdf.stroke(custom_color, 0.8);
///            
///             return sdf.result;
///         }
///     }
/// }
/// ```
#[derive(Clone, Default, Debug)]
pub struct DrawQuad {
    // #[live] pub geometry: GeometryQuad2D,
    pub draw_depth: Option<f32>,
}

impl DrawQuad {
    pub fn pixel(&self) {}
}

impl Display for DrawQuad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            quote! {
                fn pixel(self) -> vec4 {

                }
            }
            .to_string()
            .as_str(),
        )
    }
}
