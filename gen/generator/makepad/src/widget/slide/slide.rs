use std::{collections::HashMap, fmt::Display};

use gen_parser::{PropsKey, Value};
use gen_utils::{
    error::Errors,
    props_manul::{Background, Cursor, Event, Font, Others, Position, Size, Text},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_color::DrawColor, EventOrder, Layout, MouseCursor, ViewOptimize, Walk},
        ABS_POS, ALIGN, BLOCK_SIGNAL_EVENT, CLIP_X, CLIP_Y, COLOR, CURSOR, DRAW_BG, EVENT_ORDER,
        FLOW, GRAB_KEY_FOCUS, HEIGHT, LINE_SPACING, MARGIN, OPTIMIZE, PADDING, SCROLL, SHOW_BG,
        SPACING, VISIBLE, WIDTH,
    },
    widget::{
        label::LabelProps, prop_ignore, utils::{bind_prop_value, bool_prop, quote_prop}, DynProps, StaticProps
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct SlideProps {
    pub draw_bg: Option<DrawColor>,
    pub show_bg: Option<bool>,
    pub layout: Option<Layout>,
    pub walk: Option<Walk>,
    pub optimize: Option<ViewOptimize>,
    pub event_order: Option<EventOrder>,
    pub visible: Option<bool>,
    pub grab_key_focus: Option<bool>,
    pub block_signal_event: Option<bool>,
    pub cursor: Option<MouseCursor>,
    pub title: Option<LabelProps>,
}
impl DynProps for SlideProps {
    fn prop_bind(prop: &PropsKey, value: &Value, is_prop: bool, ident: &str) -> TokenStream {
        let value = bind_prop_value(value, is_prop, ident);

        match prop.name() {
            DRAW_BG => quote_prop(vec![DRAW_BG, COLOR], &value),
            SHOW_BG => quote_prop(vec![SHOW_BG], &value),
            // ----------------- layout -----------------
            SCROLL => quote_prop(vec![SCROLL], &value),
            CLIP_X => quote_prop(vec![CLIP_X], &value),
            CLIP_Y => quote_prop(vec![CLIP_Y], &value),
            PADDING => quote_prop(vec![PADDING], &value),
            ALIGN => quote_prop(vec![ALIGN], &value),
            FLOW => quote_prop(vec![FLOW], &value),
            SPACING => quote_prop(vec![SPACING], &value),
            LINE_SPACING => quote_prop(vec![LINE_SPACING], &value),
            // ----------------- walk -----------------
            HEIGHT => quote_prop(vec![HEIGHT], &value),
            WIDTH => quote_prop(vec![WIDTH], &value),
            ABS_POS => quote_prop(vec![ABS_POS], &value),
            MARGIN => quote_prop(vec![MARGIN], &value),
            // ----------------- other -----------------
            OPTIMIZE => quote_prop(vec![OPTIMIZE], &value),
            EVENT_ORDER => quote_prop(vec![EVENT_ORDER], &value),
            VISIBLE => quote_prop(vec![VISIBLE], &value),
            GRAB_KEY_FOCUS => quote_prop(vec![GRAB_KEY_FOCUS], &value),
            BLOCK_SIGNAL_EVENT => quote_prop(vec![BLOCK_SIGNAL_EVENT], &value),
            CURSOR => quote_prop(vec![CURSOR], &value),

            _ => panic!("cannot match prop"),
        }
    }
}

impl StaticProps for SlideProps {
    fn props(props: &HashMap<PropsKey, Value>) -> Self {
        let mut view = SlideProps::default();
        for (k, v) in props {
            view.prop(k.name(), v)
        }
        view
    }

    fn prop(&mut self, prop_name: &str, value: &Value) -> () {
        let _ = match prop_name {
            Background::BACKGROUND_COLOR => self.draw_bg(&value),
            Background::BACKGROUND_VISIBLE => self.show_bg(&value),
            // ----------------- layout -----------------
            Others::SCROLL => self.scroll(&value),
            Size::CLIP_X => self.clip_x(&value),
            Size::CLIP_Y => self.clip_y(&value),
            Size::PADDING => self.padding(&value),
            Position::ALIGN => self.align(&value),
            Position::FLOW => self.flow(&value),
            Position::SPACING => self.spacing(&value),
            LINE_SPACING => self.line_spacing(&value),
            // ----------------- walk -----------------
            Size::HEIGHT => self.height(&value),
            Size::WIDTH => self.width(&value),
            Position::ABS_POS => self.abs_pos(&value),
            Size::MARGIN => self.margin(&value),
            // ----------------- other -----------------
            Others::OPTIMIZE => self.optimize(&value),
            Event::EVENT_ORDER => self.event_order(&value),
            Others::VISIBLE => self.visible(&value),
            Event::GRAB_KEY_FOCUS => self.grab_key_focus(&value),
            Event::BLOCK_SIGNAL_EVENT => self.block_signal_event(&value),
            Cursor::CURSOR => self.mouse_cursor(&value),
            // ----------------- title -----------------
            Font::FONT_FAMILY => self.title(Font::FONT_FAMILY, &value),
            Font::FONT_SIZE => self.title(Font::FONT_SIZE, &value),
            Font::BRIGHTNESS => self.title(Font::BRIGHTNESS, &value),
            Font::CURVE => self.title(Font::CURVE, &value),
            "text_line_spacing" => self.title(Font::LINE_SPACING, &value),
            Font::TOP_DROP => self.title(Font::TOP_DROP, &value),
            Font::HEIGHT_FACTOR => self.title(Font::HEIGHT_FACTOR, &value),
            Text::TEXT_WRAP => self.title(Text::TEXT_WRAP, &value),
            Text::IGNORE_NEWLINES => self.title( Text::IGNORE_NEWLINES, &value),
            Text::COMBINE_SPACES => self.title(Text::COMBINE_SPACES, &value),
            Font::FONT_SCALE => self.title(Font::FONT_SCALE, &value),
            Text::DRAW_DEPTH => self.title(Text::DRAW_DEPTH, &value),
            Text::COLOR => self.title( Text::COLOR, &value),
            // ----------------- walk -----------------
            "text_height" => self.title(Size::HEIGHT, &value),
            "text_width"=> self.title(Size::WIDTH, &value),
            "text_abs_pos" => self.title(Position::ABS_POS, &value),
            "text_margin" => self.title(Size::MARGIN, &value),
            "text_padding" => self.title(Size::PADDING, &value),
            "text_align" => self.title(Position::ALIGN, &value),
            Text::TEXT => self.title(Text::TEXT, &value),
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

impl ToToken for SlideProps {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl SlideProps {
    fn check_title(&mut self) -> &mut LabelProps {
        if self.title.is_none() {
            self.title = Some(LabelProps::default());
        }
        self.title.as_mut().unwrap()
    }
    fn title(&mut self, prop_name: &str, value: &Value) -> Result<(), Errors> {
        self.check_title().prop(prop_name, value);
        Ok(())
    }
    fn show_bg(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.show_bg = Some(b);
        })
    }
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        let color = DrawColor::try_from(value)?;
        self.draw_bg = Some(color);
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
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
    fn visible(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.visible = Some(b);
        })
    }
    fn grab_key_focus(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.grab_key_focus = Some(b);
        })
    }
    fn block_signal_event(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.block_signal_event = Some(b);
        })
    }
    fn optimize(&mut self, value: &Value) -> Result<(), Errors> {
        self.optimize = Some(value.try_into()?);
        Ok(())
    }
    fn mouse_cursor(&mut self, value: &Value) -> Result<(), Errors> {
        self.cursor = Some(value.try_into()?);
        Ok(())
    }
    fn event_order(&mut self, value: &Value) -> Result<(), Errors> {
        self.event_order = Some(value.try_into()?);
        Ok(())
    }
    fn abs_pos(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().abs_pos(value)
    }
    fn margin(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().margin(value)
    }
    fn check_layout(&mut self) -> &mut Layout {
        if self.layout.is_none() {
            self.layout = Some(Layout::default());
        }
        self.layout.as_mut().unwrap()
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
}

