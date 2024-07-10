use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Event, Others, Position, Size},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{Layout, Walk},
        ABS_POS, ALIGN, CLIP_X, CLIP_Y, FLOW, HEIGHT, LINE_SPACING, MARGIN, PADDING, SCROLL,
        SPACING, WIDTH,
    },
    props_to_token,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, bool_prop, quote_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

enum NodeType {
    Outter,
    Body,
}

#[derive(Debug, Clone, Default)]
pub struct FoldHeaderProps {
    pub walk: Option<Walk>,
    pub layout: Option<Layout>,
    pub opened: Option<bool>,
    pub body_walk: Option<Walk>,
}

impl DynProps for FoldHeaderProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
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
            // ------------------- other -----------------
            Event::OPENED => quote_prop(vec!["opened"], &value),
            "body_height" => quote_prop(vec!["body_walk", "height"], &value),
            "body_width" => quote_prop(vec!["body_walk", "width"], &value),
            "body_abs_pos" => quote_prop(vec!["body_walk", "abs_pos"], &value),
            "body_margin" => quote_prop(vec!["body_walk", "margin"], &value),
            _ => panic!("cannot match prop in BuiltIn Icon"),
        }
    }
}

impl StaticProps for FoldHeaderProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = FoldHeaderProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v.clone())
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: gen_parser::Value) -> () {
        let _ = match prop_name {
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
            LINE_SPACING => self.line_spacing(&value),
            // ------------------- other -----------------
            Event::OPENED => self.opened(&value),
            "body_height" => self.height(&value, NodeType::Body),
            "body_width" => self.width(&value, NodeType::Body),
            "body_abs_pos" => self.abs_pos(&value, NodeType::Body),
            "body_margin" => self.margin(&value, NodeType::Body),
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
impl FoldHeaderProps {
    fn check_walk(&mut self) -> &mut Walk {
        if self.walk.is_none() {
            self.walk = Some(Walk::default());
        }
        self.walk.as_mut().unwrap()
    }
    fn check_body_walk(&mut self) -> &mut Walk {
        if self.body_walk.is_none() {
            self.body_walk = Some(Walk::default());
        }
        self.body_walk.as_mut().unwrap()
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
            NodeType::Body => self.check_body_walk().height(value),
        }
    }
    fn width(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Outter => self.check_walk().width(value),
            NodeType::Body => self.check_body_walk().width(value),
        }
    }
    fn abs_pos(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Outter => self.check_walk().abs_pos(value),
            NodeType::Body => self.check_body_walk().abs_pos(value),
        }
    }
    fn margin(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Outter => self.check_walk().margin(value),
            NodeType::Body => self.check_body_walk().margin(value),
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
    fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().line_spacing(value)
    }
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
    fn opened(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.opened = Some(b);
        })
    }
}

impl Display for FoldHeaderProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(body_walk) = self.body_walk.as_ref() {
            let _ = f.write_fmt(format_args!("body_walk: {{{}}}", body_walk));
        }
        if let Some(walk) = self.walk.as_ref() {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(layout) = self.layout.as_ref() {
            let _ = f.write_fmt(format_args!("{}", layout));
        }
        if let Some(opened) = self.opened.as_ref() {
            let _ = f.write_fmt(format_args!("opened: {}", opened));
        }

        write!(f, "")
    }
}

props_to_token!(FoldHeaderProps);
