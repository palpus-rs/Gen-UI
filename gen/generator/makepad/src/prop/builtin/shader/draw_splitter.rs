use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;
use proc_macro2::TokenStream;

use crate::{widget::utils::f32_prop, ToToken};

use super::draw_quad::DrawQuad;

#[derive(Debug, Clone, Default)]
pub struct DrawSplitter {
    pub is_vertical: Option<f32>,
    pub draw_super: DrawQuad,
}

impl DrawSplitter {
    pub fn is_vertical(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.is_vertical.replace(f);
        })
    }
}

impl TryFrom<&Value> for DrawSplitter {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let (quad, _) = DrawQuad::try_from_back(value)?;
        Ok(DrawSplitter {
            is_vertical: None,
            draw_super: quad,
        })
    }
}


impl Display for DrawSplitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.draw_super.fmt(f)
    }
}

impl ToToken for DrawSplitter {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}