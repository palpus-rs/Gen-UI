use std::{collections::HashMap, fmt::Display};

use gen_parser::{PropsKey, Value};
use gen_utils::{
    error::Errors,
    props_manul::{Background, Cursor, Event, Font, Others, Position, Size, Text},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{
            draw_color::DrawColor, EventOrder, Layout, MouseCursor, Vec2, ViewOptimize, Walk,
        },
        ABS_POS, ALIGN, BLOCK_SIGNAL_EVENT, CLIP_X, CLIP_Y, COLOR, CURSOR, DRAW_BG, EVENT_ORDER,
        FLOW, GRAB_KEY_FOCUS, HEIGHT, LINE_SPACING, MARGIN, OPTIMIZE, PADDING, SCROLL, SHOW_BG,
        SPACING, VISIBLE, WIDTH,
    },
    widget::{
        prop_ignore,
        utils::{bind_prop_value, bool_prop, quote_prop, string_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct CXWindow {
    pub position: Option<Vec2>,
    pub inner_size: Option<Vec2>,
}

impl CXWindow {
    pub fn position(&mut self, value: &Value) -> Result<(), Errors> {
        self.position = Some(value.try_into()?);
        Ok(())
    }
    pub fn inner_size(&mut self, value: &Value) -> Result<(), Errors> {
        self.inner_size = Some(value.try_into()?);
        Ok(())
    }
}

impl Display for CXWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(position) = &self.position {
            let _ = f.write_fmt(format_args!("position: {}, ", position));
        }
        if let Some(inner_size) = &self.inner_size {
            let _ = f.write_fmt(format_args!("inner_size: {}, ", inner_size));
        }
        f.write_str("")
    }
}

#[derive(Debug, Clone, Default)]
pub struct WindowProps {
    pub window: Option<CXWindow>,
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
    pub title: Option<String>,
}
impl DynProps for WindowProps {
    fn prop_bind(prop: &PropsKey, value: &Value, is_prop: bool, ident: &str) -> TokenStream {
        let value = bind_prop_value(value, is_prop, ident);

        match prop.name() {
            Background::BACKGROUND_COLOR => quote_prop(vec![DRAW_BG, COLOR], &value),
            Background::BACKGROUND_VISIBLE => quote_prop(vec![SHOW_BG], &value),
            // ----------------- layout -----------------
            Others::SCROLL => quote_prop(vec![SCROLL], &value),
            Size::CLIP_X => quote_prop(vec![CLIP_X], &value),
            Size::CLIP_Y => quote_prop(vec![CLIP_Y], &value),
            Size::PADDING => quote_prop(vec![PADDING], &value),
            Position::ALIGN => quote_prop(vec![ALIGN], &value),
            Position::FLOW => quote_prop(vec![FLOW], &value),
            Position::SPACING => quote_prop(vec![SPACING], &value),
            Font::LINE_SPACING => quote_prop(vec![LINE_SPACING], &value),
            // ----------------- walk -----------------
            Size::HEIGHT => quote_prop(vec![HEIGHT], &value),
            Size::WIDTH => quote_prop(vec![WIDTH], &value),
            Position::ABS_POS => quote_prop(vec![ABS_POS], &value),
            Size::MARGIN => quote_prop(vec![MARGIN], &value),
            // ----------------- other -----------------
            Others::OPTIMIZE => quote_prop(vec![OPTIMIZE], &value),
            Event::EVENT_ORDER => quote_prop(vec![EVENT_ORDER], &value),
            Others::VISIBLE => quote_prop(vec![VISIBLE], &value),
            Event::GRAB_KEY_FOCUS => quote_prop(vec![GRAB_KEY_FOCUS], &value),
            Event::BLOCK_SIGNAL_EVENT => quote_prop(vec![BLOCK_SIGNAL_EVENT], &value),
            Cursor::CURSOR => quote_prop(vec![CURSOR], &value),
            // ----------------- window -----------------
            Position::WINDOW_POSITION => quote_prop(vec!["window", "position"], &value),
            Size::WINDOW_SIZE => quote_prop(vec!["window", "inner_size"], &value),
            _ => panic!("cannot match prop"),
        }
    }
}

impl StaticProps for WindowProps {
    fn props(props: &HashMap<PropsKey, Value>) -> Self {
        let mut view = WindowProps::default();
        for (k, v) in props {
            view.prop(k.name(), v.clone())
        }
        view
    }

    fn prop(&mut self, prop_name: &str, value: Value) -> () {
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
            Font::LINE_SPACING => self.line_spacing(&value),
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
            // ----------------- window -----------------
            Position::WINDOW_POSITION => self.position(&value),
            Size::WINDOW_SIZE => self.inner_size(&value),
            Text::WINDOW_TITLE => self.title(&value),
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

impl ToToken for WindowProps {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl WindowProps {
    fn title(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |title| {
            self.title = Some(title.to_string());
        })
    }
    fn check_window(&mut self) -> &mut CXWindow {
        if self.window.is_none() {
            self.window = Some(CXWindow::default());
        }
        self.window.as_mut().unwrap()
    }
    fn position(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_window().position(value)
    }
    fn inner_size(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_window().inner_size(value)
    }
    fn show_bg(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.show_bg = Some(b);
        })
    }
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_bg = Some(value.try_into()?);
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

impl Display for WindowProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(window) = self.window.as_ref() {
            let _ = f.write_fmt(format_args!("window: {{{}}}", window));
        }
        if let Some(draw_bg) = self.draw_bg.as_ref() {
            let _ = f.write_fmt(format_args!("draw_bg: {{{}}}, ", draw_bg));
        }
        if let Some(show_bg) = self.show_bg.as_ref() {
            let _ = f.write_fmt(format_args!("show_bg: {}, ", show_bg));
        }
        if let Some(layout) = self.layout.as_ref() {
            let _ = f.write_fmt(format_args!("{}", layout));
        }
        if let Some(walk) = self.walk.as_ref() {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(optimize) = self.optimize.as_ref() {
            let _ = f.write_fmt(format_args!("optimize: {}, ", optimize));
        }
        if let Some(event_order) = self.event_order.as_ref() {
            let _ = f.write_fmt(format_args!("event_order: {}, ", event_order));
        }
        if let Some(visible) = self.visible.as_ref() {
            let _ = f.write_fmt(format_args!("visible: {}, ", visible));
        }
        if let Some(grab_key_focus) = self.grab_key_focus.as_ref() {
            let _ = f.write_fmt(format_args!("grab_key_focus: {}, ", grab_key_focus));
        }
        if let Some(block_signal_event) = self.block_signal_event.as_ref() {
            let _ = f.write_fmt(format_args!("block_signal_event: {}, ", block_signal_event));
        }
        if let Some(cursor) = self.cursor.as_ref() {
            let _ = f.write_fmt(format_args!("cursor: {}, ", cursor));
        }
        if let Some(title) = self.title.as_ref() {
            let _ = f.write_fmt(format_args!(
                "caption_bar = {{caption_label = {{label = {{text: \"{}\"}}}}}}, ",
                title
            ));
        }
        f.write_str("")
    }
}
