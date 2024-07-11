use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Background, Cursor, Event, Font, Others, Position, Size, State, Text},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_slider::DrawSlider, draw_text::DrawText, Align, Layout, Walk},
        ABS_POS, ALIGN, ASCII_ONLY, BIND, BRIGHTNESS, CLIP_X, CLIP_Y, COLOR, COMBINE_SPACES,
        CURSOR_MARGIN_BOTTOM, CURSOR_MARGIN_TOP, CURSOR_SIZE, CURVE, DEFAULT, DRAW_BG, DRAW_CURSOR,
        DRAW_DEPTH, DRAW_SELECT, DRAW_TEXT, EMPTY_MESSAGE, FLOW, FONT, FONT_SCALE, FONT_SIZE,
        HEIGHT, HEIGHT_FACTOR, INGORE_NEWLINES, IS_EMPTY, LABEL_ALIGN, LABEL_WALK, LINE_SPACING,
        MARGIN, MAX, MIN, NUMERIC_ONLY, ON_FOCUS_SELECT_ALL, PADDING, PATH, READ_ONLY, SCROLL,
        SECRET, SELECT_PAD_EDGES, SPACING, STEP, TEXT, TEXT_STYLE, TOP_DROP, WIDTH, WRAP,
    },
    props_to_token,
    widget::{
        prop_ignore,
        text_input::TextInputProps,
        utils::{bind_prop_value, f64_prop, quote_prop, string_prop, usize_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

const TEXT_INPUT: &str = "text_input";

enum NodeType {
    Outter,
    Label,
}

#[derive(Debug, Clone, Default)]
pub struct SliderProps {
    pub draw_slider: Option<DrawSlider>,
    pub walk: Option<Walk>,
    pub layout: Option<Layout>,
    pub label_walk: Option<Walk>,
    pub label_align: Option<Align>,
    pub draw_text: Option<DrawText>,
    pub text: Option<String>,
    pub text_input: Option<TextInputProps>,
    pub precision: Option<usize>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub default: Option<f64>,
    pub bind: Option<String>,
}

impl DynProps for SliderProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            // ----------------- draw_slider -----------------
            "draw_slider" => quote_prop(vec!["draw_slider"], &value),
            "slider_pos" => quote_prop(vec!["slider_pos"], &value),
            "slider_type" => quote_prop(vec!["slider_type"], &value),
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
            // ------------------- label walk -----------------
            "text_height" => quote_prop(vec![LABEL_WALK, HEIGHT], &value),
            "text_width" => quote_prop(vec![LABEL_WALK, WIDTH], &value),
            "text_abs_pos" => quote_prop(vec![LABEL_WALK, ABS_POS], &value),
            "text_margin" => quote_prop(vec![LABEL_WALK, MARGIN], &value),
            Text::TEXT_ALIGN => quote_prop(vec![LABEL_ALIGN], &value),
            // ------------------- draw_text -----------------
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
            // ------------------- text -----------------
            Text::TEXT => quote_prop(vec![TEXT], &value),
            // ------------------- text_input -----------------
            "input_background_color" => quote_prop(vec![TEXT_INPUT, DRAW_BG, COLOR], &value),
            DRAW_SELECT => quote_prop(vec![TEXT_INPUT, DRAW_SELECT], &value),
            DRAW_CURSOR => quote_prop(vec![TEXT_INPUT, DRAW_CURSOR], &value),
            "input_font_family" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, FONT], &value),
            "input_font_size" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, FONT_SIZE], &value),
            "input_brightness" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, BRIGHTNESS], &value),
            "input_curve" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, CURVE], &value),
            "input_text_line_spacing" => {
                quote_prop(vec![TEXT_INPUT, DRAW_TEXT, LINE_SPACING], &value)
            }
            "input_top_drop" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, TOP_DROP], &value),
            "input_height_factor" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, HEIGHT_FACTOR], &value),
            "input_text_wrap" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, WRAP], &value),
            "input_font_color" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, COLOR], &value),
            "input_ignore_newlines" => {
                quote_prop(vec![TEXT_INPUT, DRAW_TEXT, INGORE_NEWLINES], &value)
            }
            "input_combine_spaces" => {
                quote_prop(vec![TEXT_INPUT, DRAW_TEXT, COMBINE_SPACES], &value)
            }
            "input_font_scale" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, FONT_SCALE], &value),
            "input_draw_depth" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, DRAW_DEPTH], &value),
            "input_text_empty" => quote_prop(vec![TEXT_INPUT, DRAW_TEXT, IS_EMPTY], &value),
            "input_height" => quote_prop(vec![TEXT_INPUT, HEIGHT], &value),
            "input_width" => quote_prop(vec![TEXT_INPUT, WIDTH], &value),
            "input_abs_pos" => quote_prop(vec![TEXT_INPUT, ABS_POS], &value),
            "input_margin" => quote_prop(vec![TEXT_INPUT, MARGIN], &value),
            "input_scroll" => quote_prop(vec![TEXT_INPUT, SCROLL], &value),
            "input_clip_x" => quote_prop(vec![TEXT_INPUT, CLIP_X], &value),
            "input_clip_y" => quote_prop(vec![TEXT_INPUT, CLIP_Y], &value),
            "input_padding" => quote_prop(vec![TEXT_INPUT, PADDING], &value),
            "input_align" => quote_prop(vec![TEXT_INPUT, ALIGN], &value),
            "input_flow" => quote_prop(vec![TEXT_INPUT, FLOW], &value),
            "input_spacing" => quote_prop(vec![TEXT_INPUT, SPACING], &value),
            "input_line_spacing" => quote_prop(vec![TEXT_INPUT, LINE_SPACING], &value),
            "input_text_align" => quote_prop(vec![TEXT_INPUT, LABEL_ALIGN], &value),
            Cursor::CURSOR_SIZE => quote_prop(vec![TEXT_INPUT, CURSOR_SIZE], &value),
            Cursor::CURSOR_MARGIN_BOTTOM => {
                quote_prop(vec![TEXT_INPUT, CURSOR_MARGIN_BOTTOM], &value)
            }
            Cursor::CURSOR_MARGIN_TOP => quote_prop(vec![TEXT_INPUT, CURSOR_MARGIN_TOP], &value),
            Others::SELECT_PAD_EDGES => quote_prop(vec![TEXT_INPUT, SELECT_PAD_EDGES], &value),
            Text::EMPTY_MESSAGE => quote_prop(vec![TEXT_INPUT, EMPTY_MESSAGE], &value),
            State::NUMERIC_ONLY => quote_prop(vec![TEXT_INPUT, NUMERIC_ONLY], &value),
            State::SECRET => quote_prop(vec![TEXT_INPUT, SECRET], &value),
            Others::ON_FOCUS_SELECT_ALL => {
                quote_prop(vec![TEXT_INPUT, ON_FOCUS_SELECT_ALL], &value)
            }
            State::READ_ONLY => quote_prop(vec![TEXT_INPUT, READ_ONLY], &value),
            State::ASCII_ONLY => quote_prop(vec![TEXT_INPUT, ASCII_ONLY], &value),
            // ------------------- other ----------------
            Others::PRECISION => quote_prop(vec![BIND], &value),
            Others::MIN => quote_prop(vec![MIN], &value),
            Others::MAX => quote_prop(vec![MAX], &value),
            Others::STEP => quote_prop(vec![STEP], &value),
            Others::VALUE => quote_prop(vec![DEFAULT], &value),
            Event::BIND => quote_prop(vec![BIND], &value),
            _ => panic!("cannot match prop in BuiltIn Icon"),
        }
    }
}

