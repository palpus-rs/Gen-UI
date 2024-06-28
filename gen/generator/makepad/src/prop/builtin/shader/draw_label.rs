use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;
use proc_macro2::TokenStream;

use crate::{widget::utils::f32_prop, ToToken};

use super::draw_text::DrawText;

#[derive(Debug, Clone, Default)]
pub struct DrawLabel {
    pub is_empty: Option<f32>,
    pub draw_super: DrawText,
}

impl DrawLabel {
    pub fn is_empty(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.is_empty.replace(f);
        })
    }
    
}

impl Display for DrawLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut draw_label = String::new();
        draw_label.push_str(self.draw_super.to_string().as_str());

        if let Some(is_empty) = &self.is_empty {
            draw_label.push_str(&format!("is_empty: {}, ", is_empty));
        }

        f.write_str(draw_label.as_str())
    }
}

impl ToToken for DrawLabel {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}