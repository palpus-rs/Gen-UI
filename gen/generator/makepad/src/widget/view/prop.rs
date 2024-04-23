use std::fmt::Display;

use crate::prop::builtin::{
    draw_color::DrawColor, EventOrder, Layout, MouseCursor, ViewOptimize, Walk,
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