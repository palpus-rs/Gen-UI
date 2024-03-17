use std::fmt::Display;

use syn::parse::Parse;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{
        ARROW, COL_RESIZE, CROSSHAIR, DEFAULT, EW_RESIZE, E_RESIZE, HAND, HELP, HIDDEN, MOVE,
        NESW_RESIZE, NE_RESIZE, NOT_ALLOWED, NS_RESIZE, NWSE_RESIZE, NW_RESIZE, N_RESIZE,
        ROW_RESIZE, SE_RESIZE, SW_RESIZE, S_RESIZE, TEXT, WAIT, W_RESIZE,
    },
};

use super::MapValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Cursor {
    Hidden,
    /// default
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    NotAllowed,
    NResize,
    NeResize,
    EResize,
    SeResize,
    SResize,
    SwResize,
    WResize,
    NwResize,
    NsResize,
    NeswResize,
    EwResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl MapValue for Cursor {
    fn map_value_code(&self) -> String {
        match self {
            Cursor::Hidden => "Cursor::Hidden".to_string(),
            Cursor::Default => "Cursor::Default".to_string(),
            Cursor::Crosshair => "Cursor::Crosshair".to_string(),
            Cursor::Hand => "Cursor::Hand".to_string(),
            Cursor::Arrow => "Cursor::Arrow".to_string(),
            Cursor::Move => "Cursor::Move".to_string(),
            Cursor::Text => "Cursor::Text".to_string(),
            Cursor::Wait => "Cursor::Wait".to_string(),
            Cursor::Help => "Cursor::Help".to_string(),
            Cursor::NotAllowed => "Cursor::NotAllowed".to_string(),
            Cursor::NResize => "Cursor::NResize".to_string(),
            Cursor::NeResize => "Cursor::NeResize".to_string(),
            Cursor::EResize => "Cursor::EResize".to_string(),
            Cursor::SeResize => "Cursor::SeResize".to_string(),
            Cursor::SResize => "Cursor::SResize".to_string(),
            Cursor::SwResize => "Cursor::SwResize".to_string(),
            Cursor::WResize => "Cursor::WResize".to_string(),
            Cursor::NwResize => "Cursor::NwResize".to_string(),
            Cursor::NsResize => "Cursor::NsResize".to_string(),
            Cursor::NeswResize => "Cursor::NeswResize".to_string(),
            Cursor::EwResize => "Cursor::EwResize".to_string(),
            Cursor::NwseResize => "Cursor::NwseResize".to_string(),
            Cursor::ColResize => "Cursor::ColResize".to_string(),
            Cursor::RowResize => "Cursor::RowResize".to_string(),
        }
    }
}

impl TryFrom<&str> for Cursor {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            HIDDEN => Ok(Cursor::Hidden),
            DEFAULT => Ok(Cursor::Default),
            CROSSHAIR => Ok(Cursor::Crosshair),
            HAND => Ok(Cursor::Hand),
            ARROW => Ok(Cursor::Arrow),
            MOVE => Ok(Cursor::Move),
            TEXT => Ok(Cursor::Text),
            WAIT => Ok(Cursor::Wait),
            HELP => Ok(Cursor::Help),
            NOT_ALLOWED => Ok(Cursor::NotAllowed),
            N_RESIZE => Ok(Cursor::NResize),
            NE_RESIZE => Ok(Cursor::NeResize),
            E_RESIZE => Ok(Cursor::EResize),
            SE_RESIZE => Ok(Cursor::SeResize),
            S_RESIZE => Ok(Cursor::SResize),
            SW_RESIZE => Ok(Cursor::SwResize),
            W_RESIZE => Ok(Cursor::WResize),
            NW_RESIZE => Ok(Cursor::NwResize),
            NS_RESIZE => Ok(Cursor::NsResize),
            NESW_RESIZE => Ok(Cursor::NeswResize),
            EW_RESIZE => Ok(Cursor::EwResize),
            NWSE_RESIZE => Ok(Cursor::NwseResize),
            COL_RESIZE => Ok(Cursor::ColResize),
            ROW_RESIZE => Ok(Cursor::RowResize),
            _ => Err(Errors::PropConvertFail(format!(
                "Cursor: {} is not supported",
                value
            ))),
        }
    }
}

str_to_string_try_from! {Cursor}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Cursor::Default => DEFAULT,
            Cursor::Hidden => HIDDEN,
            Cursor::Crosshair => CROSSHAIR,
            Cursor::Hand => HAND,
            Cursor::Arrow => ARROW,
            Cursor::Move => MOVE,
            Cursor::Text => TEXT,
            Cursor::Wait => WAIT,
            Cursor::Help => HELP,
            Cursor::NotAllowed => NOT_ALLOWED,
            Cursor::NResize => N_RESIZE,
            Cursor::NeResize => NE_RESIZE,
            Cursor::EResize => E_RESIZE,
            Cursor::SeResize => SE_RESIZE,
            Cursor::SResize => S_RESIZE,
            Cursor::SwResize => SW_RESIZE,
            Cursor::WResize => W_RESIZE,
            Cursor::NwResize => NW_RESIZE,
            Cursor::NsResize => NS_RESIZE,
            Cursor::NeswResize => NESW_RESIZE,
            Cursor::EwResize => EW_RESIZE,
            Cursor::NwseResize => NWSE_RESIZE,
            Cursor::ColResize => COL_RESIZE,
            Cursor::RowResize => ROW_RESIZE,
        })
    }
}

impl Parse for Cursor {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let ident_str = ident.to_string();
        match ident_str.as_str().try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("{} cannot be converted to Makepad::Cursor!", ident_str),
            )),
        }
    }
}
