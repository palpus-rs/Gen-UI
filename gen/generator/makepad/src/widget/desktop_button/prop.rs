use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Background, Others, Position, Size},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_desktop_button::DrawDesktopButton, Walk},
        ABS_POS, DRAW_BG, HEIGHT, MARGIN, WIDTH,
    },
    widget::{
        prop_ignore,
        utils::{bind_prop_value, quote_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct DesktopButtonProps {
    pub draw_bg: Option<DrawDesktopButton>,
    pub walk: Option<Walk>,
}

impl DynProps for DesktopButtonProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &Value,
        is_prop: bool,
        ident: &str,
    ) -> TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            Background::BACKGROUND_COLOR => quote_prop(vec![DRAW_BG], &value),
            Others::TYPE => quote_prop(vec![DRAW_BG, "button_type"], &value),
            // ----------------- walk -----------------
            Size::HEIGHT => quote_prop(vec![HEIGHT], &value),
            Size::WIDTH => quote_prop(vec![WIDTH], &value),
            Position::ABS_POS => quote_prop(vec![ABS_POS], &value),
            Size::MARGIN => quote_prop(vec![MARGIN], &value),
            _ => panic!("cannot match prop in BuiltIn label"),
        }
    }
}

impl StaticProps for DesktopButtonProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut label = DesktopButtonProps::default();
        for (k, v) in props {
            label.prop(k.name(), v)
        }
        label
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            Background::BACKGROUND_COLOR => self.draw_bg(&value),
            Others::TYPE => self.button_type(&value),
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

impl ToToken for DesktopButtonProps {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl DesktopButtonProps {
    fn check_draw_bg(&mut self) -> &mut DrawDesktopButton {
        if self.draw_bg.is_none() {
            self.draw_bg = Some(DrawDesktopButton::default());
        }
        self.draw_bg.as_mut().unwrap()
    }
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_bg().color(value)
    }
    fn button_type(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_bg().button_type(value)
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

impl Display for DesktopButtonProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{}", walk.to_string()));
        }
        if let Some(draw_bg) = &self.draw_bg {
            let _ = f.write_fmt(format_args!("{}: {{{}}}", DRAW_BG, draw_bg.to_string()));
        }
        write!(f, "")
    }
}
