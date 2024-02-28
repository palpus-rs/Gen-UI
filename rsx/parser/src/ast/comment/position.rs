#[derive(Debug,Clone,PartialEq)]
pub enum OfflinePosition{
    AboveTemplate,
    AboveScript,
    AboveStyle,
    BelowTemplate,
    BelowScript,
    BelowStyle,
    /// no template no style no rust script, only has comment
    Flexible
}