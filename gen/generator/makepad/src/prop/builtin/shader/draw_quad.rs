//! 暂不开启使用
use std::fmt::Display;

use gen_parser::{
    common::{parse_hex_color, MakepadShader},
    Value,
};
use gen_utils::error::Errors;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

use crate::str_to_string_try_from;

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

/// DrawQuad是Makepad最基础的绘制模型
/// 用于绘制矩形
/// 绘制函数为pixel，返回vec4
/// 但是DrawQuad只有draw_depth
#[derive(Clone, Default, Debug)]
pub struct DrawQuad {
    pub pixel: TokenStream,
    pub draw_depth: Option<f32>,
}

impl DrawQuad {
    pub fn pixel(&mut self, value: &Value) -> Result<(), Errors> {
        let quad = DrawQuad::try_from(value)?;
        self.pixel = quad.pixel;
        Ok(())
    }
}

impl TryFrom<&Value> for DrawQuad {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else if let Some(s) = value.is_string_and_get() {
            s.try_into()
        } else {
            value
                .is_fn_and_get()
                .map(|func| {
                    let pixel = MakepadShader::try_from(func)?;
                    Ok(DrawQuad {
                        pixel: pixel.0,
                        draw_depth: None,
                    })
                })
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{} can not convert to DrawQuad",
                        value
                    )))
                })
        }
    }
}

impl TryFrom<&str> for DrawQuad {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match parse_hex_color(value) {
            Ok((input, color)) => {
                if input.is_empty() {
                    return Ok(DrawQuad {
                        pixel: hex_to_pixel(&color),
                        draw_depth: None,
                    });
                }
                Err(Errors::PropConvertFail(format!(
                    "{} is not a right hex color",
                    value
                )))
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} is not a right hex color",
                value
            ))),
        }
    }
}

str_to_string_try_from!(DrawQuad);

impl Display for DrawQuad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pixel.to_string().as_str())
    }
}

/// convert hex to pixel
pub fn hex_to_pixel(value: &str) -> TokenStream {
    let color = parse_str::<TokenStream>(value).unwrap();
    quote! {
        fn pixel(self) -> vec4{
            return #color;
        }
    }
}
