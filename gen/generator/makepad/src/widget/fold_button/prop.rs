use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Background, Position, Size},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_quad::DrawQuad, DVec2, Walk},
        ABS_POS, DRAW_BG, HEIGHT, MARGIN, WIDTH,
    },
    props_to_token,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, quote_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct FoldButtonProps {
    pub walk: Option<Walk>,
    pub draw_bg: Option<DrawQuad>,
    pub abs_size: Option<DVec2>,
    pub abs_offset: Option<DVec2>,
}

impl DynProps for FoldButtonProps {
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
            Background::BACKGROUND_COLOR => quote_prop(vec![DRAW_BG], &value),

            _ => panic!("cannot match prop in BuiltIn FoldButton"),
        }
    }
}

impl StaticProps for FoldButtonProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = FoldButtonProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v.clone())
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: gen_parser::Value) -> () {
        let _ = match prop_name {
            // ----------------- walk -----------------
            Size::HEIGHT => self.height(&value),
            Size::WIDTH => self.width(&value),
            Position::ABS_POS => self.abs_pos(&value),
            Size::MARGIN => self.margin(&value),
            Background::BACKGROUND_COLOR => self.draw_bg(&value),
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
impl FoldButtonProps {
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
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_bg = Some(value.try_into()?);
        Ok(())
    }
}

impl Display for FoldButtonProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo!(DrawQuard pixel())
        if let Some(draw_bg) = self.draw_bg.as_ref() {
            let _ = f.write_fmt(format_args!("{}: {{{}}}", DRAW_BG, draw_bg));
        }
        if let Some(walk) = self.walk.as_ref() {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        write!(f, "")
    }
}

props_to_token!(FoldButtonProps);
