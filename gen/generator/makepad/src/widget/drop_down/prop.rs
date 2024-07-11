use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Background, Event, Font, Others, Position, Size, Text},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{
            draw_label_text::DrawLabelText, draw_quad::DrawQuad, Layout, LiveValue,
            PopupMenuPosition, Walk,
        }, ABS_POS, ALIGN, BIND, BRIGHTNESS, CLIP_X, CLIP_Y, COLOR, COMBINE_SPACES, CURVE, DRAW_BG, DRAW_DEPTH, DRAW_TEXT, FLOW, FONT, FONT_SCALE, FONT_SIZE, HEIGHT, HEIGHT_FACTOR, INGORE_NEWLINES, LINE_SPACING, MARGIN, PADDING, PATH, SCROLL, SPACING, TEXT_STYLE, TOP_DROP, WIDTH, WRAP
    },
    props_to_token,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, quote_prop, string_prop, usize_prop, vec_string_to_string},
        DynProps, StaticProps,
    },
    ToToken,
};



#[derive(Debug, Clone, Default)]
pub struct DropDownProps {
    pub draw_bg: Option<DrawQuad>,
    pub draw_text: Option<DrawLabelText>,
    pub walk: Option<Walk>,
    pub bind: Option<String>,
    pub bind_enum: Option<String>,
    // pub popup_menu: Option<LivePtr>,
    pub labels: Option<Vec<String>>,
    pub values: Option<Vec<LiveValue>>,
    pub popup_menu_position: Option<PopupMenuPosition>,
    pub selected_item: Option<usize>,
    pub layout: Option<Layout>,
}

impl DynProps for DropDownProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            // ----------------- draw_bg -----------------
            Background::BACKGROUND_COLOR => quote_prop(vec![DRAW_BG, COLOR], &value),
            // ----------------- draw_text -----------------
            Font::FONT_FAMILY => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, FONT, PATH], &value),
            Font::FONT_SIZE => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, FONT_SIZE], &value),
            Font::BRIGHTNESS => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, BRIGHTNESS], &value),
            Font::CURVE => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, CURVE], &value),
            "text_line_spacing" => quote_prop(vec![DRAW_TEXT, TEXT_STYLE, LINE_SPACING], &value),
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
            // ------------------- layout -----------------
            Others::SCROLL => quote_prop(vec![SCROLL], &value),
            Size::CLIP_X => quote_prop(vec![CLIP_X], &value),
            Size::CLIP_Y => quote_prop(vec![CLIP_Y], &value),
            Size::PADDING => quote_prop(vec![PADDING], &value),
            Position::ALIGN => quote_prop(vec![ALIGN], &value),
            Position::FLOW => quote_prop(vec![FLOW], &value),
            Position::SPACING => quote_prop(vec![SPACING], &value),
            LINE_SPACING => quote_prop(vec![LINE_SPACING], &value),
            Event::BIND => quote_prop(vec![BIND], &value),
            "bind_enum" => quote_prop(vec!["bind_enum"], &value),
            "labels" => quote_prop(vec!["labels"], &value),
            "values" => quote_prop(vec!["values"], &value),
            Position::POPUP_MENU_POSITION => quote_prop(vec!["popup_menu_position"], &value),
            "selected_item" => quote_prop(vec!["selected_item"], &value),
            _ => panic!("cannot match prop in BuiltIn DropDown"),
        }
    }
}

impl StaticProps for DropDownProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = DropDownProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v)
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            // ----------------- draw_bg -----------------
            Background::BACKGROUND_COLOR => self.draw_bg(&value),
            // ----------------- draw_text -----------------
            Font::FONT_FAMILY => self.font(&value),
            Font::FONT_SIZE => self.font_size(&value),
            Font::BRIGHTNESS => self.brightness(&value),
            Font::CURVE => self.curve(&value),
            "text_line_spacing" => self.label_line_spacing(&value),
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
            // ----------------- layout ---------------
            Others::SCROLL => self.scroll(&value),
            Size::CLIP_X => self.clip_x(&value),
            Size::CLIP_Y => self.clip_y(&value),
            Size::PADDING => self.padding(&value),
            Position::ALIGN => self.align(&value),
            Position::FLOW => self.flow(&value),
            Position::SPACING => self.spacing(&value),
            LINE_SPACING => self.line_spacing(&value),
            // ----------------- other -----------------
            Event::BIND => self.bind(&value),
            "bind_enum" => self.bind_enum(&value),
            "labels" => self.labels(&value),
            "values" => self.values(&value),
            Position::POPUP_MENU_POSITION => self.popup_menu_position(&value),
            "selected_item" => self.selected_item(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("<drop_down> cannot match prop: {}", prop_name);
                } else {
                    panic!("<drop_down> unslolved prop: {}", prop_name);
                }
            }
        };
    }
}

