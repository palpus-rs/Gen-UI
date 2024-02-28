use super::position::OfflinePosition;

/// # Offline Comment
/// ## Display
/// ```rsx
/// // this is offline comment
/// <template>
///  // this is inline comment
/// </template>
/// ``` 
#[derive(Debug,Clone,PartialEq)]
pub struct OfflineComment {
    value: String,
    position: OfflinePosition
}

impl OfflineComment {
    pub fn value(&self)->&str{
        &self.value
    }
}

impl ToString for OfflineComment {
    fn to_string(&self) -> String {
        self.value().to_string()
    }
}