impl Display for SlideProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_bg) = &self.draw_bg {
            let _ = f.write_fmt(format_args!("draw_bg: {{{}}}, ", draw_bg));
        }
        if let Some(show_bg) = &self.show_bg {
            let _ = f.write_fmt(format_args!("show_bg: {}, ", show_bg));
        }
        if let Some(layout) = &self.layout {
            let _ = f.write_fmt(format_args!("{}", layout));
        }
        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(optimize) = &self.optimize {
            let _ = f.write_fmt(format_args!("optimize: {}, ", optimize));
        }
        if let Some(event_order) = &self.event_order {
            let _ = f.write_fmt(format_args!("event_order: {}, ", event_order));
        }
        if let Some(visible) = &self.visible {
            let _ = f.write_fmt(format_args!("visible: {}, ", visible));
        }
        if let Some(grab_key_focus) = &self.grab_key_focus {
            let _ = f.write_fmt(format_args!("grab_key_focus: {}, ", grab_key_focus));
        }
        if let Some(block_signal_event) = &self.block_signal_event {
            let _ = f.write_fmt(format_args!("block_signal_event: {}, ", block_signal_event));
        }
        if let Some(cursor) = &self.cursor {
            let _ = f.write_fmt(format_args!("cursor: {}, ", cursor));
        }
        if let Some(title) = self.title.as_ref() {
            let _ = f.write_fmt(format_args!("title = {{{}}}, ", title));
            
        }
        f.write_str("")
    }
}