#[allow(dead_code)]
impl DropDownProps {
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_bg = Some(DrawQuad::try_from(value).unwrap());
        Ok(())
    }
    fn check_draw_text(&mut self) -> &mut DrawLabelText {
        if self.draw_text.is_none() {
            self.draw_text = Some(DrawLabelText::default());
        }
        self.draw_text.as_mut().unwrap()
    }
    fn check_walk(&mut self) -> &mut Walk {
        if self.walk.is_none() {
            self.walk = Some(Walk::default());
        }
        self.walk.as_mut().unwrap()
    }
    fn check_layout(&mut self) -> &mut Layout {
        if self.layout.is_none() {
            self.layout = Some(Layout::default());
        }
        self.layout.as_mut().unwrap()
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
    fn scroll(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().scroll(value)
    }
    fn clip_x(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().clip_x(value)
    }
    fn clip_y(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().clip_y(value)
    }
    fn padding(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().padding(value)
    }
    fn flow(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().flow(value)
    }
    fn spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().spacing(value)
    }
    fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().line_spacing(value)
    }
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
    fn font(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.font(value)
    }
    fn font_size(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.font_size(value)
    }
    fn brightness(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.brightness(value)
    }
    fn curve(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.curve(value)
    }
    fn label_line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.line_spacing(value)
    }
    fn top_drop(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.top_drop(value)
    }
    fn height_factor(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.height_factor(value)
    }
    fn wrap(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.wrap(value)
    }
    fn ignore_newlines(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.ignore_newlines(value)
    }
    fn combine_spaces(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.combine_spaces(value)
    }
    fn font_scale(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.font_scale(value)
    }
    fn draw_depth(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.draw_depth(value)
    }
    fn color(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.color(value)
    }
    fn bind(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {
            self.bind = Some(s.to_string());
        })
    }
    fn bind_enum(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {
            self.bind_enum = Some(s.to_string());
        })
    }
    fn labels(&mut self, value: &Value) -> Result<(), Errors> {
        match value {
            Value::UnKnown(_) => {
                self.labels.replace(value.to_vec_string()?);
                Ok(())
            }
            _ => Err(Errors::PropConvertFail(format!(
                "cannot convert {:?} to labels",
                value
            ))),
        }
    }
    fn values(&mut self, value: &Value) -> Result<(), Errors> {
        match value {
            Value::UnKnown(_) => {
                self.values.replace(LiveValue::try_from_value_vec(value)?);
                Ok(())
            }
            _ => Err(Errors::PropConvertFail(format!(
                "cannot convert {:?} to values",
                value
            ))),
        }
    }
    fn popup_menu_position(&mut self, value: &Value) -> Result<(), Errors> {
        self.popup_menu_position = Some(PopupMenuPosition::try_from(value)?);
        Ok(())
    }
    fn selected_item(&mut self, value: &Value) -> Result<(), Errors> {
        usize_prop(value, |u| {
            self.selected_item = Some(u);
        })
    }
}

impl Display for DropDownProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_bg) = self.draw_bg.as_ref() {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_BG, draw_bg));
        }
        if let Some(draw_text) = self.draw_text.as_ref() {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_TEXT, draw_text));
        }
        if let Some(walk) = self.walk.as_ref() {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(layout) = self.layout.as_ref() {
            let _ = f.write_fmt(format_args!("{}", layout));
        }
        if let Some(bind) = self.bind.as_ref() {
            let _ = f.write_fmt(format_args!("bind: {}", bind));
        }
        if let Some(bind_enum) = self.bind_enum.as_ref() {
            let _ = f.write_fmt(format_args!("bind_enum: {},", bind_enum));
        }
        if let Some(labels) = self.labels.as_ref() {
            let _ = f.write_fmt(format_args!("labels: {}", vec_string_to_string(labels)));
        }
        if let Some(values) = self.values.as_ref() {
            let _ = f.write_fmt(format_args!("values: {}", LiveValue::vec_to_string(values)));
        }
        if let Some(popup_menu_position) = self.popup_menu_position.as_ref() {
            let _ = f.write_fmt(format_args!("popup_menu_position: {},", popup_menu_position));
        }
        if let Some(selected_item) = self.selected_item.as_ref() {
            let _ = f.write_fmt(format_args!("selected_item: {},", selected_item));
        }
        write!(f, "")
    }
}

props_to_token!(DropDownProps);