impl StaticProps for SliderProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = SliderProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v)
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            // ----------------- draw_slider -----------------
            "draw_slider" => self.draw_slider(&value),
            "slider_pos" => self.slider_pos(&value),
            "slider_type" => self.slider_type(&value),
            // ----------------- walk -----------------
            Size::HEIGHT => self.height(&value, NodeType::Outter),
            Size::WIDTH => self.width(&value, NodeType::Outter),
            Position::ABS_POS => self.abs_pos(&value, NodeType::Outter),
            Size::MARGIN => self.margin(&value, NodeType::Outter),
            // ------------------- layout -----------------
            Others::SCROLL => self.scroll(&value),
            Size::CLIP_X => self.clip_x(&value),
            Size::CLIP_Y => self.clip_y(&value),
            Size::PADDING => self.padding(&value),
            Position::ALIGN => self.align(&value),
            Position::FLOW => self.flow(&value),
            Position::SPACING => self.spacing(&value),
            LINE_SPACING => self.line_spacing(&value, NodeType::Outter),
            // ------------------- label walk -----------------
            "text_height" => self.height(&value, NodeType::Label),
            "text_width" => self.width(&value, NodeType::Label),
            "text_abs_pos" => self.abs_pos(&value, NodeType::Label),
            "text_margin" => self.margin(&value, NodeType::Label),
            Text::TEXT_ALIGN => self.label_align(&value),
            // ------------------- draw_text -----------------
            Font::FONT_FAMILY => self.font(&value),
            Font::FONT_SIZE => self.font_size(&value),
            Font::BRIGHTNESS => self.brightness(&value),
            Font::CURVE => self.curve(&value),
            "text_line_spacing" => self.line_spacing(&value, NodeType::Label),
            Font::TOP_DROP => self.top_drop(&value),
            Font::HEIGHT_FACTOR => self.height_factor(&value),
            Text::TEXT_WRAP => self.wrap(&value),
            Text::IGNORE_NEWLINES => self.ignore_newlines(&value),
            Text::COMBINE_SPACES => self.combine_spaces(&value),
            Font::FONT_SCALE => self.font_scale(&value),
            Text::DRAW_DEPTH => self.draw_depth(&value),
            Text::COLOR => self.color(&value),
            // ------------------- text -----------------
            Text::TEXT => self.text(&value),
            // ------------------- text_input -----------------
            "input_background_color" => self.text_input(Background::BACKGROUND_COLOR, &value),
            DRAW_SELECT => self.text_input(DRAW_SELECT, &value),
            DRAW_CURSOR => self.text_input(DRAW_CURSOR, &value),
            "input_font_family" => self.text_input(Font::FONT_FAMILY, &value),
            "input_font_size" => self.text_input(Font::FONT_SIZE, &value),
            "input_brightness" => self.text_input(Font::BRIGHTNESS, &value),
            "input_curve" => self.text_input(Font::CURVE, &value),
            "input_text_line_spacing" => self.text_input("text_line_spacing", &value),
            "input_top_drop" => self.text_input(Font::TOP_DROP, &value),
            "input_height_factor" => self.text_input(Font::HEIGHT_FACTOR, &value),
            "input_text_wrap" => self.text_input(Text::TEXT_WRAP, &value),
            "input_font_color" => self.text_input(Text::COLOR, &value),
            "input_ignore_newlines" => self.text_input(Text::IGNORE_NEWLINES, &value),
            "input_combine_spaces" => self.text_input(Text::COMBINE_SPACES, &value),
            "input_font_scale" => self.text_input(Font::FONT_SCALE, &value),
            "input_draw_depth" => self.text_input(Text::DRAW_DEPTH, &value),
            "input_text_empty" => self.text_input(Text::EMPTY, &value),
            "input_height" => self.text_input(Size::HEIGHT, &value),
            "input_width" => self.text_input(Size::WIDTH, &value),
            "input_abs_pos" => self.text_input(Position::ABS_POS, &value),
            "input_margin" => self.text_input(Size::MARGIN, &value),
            "input_scroll" => self.text_input(Others::SCROLL, &value),
            "input_clip_x" => self.text_input(Size::CLIP_X, &value),
            "input_clip_y" => self.text_input(Size::CLIP_Y, &value),
            "input_padding" => self.text_input(Size::PADDING, &value),
            "input_align" => self.text_input(Position::ALIGN, &value),
            "input_flow" => self.text_input(Position::FLOW, &value),
            "input_spacing" => self.text_input(Position::SPACING, &value),
            "input_line_spacing" => self.text_input(LINE_SPACING, &value),
            "input_text_align" => self.text_input(Text::TEXT_ALIGN, &value),
            Cursor::CURSOR_SIZE => self.text_input(Cursor::CURSOR_SIZE, &value),
            Cursor::CURSOR_MARGIN_BOTTOM => self.text_input(Cursor::CURSOR_MARGIN_BOTTOM, &value),
            Cursor::CURSOR_MARGIN_TOP => self.text_input(Cursor::CURSOR_MARGIN_TOP, &value),
            Others::SELECT_PAD_EDGES => self.text_input(Others::SELECT_PAD_EDGES, &value),
            Text::EMPTY_MESSAGE => self.text_input(Text::EMPTY_MESSAGE, &value),
            State::NUMERIC_ONLY => self.text_input(State::NUMERIC_ONLY, &value),
            State::SECRET => self.text_input(State::SECRET, &value),
            Others::ON_FOCUS_SELECT_ALL => self.text_input(Others::ON_FOCUS_SELECT_ALL, &value),
            State::READ_ONLY => self.text_input(State::READ_ONLY, &value),
            State::ASCII_ONLY => self.text_input(State::ASCII_ONLY, &value),
            // ------------------- other ----------------
            Others::PRECISION => self.precision(&value),
            Others::MIN => self.min(&value),
            Others::MAX => self.max(&value),
            Others::STEP => self.step(&value),
            Others::VALUE => self.default_value(&value),
            Event::BIND => self.bind(&value),
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

