//! 暂不开启使用
use std::{collections::HashMap, fmt::Display, str::FromStr};

use gen_parser::{
    common::{BuiltinColor, Hex, LinearGradient, MakepadShader, RadialGradient, Rgb, Rgba},
    Value,
};
use gen_utils::error::Errors;
use proc_macro2::TokenStream;

use crate::{
    prop::builtin::utils::{draw_linear_gradient, draw_radial_gradient, hex_to_pixel},
    widget::utils::f32_prop,
};

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
    pub instances: HashMap<String, String>,
}

impl DrawQuad {
    pub fn add_instance(&mut self, key: &str, value: &str) -> () {
        self.instances.insert(key.to_string(), value.to_string());
    }
    pub fn draw_depth(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.draw_depth.replace(f);
        })
    }
    pub fn pixel(&mut self, value: &Value) -> Result<(), Errors> {
        let quad = DrawQuad::try_from(value)?;
        self.pixel = quad.pixel;
        Ok(())
    }
    /// try from builtin color  and back color hex
    pub fn try_from_back(value: &Value) -> Result<(Self, Option<Hex>), Errors> {
        let color = BuiltinColor::try_from(value)?;
        match &color {
            BuiltinColor::Hex(hex) => Ok((hex.into(), Some(hex.clone()))),
            BuiltinColor::Rgb(rgb) => Ok((rgb.into(), Some(rgb.into()))),
            BuiltinColor::Rgba(rgba) => Ok((rgba.into(), Some(rgba.into()))),
            BuiltinColor::LinearGradient(linear) => Ok((linear.into(), None)),
            BuiltinColor::RadialGradient(radial) => Ok((radial.into(), None)),
            BuiltinColor::Shader(shader) => Ok((shader.into(), None)),
        }
    }
}

impl TryFrom<&Value> for DrawQuad {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let color = BuiltinColor::try_from(value)?;

        match &color {
            BuiltinColor::Hex(hex) => Ok(hex.into()),
            BuiltinColor::Rgb(rgb) => Ok(rgb.into()),
            BuiltinColor::Rgba(rgba) => Ok(rgba.into()),
            BuiltinColor::LinearGradient(linear) => Ok(linear.into()),
            BuiltinColor::RadialGradient(radial) => Ok(radial.into()),
            BuiltinColor::Shader(shader) => Ok(shader.into()),
        }
    }
}

impl From<&str> for DrawQuad {
    fn from(value: &str) -> Self {
        let hex = Hex::from_str(value).unwrap();
        DrawQuad::from(&hex)
    }
}

impl From<&Hex> for DrawQuad {
    fn from(value: &Hex) -> Self {
        let pixel = hex_to_pixel(value);
        DrawQuad {
            pixel,
            draw_depth: None,
            instances: Default::default(),
        }
    }
}

impl From<&Rgb> for DrawQuad {
    fn from(value: &Rgb) -> Self {
        let hex: Hex = value.into();
        DrawQuad::from(&hex)
    }
}

impl From<&Rgba> for DrawQuad {
    fn from(value: &Rgba) -> Self {
        let hex: Hex = value.into();
        DrawQuad::from(&hex)
    }
}

impl From<&MakepadShader> for DrawQuad {
    fn from(value: &MakepadShader) -> Self {
        DrawQuad {
            pixel: value.0.clone(),
            draw_depth: None,
            instances: Default::default(),
        }
    }
}

/// ```rust
/// let gradient_angle = 45.0;
/// let direction = vec2(cos(radians(gradient_angle)), sin(radians(gradient_angle)));
/// let factor = dot(self.pos, direction);
///
/// let color1 = vec4(1.0, 0.0, 0.0, 1.0); // 红色
/// let stop1 = 0.0;
///
/// let color2 = vec4(1.0, 1.0, 0.0, 1.0); // 黄色
/// let stop2 = 0.25;
///
/// let color3 = vec4(0.0, 1.0, 0.0, 1.0); // 绿色
/// let stop3 = 0.5;
///
/// let color4 = vec4(0.0, 1.0, 1.0, 1.0); // 青色
/// let stop4 = 0.75;
///
/// let color5 = vec4(0.0, 0.0, 1.0, 1.0); // 蓝色
/// let stop5 = 1.0;
///
/// let color = mix(
///     mix(
///         mix(
///             mix(color1, color2, smoothstep(stop1, stop2, factor)),
///             color3, smoothstep(stop2, stop3, factor)
///         ),
///         color4, smoothstep(stop3, stop4, factor)
///     ),
///     color5, smoothstep(stop4, stop5, factor)
/// );
///
/// return color;
/// ```
impl From<&LinearGradient> for DrawQuad {
    fn from(value: &LinearGradient) -> Self {
        let pixel = draw_linear_gradient(value, "pixel");

        DrawQuad {
            pixel,
            draw_depth: None,
            instances: Default::default(),
        }
    }
}

// let center = vec2(0.5, 0.5); // 定义中心点在视图的中心
// let distance = distance(self.pos, center); // 计算像素位置到中心点的距离
// let factor = clamp(distance, 0.0, 1.0); // 将距离值限制在0到1之间
// return mix(#d, #FF00FF, factor); // 使用距离作为混合因子
impl From<&RadialGradient> for DrawQuad {
    fn from(value: &RadialGradient) -> Self {
        let pixel = draw_radial_gradient(value, "pixel");

        DrawQuad {
            pixel,
            draw_depth: None,
            instances: Default::default(),
        }
    }
}

impl Display for DrawQuad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instance_str = self.instances.iter().fold(String::new(), |acc, (k, v)| {
            format!("{}{}: {}, ", acc, k, v)
        });
        let _ = f.write_str(&instance_str);

        if let Some(draw_depth) = self.draw_depth.as_ref() {
            let _ = f.write_fmt(format_args!("draw_depth: {}, ", draw_depth));
        }

        f.write_str(self.pixel.to_string().as_str())
    }
}
