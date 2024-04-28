#[allow(unused)]
use std::{default, fmt::Display};

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::{
    prop::{
        ARROW, COL_RESIZE, CROSSHAIR, DEFAULT, EW_RESIZE, E_RESIZE, HAND, HELP, HIDDEN, MOVE,
        NESW_RESIZE, NE_RESIZE, NOT_ALLOWED, NS_RESIZE, NWSE_RESIZE, NW_RESIZE, N_RESIZE,
        ROW_RESIZE, SE_RESIZE, SW_RESIZE, S_RESIZE, TEXT, WAIT, W_RESIZE,
    },
    str_to_string_try_from,
};

#[derive(Debug, Clone, PartialEq,Default)]
pub enum MouseCursor {
    Hidden,
    /// default
    #[default]
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

impl TryFrom<&str> for MouseCursor {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            HIDDEN => Ok(MouseCursor::Hidden),
            DEFAULT => Ok(MouseCursor::Default),
            CROSSHAIR => Ok(MouseCursor::Crosshair),
            HAND => Ok(MouseCursor::Hand),
            ARROW => Ok(MouseCursor::Arrow),
            MOVE => Ok(MouseCursor::Move),
            TEXT => Ok(MouseCursor::Text),
            WAIT => Ok(MouseCursor::Wait),
            HELP => Ok(MouseCursor::Help),
            NOT_ALLOWED => Ok(MouseCursor::NotAllowed),
            N_RESIZE => Ok(MouseCursor::NResize),
            NE_RESIZE => Ok(MouseCursor::NeResize),
            E_RESIZE => Ok(MouseCursor::EResize),
            SE_RESIZE => Ok(MouseCursor::SeResize),
            S_RESIZE => Ok(MouseCursor::SResize),
            SW_RESIZE => Ok(MouseCursor::SwResize),
            W_RESIZE => Ok(MouseCursor::WResize),
            NW_RESIZE => Ok(MouseCursor::NwResize),
            NS_RESIZE => Ok(MouseCursor::NsResize),
            NESW_RESIZE => Ok(MouseCursor::NeswResize),
            EW_RESIZE => Ok(MouseCursor::EwResize),
            NWSE_RESIZE => Ok(MouseCursor::NwseResize),
            COL_RESIZE => Ok(MouseCursor::ColResize),
            ROW_RESIZE => Ok(MouseCursor::RowResize),
            _ => Err(Errors::PropConvertFail(format!(
                "MouseCursor: {} is not supported",
                value
            ))),
        }
    }
}

str_to_string_try_from! {MouseCursor}

impl TryFrom<&Value> for MouseCursor {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else {
            value
                .is_string_and_get()
                .map(|s| s.try_into())
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{:?} can not convert to MouseCursor",
                        value
                    )))
                })
        }
    }
}

impl Display for MouseCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MouseCursor::Default => DEFAULT,
            MouseCursor::Hidden => HIDDEN,
            MouseCursor::Crosshair => CROSSHAIR,
            MouseCursor::Hand => HAND,
            MouseCursor::Arrow => ARROW,
            MouseCursor::Move => MOVE,
            MouseCursor::Text => TEXT,
            MouseCursor::Wait => WAIT,
            MouseCursor::Help => HELP,
            MouseCursor::NotAllowed => NOT_ALLOWED,
            MouseCursor::NResize => N_RESIZE,
            MouseCursor::NeResize => NE_RESIZE,
            MouseCursor::EResize => E_RESIZE,
            MouseCursor::SeResize => SE_RESIZE,
            MouseCursor::SResize => S_RESIZE,
            MouseCursor::SwResize => SW_RESIZE,
            MouseCursor::WResize => W_RESIZE,
            MouseCursor::NwResize => NW_RESIZE,
            MouseCursor::NsResize => NS_RESIZE,
            MouseCursor::NeswResize => NESW_RESIZE,
            MouseCursor::EwResize => EW_RESIZE,
            MouseCursor::NwseResize => NWSE_RESIZE,
            MouseCursor::ColResize => COL_RESIZE,
            MouseCursor::RowResize => ROW_RESIZE,
        })
    }
}