#[allow(dead_code)]
impl SliderProps {
    fn check_walk(&mut self) -> &mut Walk {
        if self.walk.is_none() {
            self.walk = Some(Walk::default());
        }
        self.walk.as_mut().unwrap()
    }
    fn check_label_walk(&mut self) -> &mut Walk {
        if self.label_walk.is_none() {
            self.label_walk = Some(Walk::default());
        }
        self.label_walk.as_mut().unwrap()
    }
    fn check_text_input(&mut self) -> &mut TextInputProps {
        if self.text_input.is_none() {
            self.text_input = Some(TextInputProps::default());
        }
        self.text_input.as_mut().unwrap()
    }
    fn check_draw_text(&mut self) -> &mut DrawText {
        if self.draw_text.is_none() {
            self.draw_text = Some(DrawText::default());
        }
        self.draw_text.as_mut().unwrap()
    }
    fn check_draw_slider(&mut self) -> &mut DrawSlider {
        if self.draw_slider.is_none() {
            self.draw_slider = Some(DrawSlider::default());
        }
        self.draw_slider.as_mut().unwrap()
    }
    fn check_layout(&mut self) -> &mut Layout {
        if self.layout.is_none() {
            self.layout = Some(Layout::default());
        }
        self.layout.as_mut().unwrap()
    }
    fn height(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Outter => self.check_walk().height(value),
            NodeType::Label => self.check_label_walk().height(value),
        }
    }
    fn width(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Outter => self.check_walk().width(value),
            NodeType::Label => self.check_label_walk().width(value),
        }
    }
    fn abs_pos(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Outter => self.check_walk().abs_pos(value),
            NodeType::Label => self.check_label_walk().abs_pos(value),
        }
    }
    fn margin(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Outter => self.check_walk().margin(value),
            NodeType::Label => self.check_label_walk().margin(value),
        }
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

    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
    fn label_align(&mut self, value: &Value) -> Result<(), Errors> {
        self.label_align = Some(value.try_into()?);
        Ok(())
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
    fn line_spacing(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Outter => self.check_layout().line_spacing(value),
            NodeType::Label => self.check_draw_text().line_spacing(value),
        }
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
    fn text_input(&mut self, prop_name: &str, value: &Value) -> Result<(), Errors> {
        self.check_text_input().prop(prop_name, value);
        Ok(())
    }
    fn precision(&mut self, value: &Value) -> Result<(), Errors> {
        usize_prop(value, |f| self.precision = Some(f))
    }
    fn min(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| self.min = Some(f))
    }
    fn max(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| self.max = Some(f))
    }
    fn step(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| self.step = Some(f))
    }
    fn default_value(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| self.default = Some(f))
    }
    fn bind(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {
            let _ = self.bind.replace(s.to_string());
        })
    }
    fn draw_slider(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_slider().draw_super.pixel(value)
    }
    fn slider_pos(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_slider().slider_pos(value)
    }
    fn slider_type(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_slider().slider_type(value)
    }
}

