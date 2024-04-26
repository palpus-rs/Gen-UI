use std::{collections::HashMap, fmt::Display};

use gen_converter::error::Errors;
use gen_parser::{PropsKey, Value};

use crate::{
    prop::{builtin::{draw_color::DrawColor, EventOrder, Layout, MouseCursor, ViewOptimize, Walk}, VISIBLE},
    widget::{prop_ignore, utils::bool_prop},
};

#[derive(Debug, Clone, Default)]
pub struct ViewProps {
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
}

impl ViewProps {
    pub fn props(props: &HashMap<PropsKey, Value>) -> Self {
        let mut view = ViewProps::default();
        for (k, v) in props {
            view.prop(k.name(), v.clone())
        }
        view
    }

    fn prop(&mut self, prop_name: &str, value: Value) -> () {
        match prop_name {
            SHOW_BG => self.show_bg(&value),
            DRAW_BG => self.draw_bg(&value),
            HEIGHT => self.height(&value),
            WIDTH => self.width(&value),
            ALIGN => self.align(&value),
            VISIBLE => self.visible(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("cannot match prop");
                }
                panic!("unslolved prop");
            }
        };
    }
    fn show_bg(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| { self.show_bg = Some(b);})
    }
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        if let Some(s) = value.is_unknown_and_get() {
            match DrawColor::try_from((s, false)) {
                Ok(color) => {
                    self.draw_bg = Some(color);
                    Ok(())
                }
                Err(_) => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to draw_bg",
                    value
                ))),
            }
        } else {
            value
                .is_string_and_get()
                .map(|s| {
                    if let Ok(color) = DrawColor::try_from(s) {
                        self.draw_bg = Some(color);
                    }
                    Ok(())
                })
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{} can not convert to draw_bg",
                        value
                    )))
                })
        }
    }
    fn height(&mut self, value: &Value) -> Result<(), Errors> {
        let mut walk = Walk::default();
        walk.height(value)?;
        self.walk.replace(walk);
        Ok(())
    }
    fn width(&mut self, value: &Value) -> Result<(), Errors> {
        let mut walk = Walk::default();
        walk.width(value)?;
        self.walk.replace(walk);
        Ok(())
    }
    fn align(&mut self, value: &Value)->Result<(),Errors>{
        let mut layout = Layout::default();
        layout.align(value)?;
        self.layout.replace(layout);
        Ok(())
    
    }
    fn visible(&mut self, value: &Value)->Result<(),Errors>{
        bool_prop(value, |b| {self.visible = Some(b);})
    }
}

impl Display for ViewProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_bg) = &self.draw_bg {
            f.write_fmt(format_args!("draw_bg: {}, ", draw_bg));
        }
        if let Some(show_bg) = &self.show_bg {
            f.write_fmt(format_args!("show_bg: {}, ", show_bg));
        }
        if let Some(layout) = &self.layout {
            f.write_fmt(format_args!("layout: {}, ", layout));
        }
        if let Some(walk) = &self.walk {
            f.write_fmt(format_args!("walk: {}, ", walk));
        }
        if let Some(optimize) = &self.optimize {
            f.write_fmt(format_args!("optimize: {}, ", optimize));
        }
        if let Some(event_order) = &self.event_order {
            f.write_fmt(format_args!("event_order: {}, ", event_order));
        }
        if let Some(visible) = &self.visible {
            f.write_fmt(format_args!("visible: {}, ", visible));
        }
        if let Some(grab_key_focus) = &self.grab_key_focus {
            f.write_fmt(format_args!("grab_key_focus: {}, ", grab_key_focus));
        }
        if let Some(block_signal_event) = &self.block_signal_event {
            f.write_fmt(format_args!("block_signal_event: {}, ", block_signal_event));
        }
        if let Some(cursor) = &self.cursor {
            f.write_fmt(format_args!("cursor: {}, ", cursor));
        }
        f.write_str(",")
    }
}
