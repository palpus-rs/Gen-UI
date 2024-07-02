use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Font, Position, Size, Text},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_text::DrawText, Align, Padding, Walk},
        ABS_POS, ALIGN, BRIGHTNESS, COLOR, COMBINE_SPACES, CURVE, DRAW_DEPTH, DRAW_TEXT, FONT,
        FONT_SCALE, FONT_SIZE, HEIGHT, HEIGHT_FACTOR, INGORE_NEWLINES, LINE_SPACING, MARGIN,
        PADDING, PATH, TEXT, TEXT_STYLE, TOP_DROP, WIDTH, WRAP,
    },
    widget::{
        prop_ignore,
        utils::{bind_prop_value, quote_prop, string_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct LabelProps {
    pub draw_text: Option<DrawText>,
    pub walk: Option<Walk>,
    pub align: Option<Align>,
    pub padding: Option<Padding>,
    pub text: Option<String>,
}

impl DynProps for LabelProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &Value,
        is_prop: bool,
        ident: &str,
    ) -> TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            // ----------------- draw_text -----------------
            Font::FONT_FAMILY => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, FONT, PATH], &value),
            Font::FONT_SIZE => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, FONT_SIZE], &value),
            Font::BRIGHTNESS => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, BRIGHTNESS], &value),
            Font::CURVE => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, CURVE], &value),
            Font::LINE_SPACING => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, LINE_SPACING], &value),
            Font::TOP_DROP => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, TOP_DROP], &value),
            Font::HEIGHT_FACTOR => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, HEIGHT_FACTOR], &value),
            Text::TEXT_WRAP => quote_prop(vec![DRAW_TEXT, WRAP], &value),
            Text::IGNORE_NEWLINES => quote_prop(vec![DRAW_TEXT, INGORE_NEWLINES], &value),
            Text::COMBINE_SPACES => quote_prop(vec![DRAW_TEXT, COMBINE_SPACES], &value),
            Font::FONT_SCALE => quote_prop(vec![DRAW_TEXT, FONT_SCALE], &value),
            Text::DRAW_DEPTH => quote_prop(vec![DRAW_TEXT, DRAW_DEPTH], &value),
            Text::COLOR => quote_prop(vec![DRAW_TEXT, COLOR], &value),
            // ----------------- walk -----------------
            Size::HEIGHT => quote_prop(vec![HEIGHT], &value),
            Size::WIDTH => quote_prop(vec![WIDTH], &value),
            Position::ABS_POS => quote_prop(vec![ABS_POS], &value),
            Size::MARGIN => quote_prop(vec![MARGIN], &value),
            Size::PADDING => quote_prop(vec![PADDING], &value),
            Position::ALIGN => quote_prop(vec![ALIGN], &value),
            Text::TEXT => quote_prop(vec![TEXT], &value),
            _ => panic!("cannot match prop in BuiltIn label"),
        }
    }
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
        let _ = match prop_name {
            // ----------------- draw_text -----------------
            Font::FONT_FAMILY => self.font(&value),
            Font::FONT_SIZE => self.font_size(&value),
            Font::BRIGHTNESS => self.brightness(&value),
            Font::CURVE => self.curve(&value),
            Font::LINE_SPACING => self.line_spacing(&value),
            Font::TOP_DROP => self.top_drop(&value),
            Font::HEIGHT_FACTOR => self.height_factor(&value),
            Text::TEXT_WRAP => self.wrap(&value),
            Text::IGNORE_NEWLINES => self.ignore_newlines(&value),
            Text::COMBINE_SPACES => self.combine_spaces(&value),
            Font::FONT_SCALE => self.font_scale(&value),
            Text::DRAW_DEPTH => self.draw_depth(&value),
            Text::COLOR => self.color(&value),
            // ----------------- walk -----------------
            Size::HEIGHT => self.height(&value),
            Size::WIDTH => self.width(&value),
            Position::ABS_POS => self.abs_pos(&value),
            Size::MARGIN => self.margin(&value),
            Size::PADDING => self.padding(&value),
            Position::ALIGN => self.align(&value),
            Text::TEXT => self.text(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("cannot match prop: {}", prop_name);
                } else {
                    panic!("unslolved prop: {}", prop_name);
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
        string_prop(value, |s| {
            let _ = self.text.replace(s.to_string());
        })
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
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_TEXT, draw_text));
        }
        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{}", walk.to_string()));
        }
        if let Some(align) = &self.align {
            let _ = f.write_fmt(format_args!("{}: {},", ALIGN, align.to_string()));
        }
        if let Some(padding) = &self.padding {
            let _ = f.write_fmt(format_args!("{}: {},", PADDING, padding.to_string()));
        }
        if let Some(text) = &self.text {
            let _ = f.write_fmt(format_args!("{}: \"{}\",", TEXT, text));
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod test_label_props {

    use crate::{
        prop::builtin::{
            draw_text::{DrawText, TextStyle},
            TextWrap, Walk,
        },
        ToToken,
    };

    use super::LabelProps;

    #[test]
    fn to_tk() {
        let mut label = LabelProps::default();
        label.text = Some("Hello".to_string());
        let mut draw_text = DrawText::default();
        draw_text.text_style = Some(TextStyle {
            font: Some(
                "crate://self/resources/icons/Icon_Search.svg"
                    .try_into()
                    .unwrap(),
            ),
            font_size: Some(12.0_f64.try_into().unwrap()),
            brightness: Some(0.5.try_into().unwrap()),
            curve: Some(0.5.try_into().unwrap()),
            line_spacing: Some(1.5_f64.try_into().unwrap()),
            top_drop: Some(1.0_f64.try_into().unwrap()),
            height_factor: Some(1.0_f64.try_into().unwrap()),
        });
        draw_text.wrap = Some(TextWrap::Ellipsis);
        draw_text.ignore_newlines = Some(true);
        draw_text.combine_spaces = Some(true);
        draw_text.font_scale = Some(1.0_f64.try_into().unwrap());
        draw_text.draw_depth = Some(0.5_f32.try_into().unwrap());
        draw_text.color = Some("#445566".try_into().unwrap());

        label.draw_text = Some(draw_text);

        label.align = Some("0.5 0.5".try_into().unwrap());
        label.padding = Some("6 0.2 0.5 1.5".try_into().unwrap());
        let mut walk = Walk::default();
        walk.abs_pos = Some("10 10".try_into().unwrap());
        walk.margin = Some("10 10 10 10".try_into().unwrap());
        walk.width = Some("100".try_into().unwrap());
        walk.height = Some("100".try_into().unwrap());
        label.walk = Some(walk);

        let tk = label.to_token_stream();
        let prop = "draw_text : { text_style : { font : dep (\"crate://self/resources/icons/Icon_Search.svg\") , font_size : 12 , brightness : 0.5 , curve : 0.5 , line_spacing : 1.5 , top_drop : 1 , height_factor : 1 , } , wrap : Ellipsis , ignore_newlines : true , combine_spaces : true , font_scale : 1 , draw_depth : 0.5 , color : { # 445566 } , } , abs_pos : { x : 10 , y : 10 } , margin : { top : 10 , right : 10 , bottom : 10 , left : 10 } , width : 100 , height : 100 , align : { x : 0.5 , y : 0.5 } , padding : { top : 6 , right : 0.2 , bottom : 0.5 , left : 1.5 } , text : \"Hello\" ,";
        assert_eq!(prop, tk.to_string().as_str());
    }
}
