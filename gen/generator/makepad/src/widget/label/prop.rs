use std::{collections::HashMap, fmt::Display};

use gen_converter::error::Errors;
use gen_parser::Value;
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_text::DrawText, Align, Padding, Walk},
        ABS_POS, ALIGN, BRIGHTNESS, COLOR, COMBINE_SPACES, CURVE, DRAW_DEPTH, DRAW_TEXT, FONT,
        FONT_SCALE, FONT_SIZE, HEIGHT, HEIGHT_FACTOR, INGORE_NEWLINES, LINE_SPACING, MARGIN,
        PADDING, TEXT, TOP_DROP, WIDTH, WRAP,
    },
    widget::{prop_ignore, utils::string_prop, StaticProps}, ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct LabelProps {
    pub draw_text: Option<DrawText>,
    pub walk: Option<Walk>,
    pub align: Option<Align>,
    pub padding: Option<Padding>,
    pub text: Option<String>,
}

impl StaticProps for LabelProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut label = LabelProps::default();
        for (k, v) in props {
            label.prop(k.name(), v.clone())
        }
        label
    }

    fn prop(&mut self, prop_name: &str, value: gen_parser::Value) -> () {
        match prop_name {
            // ----------------- draw_text -----------------
            FONT => self.font(&value),
            FONT_SIZE => self.font_size(&value),
            BRIGHTNESS => self.brightness(&value),
            CURVE => self.curve(&value),
            LINE_SPACING => self.line_spacing(&value),
            TOP_DROP => self.top_drop(&value),
            HEIGHT_FACTOR => self.height_factor(&value),
            WRAP => self.wrap(&value),
            INGORE_NEWLINES => self.ignore_newlines(&value),
            COMBINE_SPACES => self.combine_spaces(&value),
            FONT_SCALE => self.font_scale(&value),
            DRAW_DEPTH => self.draw_depth(&value),
            COLOR => self.color(&value),
            // ----------------- walk -----------------
            HEIGHT => self.height(&value),
            WIDTH => self.width(&value),
            ABS_POS => self.abs_pos(&value),
            MARGIN => self.margin(&value),
            PADDING => self.padding(&value),
            ALIGN => self.align(&value),
            TEXT => self.text(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("cannot match prop");
                } else {
                    panic!("unslolved prop");
                }
            }
        };
    }
}

impl ToToken for LabelProps {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl LabelProps {
    fn check_draw_text(&mut self) -> &mut DrawText {
        if self.draw_text.is_none() {
            self.draw_text = Some(DrawText::default());
        }
        self.draw_text.as_mut().unwrap()
    }
    fn font(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().font(value)
    }
    fn font_size(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().font_size(value)
    }
    fn brightness(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().brightness(value)
    }
    fn curve(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().curve(value)
    }
    fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().line_spacing(value)
    }
    fn top_drop(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().top_drop(value)
    }
    fn height_factor(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().height_factor(value)
    }
    fn wrap(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().wrap(value)
    }
    fn ignore_newlines(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().ignore_newlines(value)
    }
    fn combine_spaces(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().combine_spaces(value)
    }
    fn font_scale(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().font_scale(value)
    }
    fn draw_depth(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_depth(value)
    }
    fn color(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().color(value)
    }

    fn text(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {let _ = self.text.replace(s.to_string());})
    }
    fn padding(&mut self, value: &Value) -> Result<(), Errors> {
        self.padding = Some(Padding::try_from(value)?);
        Ok(())
    }
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.align = Some(Align::try_from(value)?);
        Ok(())
    }
    fn check_walk(&mut self) -> &mut Walk {
        if self.walk.is_none() {
            self.walk = Some(Walk::default());
        }
        self.walk.as_mut().unwrap()
    }
    fn height(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().height(value)
    }
    fn width(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().width(value)
    }
    fn abs_pos(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().abs_pos(value)
    }
    fn margin(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().margin(value)
    }
}

impl Display for LabelProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_text) = &self.draw_text {
            f.write_fmt(format_args!("draw_text: {},", draw_text.to_string()));
        }
        if let Some(walk) = &self.walk {
            f.write_fmt(format_args!("{},", walk.to_string()));
        }
        if let Some(align) = &self.align {
            f.write_fmt(format_args!("align: {},", align.to_string()));
        }
        if let Some(padding) = &self.padding {
            f.write_fmt(format_args!("padding: {},", padding.to_string()));
        }
        if let Some(text) = &self.text {
            f.write_fmt(format_args!("text: {},", text));
        }
        write!(f, "")
    }
}