impl Display for SliderProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_slider) = self.draw_slider.as_ref() {
            let _ = f.write_fmt(format_args!("draw_slider: {{{}}}, ", draw_slider));
        }
        if let Some(walk) = self.walk.as_ref() {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(layout) = self.layout.as_ref() {
            let _ = f.write_fmt(format_args!("{}", layout));
        }
        if let Some(label_walk) = self.label_walk.as_ref() {
            let _ = f.write_fmt(format_args!("label_walk: {{{}}}, ", label_walk));
        }
        if let Some(label_align) = self.label_align.as_ref() {
            let _ = f.write_fmt(format_args!("label_align: {}, ", label_align));
        }
        if let Some(draw_text) = self.draw_text.as_ref() {
            let _ = f.write_fmt(format_args!("draw_text: {{{}}}, ", draw_text));
        }
        if let Some(text) = self.text.as_ref() {
            let _ = f.write_fmt(format_args!("text: \"{}\", ", text));
        }
        if let Some(text_input) = self.text_input.as_ref() {
            let _ = f.write_fmt(format_args!("text_input: {{{}}}, ", text_input));
        }
        if let Some(precision) = self.precision.as_ref() {
            let _ = f.write_fmt(format_args!("precision: {}, ", precision));
        }
        if let Some(min) = self.min.as_ref() {
            let _ = f.write_fmt(format_args!("min: {}, ", min));
        }
        if let Some(max) = self.max.as_ref() {
            let _ = f.write_fmt(format_args!("max: {}, ", max));
        }
        if let Some(step) = self.step.as_ref() {
            let _ = f.write_fmt(format_args!("step: {}, ", step));
        }
        if let Some(default) = self.default.as_ref() {
            let _ = f.write_fmt(format_args!("default: {}, ", default));
        }
        if let Some(bind) = self.bind.as_ref() {
            let _ = f.write_fmt(format_args!("bind: \"{}\", ", bind));
        }
        write!(f, "")
    }
}

props_to_token!(SliderProps);
