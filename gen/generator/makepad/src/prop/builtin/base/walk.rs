use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;
use proc_macro2::TokenStream;

use crate::{
    prop::{ABS_POS, HEIGHT, MARGIN, WIDTH},
    ToToken,
};

use super::{DVec2, Margin, Size};

#[derive(Debug, Clone, Default)]
pub struct Walk {
    pub abs_pos: Option<DVec2>,
    pub margin: Option<Margin>,
    pub width: Option<Size>,
    pub height: Option<Size>,
}

impl Walk {
    pub fn height(&mut self, value: &Value) -> Result<(), Errors> {
        let size = Size::try_from(value)?;
        self.height = Some(size);
        Ok(())
    }
    pub fn width(&mut self, value: &Value) -> Result<(), Errors> {
        let size = Size::try_from(value)?;
        self.width = Some(size);
        Ok(())
    }
    pub fn abs_pos(&mut self, value: &Value) -> Result<(), Errors> {
        let abs_pos = DVec2::try_from(value)?;
        self.abs_pos = Some(abs_pos);
        Ok(())
    }
    pub fn margin(&mut self, value: &Value) -> Result<(), Errors> {
        let margin = Margin::try_from(value)?;
        self.margin = Some(margin);
        Ok(())
    }
}

impl ToToken for Walk {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl Display for Walk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut walk = String::new();
        if let Some(abs_pos) = &self.abs_pos {
            walk.push_str(&format!("{}: {},", ABS_POS, abs_pos.to_string()));
        }
        if let Some(margin) = &self.margin {
            walk.push_str(&format!("{}: {},", MARGIN, margin.to_string()));
        }
        if let Some(width) = &self.width {
            walk.push_str(&format!("{}: {},", WIDTH, width.to_string()));
        }
        if let Some(height) = &self.height {
            walk.push_str(&format!("{}: {},", HEIGHT, height.to_string()));
        }
        write!(f, "{}", walk)
    }
}

#[cfg(test)]
mod test_walk {
    use crate::ToToken;

    #[test]
    fn to_tk() {
        let mut walk = super::Walk::default();
        walk.abs_pos = Some("10 10".try_into().unwrap());
        walk.margin = Some("10 10 10 10".try_into().unwrap());
        walk.width = Some("100".try_into().unwrap());
        walk.height = Some("100".try_into().unwrap());
        let tk = walk.to_token_stream();
        let prop ="abs_pos : { x : 10 , y : 10 } , margin : { top : 10 , right : 10 , bottom : 10 , left : 10 } , width : 100 , height : 100 ,";
        assert_eq!(prop, tk.to_string());
    }
    #[test]
    fn to_tk2() {
        let mut walk = super::Walk::default();
        walk.abs_pos = Some("10 10".try_into().unwrap());
        walk.margin = Some("10 10 10 10".try_into().unwrap());
        walk.width = Some("100".try_into().unwrap());
        walk.height = Some("100".try_into().unwrap());
        let tk = walk.to_token_stream();
       dbg!(tk.to_string());
    }
}
