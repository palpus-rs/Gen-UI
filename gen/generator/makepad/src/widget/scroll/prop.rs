use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Position, Size},
};
use proc_macro2::TokenStream;

use crate::{
    prop::builtin::{draw_scroll_bar::DrawScrollBar, Axis},
    props_to_token,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, bool_prop, f64_prop, quote_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct ScrollBarProps {
    pub draw_bar: Option<DrawScrollBar>,
    pub bar_size: Option<f64>,
    pub min_handle_size: Option<f64>,
    pub bar_side_margin: Option<f64>,
    pub axis: Option<Axis>,
    pub use_vertical_finger_scroll: Option<bool>,
    pub smoothing: Option<f64>,
}

impl DynProps for ScrollBarProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            Position::FLOW => quote_prop(vec!["axis"], &value),
            "draw_bar" => quote_prop(vec!["draw_bar"], &value),
            "is_vertical" => quote_prop(vec!["draw_bar", "is_vertical"], &value),
            "norm_handle" => quote_prop(vec!["draw_bar", "norm_handle"], &value),
            "norm_scroll" => quote_prop(vec!["draw_bar", "norm_scroll"], &value),
            Size::BAR_SIZE => quote_prop(vec!["bar_size"], &value),
            Size::MARGIN => quote_prop(vec!["bar_side_margin"], &value),
            "min_handle_size" => quote_prop(vec!["min_handle_size"], &value),
            "use_vertical_finger_scroll" => quote_prop(vec!["use_vertical_finger_scroll"], &value),
            "smoothing" => quote_prop(vec!["smoothing"], &value),

            _ => panic!("cannot match prop in BuiltIn ScrollBar"),
        }
    }
}

impl StaticProps for ScrollBarProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = ScrollBarProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v)
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            Position::FLOW => self.axis(&value),
            "draw_bar" => self.draw_bar(&value),
            "is_vertical" => self.check_draw_bar().is_vertical(&value),
            "norm_handle" => self.check_draw_bar().norm_handle(&value),
            "norm_scroll" => self.check_draw_bar().norm_scroll(&value),
            Size::BAR_SIZE => self.bar_size(&value),
            Size::MARGIN => self.bar_side_margin(&value),
            "min_handle_size" => self.min_handle_size(&value),
            "use_vertical_finger_scroll" => self.use_vertical_finger_scroll(&value),
            "smoothing" => self.smoothing(&value),
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
impl ScrollBarProps {
    fn axis(&mut self, value: &Value) -> Result<(), Errors> {
        self.axis = Some(value.try_into()?);
        Ok(())
    }
    fn check_draw_bar(&mut self) -> &mut DrawScrollBar {
        if let None = self.draw_bar.as_ref() {
            self.draw_bar = Some(DrawScrollBar::default());
        }
        self.draw_bar.as_mut().unwrap()
    }
    fn draw_bar(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_bar().draw_super = value.try_into()?;
        Ok(())
    }

    fn bar_size(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.bar_size.replace(f);
        })
    }
    fn bar_side_margin(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.bar_side_margin.replace(f);
        })
    }
    fn min_handle_size(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.min_handle_size.replace(f);
        })
    }
    fn use_vertical_finger_scroll(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.use_vertical_finger_scroll.replace(b);
        })
    }
    fn smoothing(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.smoothing.replace(f);
        })
    }
}

impl Display for ScrollBarProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(axis) = self.axis.as_ref() {
            let _ = f.write_fmt(format_args!("axis: {},", axis));
        }
        if let Some(draw_bar) = self.draw_bar.as_ref() {
            let _ = f.write_fmt(format_args!("{}", draw_bar));
        }
        if let Some(bar_size) = self.bar_size.as_ref() {
            let _ = f.write_fmt(format_args!("bar_size: {},", bar_size));
        }
        if let Some(bar_side_margin) = self.bar_side_margin.as_ref() {
            let _ = f.write_fmt(format_args!("bar_side_margin: {},", bar_side_margin));
        }
        if let Some(min_handle_size) = self.min_handle_size.as_ref() {
            let _ = f.write_fmt(format_args!("min_handle_size: {},", min_handle_size));
        }
        if let Some(use_vertical_finger_scroll) = self.use_vertical_finger_scroll.as_ref() {
            let _ = f.write_fmt(format_args!(
                "use_vertical_finger_scroll: {},",
                use_vertical_finger_scroll
            ));
        }
        if let Some(smoothing) = self.smoothing.as_ref() {
            let _ = f.write_fmt(format_args!("smoothing: {},", smoothing));
        }

        write!(f, "")
    }
}

props_to_token!(ScrollBarProps);
