use super::{position::OfflinePosition, Comments};

/// # Offline Comment
/// ## Display
/// ```rsx
/// // this is offline comment
/// <template>
///  // this is inline comment
/// </template>
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OfflineComment<'a> {
    value: Vec<Comments<'a>>,
    position: OfflinePosition,
}

impl<'a> OfflineComment<'a> {
    pub fn value(&self) -> &Vec<Comments> {
        &self.value
    }
    pub fn position(&self) -> OfflinePosition {
        self.position.clone()
    }
    
}

impl<'a> From<(Vec<Comments<'a>>, OfflinePosition)> for OfflineComment<'a> {
    fn from(value: (Vec<Comments<'a>>, OfflinePosition)) -> Self {
        OfflineComment {
            value: value.0,
            position: value.1,
        }
    }
}

impl<'a> ToString for OfflineComment<'a> {
    fn to_string(&self) -> String {
        self.value()
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
