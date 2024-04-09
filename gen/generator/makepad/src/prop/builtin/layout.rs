use std::num::ParseFloatError;

use makepad_widgets::Align;
use proc_macro2::TokenTree;

use crate::prop::ALIGN;

use super::normal_prop;

pub fn align(value: &str) -> Vec<TokenTree> {
    fn handle(value: &str) -> Align {
        match value
            .split(' ')
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(aligns) => match aligns.len() {
                1 => Align {
                    x: aligns[0],
                    y: aligns[0],
                },
                2 => Align {
                    x: aligns[0],
                    y: aligns[1],
                },
                _ => panic!( "{} cannot be converted to Makepad::Align!",value)
            },
            Err(_) => panic!( "{} cannot be converted to Makepad::Align!",value),
        }
    }

    normal_prop(ALIGN, &format!("{:?}",handle(value)))
}
