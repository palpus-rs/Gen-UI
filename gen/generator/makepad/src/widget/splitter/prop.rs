use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Position, Size},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_splitter::DrawSplitter, SplitterAlign, SplitterAxis, Walk},
        ABS_POS, HEIGHT, MARGIN, WIDTH,
    },
    props_to_token,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, f64_prop, quote_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

/// if true, the splitter is horizontal, otherwise it is vertical
static mut SPLITTER_V_H: bool = true;

#[derive(Debug, Clone, Default)]
pub struct SplitterProps {
    pub axis: Option<SplitterAxis>,
    pub align: Option<SplitterAlign>,
    pub min_vertical: Option<f64>,
    pub max_vertical: Option<f64>,
    pub min_horizontal: Option<f64>,
    pub max_horizontal: Option<f64>,
    pub draw_splitter: Option<DrawSplitter>,
    pub split_bar_size: Option<f64>,
    // #[find] pub a: WidgetRef,
    // #[find] pub b: WidgetRef,
    pub walk: Option<Walk>,
}

impl DynProps for SplitterProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        fn quote_min_proportion(value: &str) -> TokenStream {
            if unsafe { SPLITTER_V_H } {
                quote_prop(vec!["min_horizontal"], value)
            } else {
                quote_prop(vec!["min_vertical"], value)
            }
        }

        fn quote_max_proportion(value: &str) -> TokenStream {
            if unsafe { SPLITTER_V_H } {
                quote_prop(vec!["max_horizontal"], value)
            } else {
                quote_prop(vec!["max_vertical"], value)
            }
        }

        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            Position::FLOW => {
                if value == "Vertical" {
                    unsafe {
                        SPLITTER_V_H = false;
                    }
                }

                if value == "Horizontal" {
                    unsafe {
                        SPLITTER_V_H = true;
                    }
                }

                quote_prop(vec!["axis"], &value)
            }
            Position::ALIGN => quote_prop(vec!["align"], &value),
            Size::MIN_PROPORTION => quote_min_proportion(&value),
            Size::MAX_PROPORTION => quote_max_proportion(&value),
            "draw_splitter" => quote_prop(vec!["draw_splitter"], &value),
            "splitter_width" => quote_prop(vec!["split_bar_size"], &value),
            // ----------------- walk -----------------
            Size::HEIGHT => quote_prop(vec![HEIGHT], &value),
            Size::WIDTH => quote_prop(vec![WIDTH], &value),
            Position::ABS_POS => quote_prop(vec![ABS_POS], &value),
            Size::MARGIN => quote_prop(vec![MARGIN], &value),

            _ => panic!("cannot match prop in BuiltIn Splitter"),
        }
    }
}

impl StaticProps for SplitterProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = SplitterProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v)
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            Position::FLOW => self.axis(&value),
            Position::ALIGN => self.align(&value),
            Size::MIN_PROPORTION => self.min_v_h(&value),
            Size::MAX_PROPORTION => self.max_v_h(&value),
            "draw_splitter" => self.draw_splitter(&value),
            "splitter_width" => self.split_bar_size(&value),
            // ----------------- walk -----------------
            Size::HEIGHT => self.height(&value),
            Size::WIDTH => self.width(&value),
            Position::ABS_POS => self.abs_pos(&value),
            Size::MARGIN => self.margin(&value),
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
impl SplitterProps {
    fn axis(&mut self, value: &Value) -> Result<(), Errors> {
        self.axis = Some(value.try_into()?);
        Ok(())
    }
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.align = Some(value.try_into()?);
        Ok(())
    }
    fn min_v_h(&mut self, value: &Value) -> Result<(), Errors> {
        match self.axis.as_ref() {
            Some(SplitterAxis::Vertical) => f64_prop(value, |v| {
                self.min_vertical = Some(v);
            }),
            _ => f64_prop(value, |v| {
                self.min_horizontal = Some(v);
            }),
        }
    }
    fn max_v_h(&mut self, value: &Value) -> Result<(), Errors> {
        match self.axis.as_ref() {
            Some(SplitterAxis::Vertical) => f64_prop(value, |v| {
                self.max_vertical = Some(v);
            }),
            _ => f64_prop(value, |v| {
                self.max_horizontal = Some(v);
            }),
        }
    }
    fn draw_splitter(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_splitter = Some(value.try_into()?);
        Ok(())
    }
    fn split_bar_size(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |v| {
            self.split_bar_size = Some(v);
        })
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

impl Display for SplitterProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(walk) = self.walk.as_ref() {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(axis) = self.axis.as_ref() {
            let _ = f.write_fmt(format_args!("axis: {},", axis));
        }
        if let Some(align) = self.align.as_ref() {
            let _ = f.write_fmt(format_args!("align: {},", align));
        }
        if let Some(min_vertical) = self.min_vertical.as_ref() {
            let _ = f.write_fmt(format_args!("min_vertical: {},", min_vertical));
        }
        if let Some(max_vertical) = self.max_vertical.as_ref() {
            let _ = f.write_fmt(format_args!("max_vertical: {},", max_vertical));
        }
        if let Some(min_horizontal) = self.min_horizontal.as_ref() {
            let _ = f.write_fmt(format_args!("min_horizontal: {},", min_horizontal));
        }
        if let Some(max_horizontal) = self.max_horizontal.as_ref() {
            let _ = f.write_fmt(format_args!("max_horizontal: {},", max_horizontal));
        }
        if let Some(draw_splitter) = self.draw_splitter.as_ref() {
            let _ = f.write_fmt(format_args!("draw_splitter: {},", draw_splitter));
        }
        if let Some(split_bar_size) = self.split_bar_size.as_ref() {
            let _ = f.write_fmt(format_args!("split_bar_size: {},", split_bar_size));
        }

        write!(f, "")
    }
}

props_to_token!(SplitterProps);
