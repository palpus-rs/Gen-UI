use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::LiveDependency, BRIGHTNESS, COLOR, CURVE, DRAW_DEPTH, LINEARIZE, SCALE, SVG_FILE,
    },
    widget::utils::{f32_prop, f64_prop},
    ToToken,
};

use super::draw_color::DrawColor;

#[derive(Clone, Default, Debug)]
pub struct DrawIcon {
    pub brightness: Option<f32>,
    pub curve: Option<f32>,
    pub linearize: Option<f32>,
    pub svg_file: Option<LiveDependency>,
    // pub svg_path: Option<String>,
    // pub translate: DVec2,
    pub scale: Option<f64>,
    pub draw_depth: Option<f32>,
    pub color: Option<DrawColor>,
}

impl ToToken for DrawIcon {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl DrawIcon {
    pub fn brightness(&mut self, value: &Value) -> Result<(), Errors> {
        dbg!(value);
        f32_prop(value, |b| {
            self.brightness = Some(b);
        })
    }
    pub fn curve(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |b| {
            self.curve = Some(b);
        })
    }
    pub fn linearize(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |b| {
            self.linearize = Some(b);
        })
    }
    pub fn svg_file(&mut self, value: &Value) -> Result<(), Errors> {
        self.svg_file = Some(LiveDependency::try_from(value)?);
        Ok(())
    }
    // pub fn svg_path(&mut self, value: &Value) -> Result<(), Errors> {
    //     string_prop(value, |s| {
    //         self.svg_path = Some(s.to_string());
    //     })
    // }
    pub fn scale(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |b| {
            self.scale = Some(b as f64);
        })
    }
    pub fn draw_depth(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |b| {
            self.draw_depth = Some(b);
        })
    }
    pub fn color(&mut self, value: &Value) -> Result<(), Errors> {
        self.color = Some((value, false).try_into()?);
        Ok(())
    }
}

impl Display for DrawIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(brightness) = self.brightness {
            let _ = f.write_fmt(format_args!("{}: {},", BRIGHTNESS, brightness));
        }
        if let Some(curve) = self.curve {
            let _ = f.write_fmt(format_args!("{}: {},", CURVE, curve));
        }
        if let Some(linearize) = self.linearize {
            let _ = f.write_fmt(format_args!("{}: {},", LINEARIZE, linearize));
        }
        if let Some(svg_file) = &self.svg_file {
            let _ = f.write_fmt(format_args!("{}: {},", SVG_FILE, svg_file));
        }
        // if let Some(svg_path) = &self.svg_path {
        //     let _ = f.write_fmt(format_args!("{}: {},", SVG_PATH, svg_path));
        // }
        if let Some(scale) = self.scale {
            let _ = f.write_fmt(format_args!("{}: {},", SCALE, scale));
        }
        if let Some(draw_depth) = self.draw_depth {
            let _ = f.write_fmt(format_args!("{}: {},", DRAW_DEPTH, draw_depth));
        }
        if let Some(color) = &self.color {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", COLOR, color));
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod test_draw_icon {
    use super::*;
    #[test]
    fn to_tk() {
        let mut draw_icon = DrawIcon::default();
        draw_icon.brightness = Some(0.5);
        draw_icon.curve = Some(0.5);
        draw_icon.linearize = Some(0.5);
        draw_icon.svg_file = Some(
            "crate://self/resources/icons/Icon_Search.svg"
                .try_into()
                .unwrap(),
        );

        draw_icon.scale = Some(1.5);
        draw_icon.draw_depth = Some(0.5);
        draw_icon.color = Some(DrawColor::Color("0x000000".to_string()));
        let tk = draw_icon.to_token_stream();
        let prop = "brightness : 0.5 , curve : 0.5 , linearize : 0.5 , svg_file : dep (\"crate://self/resources/icons/Icon_Search.svg\") , scale : 1.5 , draw_depth : 0.5 , color : { # 0x000000 } ,";
        assert_eq!(prop,tk.to_string().as_str());
    }
}
