use std::path::Display;

use super::{button::ButtonAction, Widgets};

pub enum MakepadWidgetActions {
    Button(ButtonAction),
}

impl MakepadWidgetActions {
    pub fn match_action(tag: Widgets, action: &str) -> MakepadWidgetActions {
        match tag {
            Widgets::Button => ButtonAction::match_action(action).into(),
            _ => panic!("Invalid Makepad Widget Actions"),
        }
    }
}

impl ToString for MakepadWidgetActions {
    fn to_string(&self) -> String {
        match self {
            MakepadWidgetActions::Button(b) => b.to_string(),
        }
    }
}

impl From<ButtonAction> for MakepadWidgetActions {
    fn from(value: ButtonAction) -> Self {
        MakepadWidgetActions::Button(value)
    }
}
